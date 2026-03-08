use petgraph::Direction;
use petgraph::graph::{DiGraph, NodeIndex};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use thiserror::Error;

use super::node::DirNode;

#[derive(Debug, Error)]
pub enum FilesystemError {
    #[error("Directory not found: {0}")]
    NotFound(String),
    #[error("Cannot navigate above root")]
    AboveRoot,
}

/// Edge types in the filesystem graph
#[derive(Debug, Clone)]
pub enum EdgeType {
    /// Normal parent-child relationship
    Child,
    /// Paradoxical link (child contains parent, loops, etc.)
    Paradox,
}

/// The impossible filesystem - a directed graph with cycles
pub struct FilesystemGraph {
    graph: DiGraph<DirNode, EdgeType>,
    #[allow(dead_code)] // May be used for future traversal operations
    root: NodeIndex,
    current: NodeIndex,
    /// Path history for .. navigation (can diverge from graph structure)
    path_stack: Vec<NodeIndex>,
}

impl FilesystemGraph {
    #[must_use]
    pub fn new() -> Self {
        let mut graph = DiGraph::new();
        let root = graph.add_node(DirNode::new("/", 0));
        Self {
            graph,
            root,
            current: root,
            path_stack: vec![root],
        }
    }

    #[must_use]
    pub fn current_path(&self) -> String {
        if self.path_stack.len() <= 1 {
            return "/".to_string();
        }

        // Calculate exact required capacity to avoid reallocations
        let capacity: usize = self.path_stack[1..]
            .iter()
            .map(|&idx| 1 + self.graph[idx].name().len()) // 1 for '/' + length of name
            .sum();

        let mut path = String::with_capacity(capacity);
        for &idx in &self.path_stack[1..] {
            path.push('/');
            path.push_str(self.graph[idx].name());
        }
        path
    }

    #[must_use]
    pub fn current_dir_name(&self) -> &str {
        self.graph[self.current].name()
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn current_depth(&self) -> u32 {
        // Depth is based on path stack length, not node depth
        // This allows paradox navigation to increase depth infinitely
        (self.path_stack.len() - 1) as u32
    }

    pub fn add_child(&mut self, name: &str) -> NodeIndex {
        let depth = self.current_depth() + 1;
        let child = self.graph.add_node(DirNode::new(name, depth));
        self.graph.add_edge(self.current, child, EdgeType::Child);
        child
    }

    #[must_use]
    pub fn list_directories(&self) -> Vec<String> {
        self.graph
            .neighbors_directed(self.current, Direction::Outgoing)
            .filter(|&idx| !self.graph[idx].is_hidden())
            .map(|idx| self.graph[idx].name().to_string())
            .collect()
    }

    /// Add a hidden child directory (invisible until fsck reveals it)
    pub fn add_hidden_child(&mut self, name: &str) -> NodeIndex {
        let depth = self.current_depth() + 1;
        let child = self.graph.add_node(DirNode::new_hidden(name, depth));
        self.graph.add_edge(self.current, child, EdgeType::Child);
        child
    }

    /// Reveal all hidden files and directories in the current node.
    /// Returns names of everything that was revealed.
    pub fn reveal_hidden_in_current(&mut self) -> Vec<String> {
        // Reveal hidden files in current directory
        let mut revealed = self.graph[self.current].reveal_hidden_files();

        // Reveal hidden child directories
        let hidden_children: Vec<NodeIndex> = self
            .graph
            .neighbors_directed(self.current, Direction::Outgoing)
            .filter(|&idx| self.graph[idx].is_hidden())
            .collect();

        for idx in hidden_children {
            self.graph[idx].reveal();
            revealed.push(self.graph[idx].name().to_string());
        }

        revealed
    }

    /// Changes directory.
    /// # Panics
    /// Panics if path stack is empty.
    /// # Errors
    /// Returns an error if directory not found.
    pub fn change_dir(
        &mut self,
        name: &str,
        seed: u64,
        disorientation_prob: f64,
    ) -> Result<(), FilesystemError> {
        let name_upper = name.to_uppercase();

        if name_upper == ".." {
            if self.path_stack.len() <= 1 {
                return Err(FilesystemError::AboveRoot);
            }

            // At Corruption layer+, `cd ..` occasionally returns to wrong parent
            if self.path_stack.len() >= 3 {
                let mut rng = ChaCha8Rng::seed_from_u64(seed);
                if rng.gen_bool(disorientation_prob) {
                    let grandparent = self.path_stack[self.path_stack.len() - 3];
                    let current_parent = self.path_stack[self.path_stack.len() - 2];

                    let siblings: Vec<NodeIndex> = self
                        .graph
                        .neighbors_directed(grandparent, Direction::Outgoing)
                        .filter(|&idx| idx != current_parent && !self.graph[idx].is_hidden())
                        .collect();

                    if !siblings.is_empty() {
                        let chosen_sibling = siblings[rng.gen_range(0..siblings.len())];

                        // Pop the current node
                        self.path_stack.pop();
                        // Pop the true parent
                        self.path_stack.pop();
                        // Push the chosen sibling (the new "parent")
                        self.path_stack.push(chosen_sibling);

                        self.current = chosen_sibling;
                        return Ok(());
                    }
                }
            }

            self.path_stack.pop();
            self.current = *self
                .path_stack
                .last()
                .expect("path_stack should never be empty when popping");
            return Ok(());
        }

        // Find child with matching name
        for neighbor in self
            .graph
            .neighbors_directed(self.current, Direction::Outgoing)
        {
            if self.graph[neighbor].name() == name_upper {
                self.current = neighbor;
                self.path_stack.push(neighbor);
                return Ok(());
            }
        }

        Err(FilesystemError::NotFound(name_upper))
    }

    /// Create a paradox where current directory contains itself
    /// This creates a NEW node that looks like the current one
    pub fn add_paradox_to_self(&mut self) {
        let current_name = self.graph[self.current].name().to_string();
        let new_depth = self.current_depth() + 1;

        // Create a new node with same name but deeper
        let paradox_node = self.graph.add_node(DirNode::new(&current_name, new_depth));

        // Add paradox edge from current to the new node
        self.graph
            .add_edge(self.current, paradox_node, EdgeType::Paradox);

        // The paradox node should also contain itself (infinite regression)
        self.graph
            .add_edge(paradox_node, paradox_node, EdgeType::Paradox);
    }

    /// Create a paradox link between two arbitrary nodes
    pub fn add_paradox_link(&mut self, from: NodeIndex, to: NodeIndex) {
        self.graph.add_edge(from, to, EdgeType::Paradox);
    }

    /// Get mutable reference to current directory node
    pub fn current_node_mut(&mut self) -> &mut DirNode {
        &mut self.graph[self.current]
    }

    /// Get reference to current directory node
    #[must_use]
    pub fn current_node(&self) -> &DirNode {
        &self.graph[self.current]
    }

    /// Get mutable reference to a node by index
    pub fn node_mut(&mut self, idx: NodeIndex) -> &mut DirNode {
        &mut self.graph[idx]
    }
}

impl Default for FilesystemGraph {
    fn default() -> Self {
        Self::new()
    }
}
