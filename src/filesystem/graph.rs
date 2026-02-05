use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::Direction;
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
    root: NodeIndex,
    current: NodeIndex,
    /// Path history for .. navigation (can diverge from graph structure)
    path_stack: Vec<NodeIndex>,
}

impl FilesystemGraph {
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

    pub fn current_path(&self) -> String {
        if self.path_stack.len() <= 1 {
            return "/".to_string();
        }

        let mut path = String::new();
        for &idx in &self.path_stack[1..] {
            path.push('/');
            path.push_str(self.graph[idx].name());
        }
        path
    }

    pub fn current_dir_name(&self) -> &str {
        self.graph[self.current].name()
    }

    pub fn current_depth(&self) -> u32 {
        self.graph[self.current].depth()
    }

    pub fn add_child(&mut self, name: &str) -> NodeIndex {
        let depth = self.current_depth() + 1;
        let child = self.graph.add_node(DirNode::new(name, depth));
        self.graph.add_edge(self.current, child, EdgeType::Child);
        child
    }

    pub fn list_directories(&self) -> Vec<String> {
        self.graph
            .neighbors_directed(self.current, Direction::Outgoing)
            .map(|idx| self.graph[idx].name().to_string())
            .collect()
    }

    pub fn change_dir(&mut self, name: &str) -> Result<(), FilesystemError> {
        let name_upper = name.to_uppercase();

        if name_upper == ".." {
            if self.path_stack.len() <= 1 {
                return Err(FilesystemError::AboveRoot);
            }
            self.path_stack.pop();
            self.current = *self.path_stack.last().unwrap();
            return Ok(());
        }

        // Find child with matching name
        for neighbor in self.graph.neighbors_directed(self.current, Direction::Outgoing) {
            if self.graph[neighbor].name() == name_upper {
                self.current = neighbor;
                self.path_stack.push(neighbor);
                return Ok(());
            }
        }

        Err(FilesystemError::NotFound(name_upper))
    }

    /// Get mutable reference to current directory node
    pub fn current_node_mut(&mut self) -> &mut DirNode {
        &mut self.graph[self.current]
    }

    /// Get reference to current directory node
    pub fn current_node(&self) -> &DirNode {
        &self.graph[self.current]
    }
}

impl Default for FilesystemGraph {
    fn default() -> Self {
        Self::new()
    }
}
