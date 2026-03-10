import re

with open("src/filesystem/graph.rs", "r") as f:
    graph_content = f.read()

imports = "use rand::prelude::*;\nuse rand_chacha::ChaCha8Rng;\nuse petgraph::Direction;\n"
if "ChaCha8Rng" not in graph_content:
    graph_content = re.sub(r"use petgraph::Direction;\n", imports, graph_content)

change_dir_sig = "pub fn change_dir(&mut self, name: &str) -> Result<(), FilesystemError> {"
new_change_dir = """/// Changes directory.
    /// # Panics
    /// Panics if path stack is empty.
    /// # Errors
    /// Returns an error if directory not found.
    pub fn change_dir(&mut self, name: &str) -> Result<(), FilesystemError> {
        self.change_dir_internal(name, None)
    }

    /// Changes directory with a seed for potential horror effects.
    /// # Panics
    /// Panics if path stack is empty.
    /// # Errors
    /// Returns an error if directory not found or if trying to navigate above root.
    pub fn change_dir_seeded(&mut self, name: &str, seed: u64) -> Result<(), FilesystemError> {
        self.change_dir_internal(name, Some(seed))
    }

    fn change_dir_internal(&mut self, name: &str, seed: Option<u64>) -> Result<(), FilesystemError> {"""

if "change_dir_internal" not in graph_content:
    graph_content = re.sub(r"    /// Changes directory\.\n    /// # Panics\n    /// Panics if path stack is empty\.\n    /// # Errors\n    /// Returns an error if directory not found\.\n    pub fn change_dir\(&mut self, name: &str\) -> Result<\(\), FilesystemError> {", new_change_dir, graph_content)

dotdot_block = """if name_upper == ".." {
            if self.path_stack.len() <= 1 {
                return Err(FilesystemError::AboveRoot);
            }
            self.path_stack.pop();
            self.current = *self.path_stack.last().unwrap();
            return Ok(());
        }"""

new_dotdot_block = """if name_upper == ".." {
            if self.path_stack.len() <= 1 {
                return Err(FilesystemError::AboveRoot);
            }

            // Disorienting navigation at depth >= 6 (Corruption layer+)
            let depth = self.current_depth();
            #[allow(clippy::collapsible_if)]
            if depth >= 6 {
                if let Some(s) = seed {
                    let mut rng = ChaCha8Rng::seed_from_u64(s);
                    // Probability increases with depth (e.g. 5% at depth 6, 20% at depth 36)
                    let probability = f64::from(depth - 6).mul_add(0.005, 0.05).min(0.20);

                    if rng.gen_bool(probability) {
                        // Go sideways: pop current node but push a random other node instead of real parent
                        self.path_stack.pop();

                        // Collect all directory nodes except root
                        let all_nodes: Vec<NodeIndex> = self.graph.node_indices()
                            .filter(|&idx| idx != self.root && !self.graph[idx].is_hidden())
                            .collect();

                        if !all_nodes.is_empty() {
                            // Pick a random node
                            let wrong_parent = *all_nodes.choose(&mut rng).unwrap();

                            // If we aren't at root, pop the actual parent so we can insert the wrong one
                            if self.path_stack.len() > 1 {
                                self.path_stack.pop();
                            }

                            // Push the wrong parent in both cases
                            self.path_stack.push(wrong_parent);
                            self.current = wrong_parent;
                            return Ok(());
                        }
                    }
                }
            }

            // Normal navigation
            self.path_stack.pop();
            self.current = *self.path_stack.last().unwrap();
            return Ok(());
        }"""

if "Disorienting navigation" not in graph_content:
    graph_content = graph_content.replace(dotdot_block, new_dotdot_block)

with open("src/filesystem/graph.rs", "w") as f:
    f.write(graph_content)

with open("src/commands/executor.rs", "r") as f:
    executor_content = f.read()

executor_cd = """fn change_dir(&mut self, path: &str) -> CommandResult {
        if path.is_empty() {
            return CommandResult::error("?SYNTAX ERROR\\n");
        }

        match self.fs.change_dir(path) {"""

new_executor_cd = """fn change_dir(&mut self, path: &str) -> CommandResult {
        if path.is_empty() {
            return CommandResult::error("?SYNTAX ERROR\\n");
        }

        let seed = 0xF5C0_0000u64.wrapping_add(u64::from(self.entity.interaction_count()));

        match self.fs.change_dir_seeded(path, seed) {"""

if "change_dir_seeded" not in executor_content:
    executor_content = executor_content.replace(executor_cd, new_executor_cd)

with open("src/commands/executor.rs", "w") as f:
    f.write(executor_content)

with open("tests/filesystem_tests.rs", "r") as f:
    tests_content = f.read()

new_test_block = """#[test]
fn test_disorienting_navigation() {
    let mut fs = FilesystemGraph::new();

    // Build a deeper path directly to bypass slow generator
    fs.add_child("A");
    fs.change_dir("A").unwrap();
    fs.add_child("B");
    fs.change_dir("B").unwrap();
    fs.add_child("C");
    fs.change_dir("C").unwrap();
    fs.add_child("D");
    fs.change_dir("D").unwrap();
    fs.add_child("E");
    fs.change_dir("E").unwrap();
    fs.add_child("F");
    fs.change_dir("F").unwrap(); // Depth 6

    // Add another node so we have something to disorient to
    fs.change_dir("..").unwrap();
    fs.add_child("SIBLING");
    fs.change_dir("F").unwrap();

    let mut triggered = false;
    for seed in 0..100 {
        // Recreate path stack artificially since cd .. pops it
        let mut test_fs = fs.clone();

        test_fs.change_dir_seeded("..", seed).unwrap();

        let curr = test_fs.current_dir_name().to_string();
        if curr != "E" {
            triggered = true;
            break;
        }
    }

    assert!(triggered, "Should have triggered disorienting navigation at least once with 100 seeds");
}"""

# Actually FilesystemGraph does not implement clone in tests!
# We'll just push and pop in the loop.

new_test_block = """#[test]
fn test_disorienting_navigation() {
    let mut fs = FilesystemGraph::new();

    // Build a deeper path directly
    fs.add_child("A");
    fs.change_dir("A").unwrap();
    fs.add_child("B");
    fs.change_dir("B").unwrap();
    fs.add_child("C");
    fs.change_dir("C").unwrap();
    fs.add_child("D");
    fs.change_dir("D").unwrap();
    fs.add_child("E");
    fs.change_dir("E").unwrap();

    // Add sibling to E
    fs.change_dir("..").unwrap();
    fs.add_child("SIBLING");
    fs.change_dir("E").unwrap();

    let mut triggered = false;
    for seed in 0..100 {
        // Create F dynamically so we can pop it
        fs.add_child("F");
        fs.change_dir("F").unwrap(); // Depth 6

        fs.change_dir_seeded("..", seed).unwrap();

        let curr = fs.current_dir_name().to_string();
        if curr != "E" {
            triggered = true;
            break;
        }

        // If it was normal, we are at E.
    }

    assert!(triggered, "Should have triggered disorienting navigation at least once with 100 seeds");
}"""

if "test_disorienting_navigation" not in tests_content:
    tests_content += "\n" + new_test_block + "\n"

with open("tests/filesystem_tests.rs", "w") as f:
    f.write(tests_content)
