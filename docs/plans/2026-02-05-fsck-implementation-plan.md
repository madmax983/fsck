# fsck Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a terminal horror game where players explore an impossible filesystem inside a sentient Apple IIe.

**Architecture:** Rust core compiled to WASM, xterm.js for terminal rendering in browser. The game engine handles command parsing, filesystem graph traversal, entity state, and persistence. All game logic in Rust; only terminal I/O in JavaScript.

**Tech Stack:** Rust (edition 2024), wasm-bindgen, web-sys, xterm.js, petgraph (filesystem graph), serde (serialization), getrandom (WASM-compatible RNG)

---

## Phase 1: WASM Terminal Foundation

Establish the build pipeline and prove we can render a terminal in browser with Rust handling input.

### Task 1.1: Project Setup and WASM Toolchain

**Files:**
- Modify: `Cargo.toml`
- Create: `rust-toolchain.toml`
- Create: `.cargo/config.toml`

**Step 1: Add WASM target**

Run: `rustup target add wasm32-unknown-unknown`
Expected: Target installed successfully

**Step 2: Install wasm-pack**

Run: `cargo install wasm-pack`
Expected: wasm-pack installed

**Step 3: Create rust-toolchain.toml**

```toml
[toolchain]
channel = "stable"
targets = ["wasm32-unknown-unknown"]
```

**Step 4: Create .cargo/config.toml**

```toml
[build]
target = "wasm32-unknown-unknown"

[target.wasm32-unknown-unknown]
rustflags = ["-C", "opt-level=z"]
```

**Step 5: Update Cargo.toml with dependencies and lib config**

```toml
[package]
name = "fsck"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = [
    "console",
    "Window",
    "Document",
    "Storage",
] }
js-sys = "0.3"
getrandom = { version = "0.2", features = ["js"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "2.0"

[dev-dependencies]
wasm-bindgen-test = "0.3"

[profile.release]
opt-level = "z"
lto = true
```

**Step 6: Commit**

```bash
git add Cargo.toml rust-toolchain.toml .cargo/config.toml
git commit -m "chore: configure WASM build toolchain"
```

---

### Task 1.2: Basic WASM Entry Point

**Files:**
- Modify: `src/main.rs` → rename to `src/lib.rs`
- Create: `tests/wasm.rs`

**Step 1: Write the test for WASM initialization**

Create `tests/wasm.rs`:

```rust
#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_game_initializes() {
    let game = fsck::Game::new();
    assert!(game.is_ready());
}
```

**Step 2: Run test to verify it fails**

Run: `wasm-pack test --headless --chrome`
Expected: FAIL - `Game` type doesn't exist

**Step 3: Create lib.rs with Game struct**

Delete `src/main.rs`, create `src/lib.rs`:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
    initialized: bool,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { initialized: true }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
```

**Step 4: Run test to verify it passes**

Run: `wasm-pack test --headless --chrome`
Expected: PASS

**Step 5: Commit**

```bash
git add src/lib.rs tests/wasm.rs
git rm src/main.rs
git commit -m "feat: add WASM game entry point"
```

---

### Task 1.3: Terminal Output Interface

**Files:**
- Create: `src/terminal/mod.rs`
- Create: `src/terminal/output.rs`
- Modify: `src/lib.rs`
- Create: `tests/terminal_tests.rs`

**Step 1: Write test for terminal output buffer**

Create `tests/terminal_tests.rs`:

```rust
use fsck::terminal::OutputBuffer;

#[test]
fn test_output_buffer_captures_text() {
    let mut buffer = OutputBuffer::new();
    buffer.write("HELLO");
    assert_eq!(buffer.drain(), "HELLO");
}

#[test]
fn test_output_buffer_accumulates() {
    let mut buffer = OutputBuffer::new();
    buffer.write("LINE 1\n");
    buffer.write("LINE 2");
    assert_eq!(buffer.drain(), "LINE 1\nLINE 2");
}

#[test]
fn test_drain_clears_buffer() {
    let mut buffer = OutputBuffer::new();
    buffer.write("TEXT");
    let _ = buffer.drain();
    assert_eq!(buffer.drain(), "");
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test terminal`
Expected: FAIL - module `terminal` doesn't exist

**Step 3: Create terminal module structure**

Create `src/terminal/mod.rs`:

```rust
mod output;

pub use output::OutputBuffer;
```

Create `src/terminal/output.rs`:

```rust
/// Buffer for accumulating terminal output before sending to JS
#[derive(Default)]
pub struct OutputBuffer {
    content: String,
}

impl OutputBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn write(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn drain(&mut self) -> String {
        std::mem::take(&mut self.content)
    }
}
```

**Step 4: Update lib.rs to expose terminal module**

Add to `src/lib.rs`:

```rust
pub mod terminal;
```

**Step 5: Run tests to verify they pass**

Run: `cargo test terminal`
Expected: All 3 tests PASS

**Step 6: Commit**

```bash
git add src/terminal/ tests/terminal_tests.rs src/lib.rs
git commit -m "feat: add terminal output buffer"
```

---

### Task 1.4: Command Input Processing

**Files:**
- Create: `src/terminal/input.rs`
- Modify: `src/terminal/mod.rs`
- Modify: `tests/terminal_tests.rs`

**Step 1: Write test for input parsing**

Add to `tests/terminal_tests.rs`:

```rust
use fsck::terminal::InputParser;

#[test]
fn test_parse_simple_command() {
    let result = InputParser::parse("CATALOG");
    assert_eq!(result.command, "CATALOG");
    assert!(result.args.is_empty());
}

#[test]
fn test_parse_command_with_args() {
    let result = InputParser::parse("CD GAMES");
    assert_eq!(result.command, "CD");
    assert_eq!(result.args, vec!["GAMES"]);
}

#[test]
fn test_parse_normalizes_to_uppercase() {
    let result = InputParser::parse("catalog");
    assert_eq!(result.command, "CATALOG");
}

#[test]
fn test_parse_empty_input() {
    let result = InputParser::parse("");
    assert_eq!(result.command, "");
    assert!(result.args.is_empty());
}

#[test]
fn test_parse_trims_whitespace() {
    let result = InputParser::parse("  CD   GAMES  ");
    assert_eq!(result.command, "CD");
    assert_eq!(result.args, vec!["GAMES"]);
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test input`
Expected: FAIL - `InputParser` doesn't exist

**Step 3: Implement InputParser**

Create `src/terminal/input.rs`:

```rust
/// Parsed command from user input
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedInput {
    pub command: String,
    pub args: Vec<String>,
}

/// Parser for Apple IIe style command input
pub struct InputParser;

impl InputParser {
    pub fn parse(input: &str) -> ParsedInput {
        let normalized = input.trim().to_uppercase();
        let mut parts = normalized.split_whitespace();

        let command = parts.next().unwrap_or("").to_string();
        let args: Vec<String> = parts.map(String::from).collect();

        ParsedInput { command, args }
    }
}
```

**Step 4: Update terminal/mod.rs**

```rust
mod input;
mod output;

pub use input::{InputParser, ParsedInput};
pub use output::OutputBuffer;
```

**Step 5: Run tests to verify they pass**

Run: `cargo test input`
Expected: All 5 tests PASS

**Step 6: Commit**

```bash
git add src/terminal/input.rs src/terminal/mod.rs tests/terminal_tests.rs
git commit -m "feat: add command input parser"
```

---

### Task 1.5: Game Loop Integration

**Files:**
- Modify: `src/lib.rs`
- Modify: `tests/wasm.rs`

**Step 1: Write test for game processing input**

Add to `tests/wasm.rs`:

```rust
#[wasm_bindgen_test]
fn test_game_processes_input() {
    let mut game = fsck::Game::new();
    let output = game.process_input("CATALOG");
    assert!(!output.is_empty());
}

#[wasm_bindgen_test]
fn test_game_shows_prompt() {
    let mut game = fsck::Game::new();
    let output = game.get_prompt();
    assert_eq!(output, "]");
}
```

**Step 2: Run tests to verify they fail**

Run: `wasm-pack test --headless --chrome`
Expected: FAIL - `process_input` method doesn't exist

**Step 3: Add process_input to Game**

Update `src/lib.rs`:

```rust
use wasm_bindgen::prelude::*;

pub mod terminal;

use terminal::{InputParser, OutputBuffer};

#[wasm_bindgen]
pub struct Game {
    initialized: bool,
    output: OutputBuffer,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            initialized: true,
            output: OutputBuffer::new(),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn get_prompt(&self) -> String {
        "]".to_string()
    }

    pub fn process_input(&mut self, input: &str) -> String {
        let parsed = InputParser::parse(input);

        // Temporary: echo back the command
        self.output.write(&format!("?SYNTAX ERROR: {}\n", parsed.command));

        self.output.drain()
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
```

**Step 4: Run tests to verify they pass**

Run: `wasm-pack test --headless --chrome`
Expected: PASS

**Step 5: Commit**

```bash
git add src/lib.rs tests/wasm.rs
git commit -m "feat: add game input processing loop"
```

---

### Task 1.6: Web Frontend with xterm.js

**Files:**
- Create: `web/index.html`
- Create: `web/main.js`
- Create: `web/style.css`

**Step 1: Create index.html**

Create `web/index.html`:

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>fsck</title>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/xterm@5.3.0/css/xterm.css">
    <link rel="stylesheet" href="style.css">
</head>
<body>
    <div id="terminal"></div>
    <script src="https://cdn.jsdelivr.net/npm/xterm@5.3.0/lib/xterm.min.js"></script>
    <script type="module" src="main.js"></script>
</body>
</html>
```

**Step 2: Create style.css**

Create `web/style.css`:

```css
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    background: #000;
    height: 100vh;
    display: flex;
    justify-content: center;
    align-items: center;
}

#terminal {
    width: 800px;
    height: 600px;
}

/* Apple IIe phosphor green */
.xterm {
    --xterm-foreground: #33ff33;
    --xterm-background: #000000;
}
```

**Step 3: Create main.js**

Create `web/main.js`:

```javascript
import init, { Game } from '../pkg/fsck.js';

async function main() {
    // Initialize WASM
    await init();

    // Create game instance
    const game = new Game();

    // Initialize xterm.js with Apple IIe aesthetics
    const term = new Terminal({
        fontFamily: '"Apple II", "Courier New", monospace',
        fontSize: 16,
        theme: {
            foreground: '#33ff33',
            background: '#000000',
            cursor: '#33ff33',
        },
        cursorBlink: true,
        cursorStyle: 'block',
        scrollback: 1000,
    });

    term.open(document.getElementById('terminal'));

    // Boot sequence
    term.writeln('APPLE ][');
    term.writeln('');
    term.write(game.get_prompt());

    // Input buffer
    let inputBuffer = '';

    // Handle keyboard input
    term.onKey(({ key, domEvent }) => {
        const ev = domEvent;

        if (ev.key === 'Enter') {
            term.writeln('');
            const output = game.process_input(inputBuffer);
            if (output) {
                term.write(output);
            }
            term.write(game.get_prompt());
            inputBuffer = '';
        } else if (ev.key === 'Backspace') {
            if (inputBuffer.length > 0) {
                inputBuffer = inputBuffer.slice(0, -1);
                term.write('\b \b');
            }
        } else if (key.length === 1 && !ev.ctrlKey && !ev.altKey) {
            inputBuffer += key;
            term.write(key.toUpperCase());
        }
    });
}

main();
```

**Step 4: Build WASM package**

Run: `wasm-pack build --target web`
Expected: Package built in `pkg/` directory

**Step 5: Commit**

```bash
git add web/
git commit -m "feat: add xterm.js web frontend"
```

---

### Task 1.7: Local Development Server

**Files:**
- Create: `serve.py` (or use any static server)
- Update: `.gitignore`

**Step 1: Update .gitignore**

```
/target
/pkg
```

**Step 2: Create simple Python server script**

Create `serve.py`:

```python
#!/usr/bin/env python3
import http.server
import socketserver

PORT = 8080

class Handler(http.server.SimpleHTTPRequestHandler):
    extensions_map = {
        '': 'application/octet-stream',
        '.html': 'text/html',
        '.js': 'application/javascript',
        '.mjs': 'application/javascript',
        '.css': 'text/css',
        '.wasm': 'application/wasm',
    }

print(f"Serving at http://localhost:{PORT}")
print("Press Ctrl+C to stop")

with socketserver.TCPServer(("", PORT), Handler) as httpd:
    httpd.serve_forever()
```

**Step 3: Test the full pipeline**

Run:
```bash
wasm-pack build --target web
python serve.py
```
Then open http://localhost:8080/web/

Expected: Green terminal with `APPLE ][` header and `]` prompt. Typing shows uppercase characters.

**Step 4: Commit**

```bash
git add serve.py .gitignore
git commit -m "chore: add development server"
```

---

## Phase 2: Impossible Filesystem Engine

Build the graph-based filesystem that can contain paradoxes.

### Task 2.1: Add petgraph Dependency

**Files:**
- Modify: `Cargo.toml`

**Step 1: Add petgraph**

Add to `[dependencies]` in `Cargo.toml`:

```toml
petgraph = "0.6"
```

**Step 2: Verify it compiles**

Run: `cargo check`
Expected: Compiles successfully

**Step 3: Commit**

```bash
git add Cargo.toml
git commit -m "chore: add petgraph for filesystem graph"
```

---

### Task 2.2: Directory Node Structure

**Files:**
- Create: `src/filesystem/mod.rs`
- Create: `src/filesystem/node.rs`
- Create: `tests/filesystem_tests.rs`
- Modify: `src/lib.rs`

**Step 1: Write tests for directory node**

Create `tests/filesystem_tests.rs`:

```rust
use fsck::filesystem::{DirNode, FileNode, NodeContent};

#[test]
fn test_create_directory_node() {
    let node = DirNode::new("GAMES", 0);
    assert_eq!(node.name(), "GAMES");
    assert_eq!(node.depth(), 0);
}

#[test]
fn test_directory_can_have_files() {
    let mut node = DirNode::new("DOCS", 1);
    node.add_file(FileNode::new("README.TXT", "Hello"));
    assert_eq!(node.files().len(), 1);
}

#[test]
fn test_file_node_content() {
    let file = FileNode::new("TEST.TXT", "Content here");
    assert_eq!(file.name(), "TEST.TXT");
    assert_eq!(file.content(), "Content here");
}

#[test]
fn test_file_with_dynamic_content() {
    let file = FileNode::with_dynamic("COUNTER.TXT", NodeContent::Counter { base: "HELLO\n", count: 0 });
    // Dynamic content is retrieved via content_dynamic()
    assert!(matches!(file.content_type(), NodeContent::Counter { .. }));
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test filesystem`
Expected: FAIL - module doesn't exist

**Step 3: Create filesystem module**

Create `src/filesystem/mod.rs`:

```rust
mod node;

pub use node::{DirNode, FileNode, NodeContent};
```

Create `src/filesystem/node.rs`:

```rust
use serde::{Deserialize, Serialize};

/// Content types for files - some are static, some dynamic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeContent {
    /// Static text content
    Static(String),
    /// Counter that increases each read
    Counter { base: &'static str, count: u32 },
    /// Content that changes based on game state
    Dynamic { key: String },
}

/// A file within a directory
#[derive(Debug, Clone)]
pub struct FileNode {
    name: String,
    content: NodeContent,
}

impl FileNode {
    pub fn new(name: &str, content: &str) -> Self {
        Self {
            name: name.to_uppercase(),
            content: NodeContent::Static(content.to_string()),
        }
    }

    pub fn with_dynamic(name: &str, content: NodeContent) -> Self {
        Self {
            name: name.to_uppercase(),
            content,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn content(&self) -> &str {
        match &self.content {
            NodeContent::Static(s) => s,
            _ => "[DYNAMIC]",
        }
    }

    pub fn content_type(&self) -> &NodeContent {
        &self.content
    }
}

/// A directory node in the filesystem graph
#[derive(Debug, Clone)]
pub struct DirNode {
    name: String,
    depth: u32,
    files: Vec<FileNode>,
}

impl DirNode {
    pub fn new(name: &str, depth: u32) -> Self {
        Self {
            name: name.to_uppercase(),
            depth,
            files: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn depth(&self) -> u32 {
        self.depth
    }

    pub fn files(&self) -> &[FileNode] {
        &self.files
    }

    pub fn add_file(&mut self, file: FileNode) {
        self.files.push(file);
    }
}
```

**Step 4: Update lib.rs**

Add to `src/lib.rs`:

```rust
pub mod filesystem;
```

**Step 5: Run tests to verify they pass**

Run: `cargo test filesystem`
Expected: All 4 tests PASS

**Step 6: Commit**

```bash
git add src/filesystem/ tests/filesystem_tests.rs src/lib.rs
git commit -m "feat: add directory and file node types"
```

---

### Task 2.3: Filesystem Graph Structure

**Files:**
- Create: `src/filesystem/graph.rs`
- Modify: `src/filesystem/mod.rs`
- Modify: `tests/filesystem_tests.rs`

**Step 1: Write tests for filesystem graph**

Add to `tests/filesystem_tests.rs`:

```rust
use fsck::filesystem::FilesystemGraph;

#[test]
fn test_create_filesystem_with_root() {
    let fs = FilesystemGraph::new();
    assert_eq!(fs.current_path(), "/");
}

#[test]
fn test_add_child_directory() {
    let mut fs = FilesystemGraph::new();
    fs.add_child("GAMES");
    let children = fs.list_directories();
    assert!(children.contains(&"GAMES".to_string()));
}

#[test]
fn test_navigate_to_child() {
    let mut fs = FilesystemGraph::new();
    fs.add_child("GAMES");
    assert!(fs.change_dir("GAMES").is_ok());
    assert_eq!(fs.current_dir_name(), "GAMES");
}

#[test]
fn test_navigate_parent() {
    let mut fs = FilesystemGraph::new();
    fs.add_child("GAMES");
    fs.change_dir("GAMES").unwrap();
    assert!(fs.change_dir("..").is_ok());
    assert_eq!(fs.current_path(), "/");
}

#[test]
fn test_navigate_nonexistent_fails() {
    let mut fs = FilesystemGraph::new();
    assert!(fs.change_dir("NOWHERE").is_err());
}

#[test]
fn test_depth_tracking() {
    let mut fs = FilesystemGraph::new();
    assert_eq!(fs.current_depth(), 0);
    fs.add_child("LEVEL1");
    fs.change_dir("LEVEL1").unwrap();
    assert_eq!(fs.current_depth(), 1);
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test filesystem`
Expected: FAIL - `FilesystemGraph` doesn't exist

**Step 3: Implement FilesystemGraph**

Create `src/filesystem/graph.rs`:

```rust
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
```

**Step 4: Update filesystem/mod.rs**

```rust
mod graph;
mod node;

pub use graph::{FilesystemError, FilesystemGraph};
pub use node::{DirNode, FileNode, NodeContent};
```

**Step 5: Run tests to verify they pass**

Run: `cargo test filesystem`
Expected: All 10 tests PASS

**Step 6: Commit**

```bash
git add src/filesystem/graph.rs src/filesystem/mod.rs tests/filesystem_tests.rs
git commit -m "feat: add filesystem graph with navigation"
```

---

### Task 2.4: Impossible Topology - Self-Containing Directories

**Files:**
- Modify: `src/filesystem/graph.rs`
- Modify: `tests/filesystem_tests.rs`

**Step 1: Write test for paradoxical loops**

Add to `tests/filesystem_tests.rs`:

```rust
#[test]
fn test_directory_can_contain_itself() {
    let mut fs = FilesystemGraph::new();
    fs.add_child("VOID");
    fs.change_dir("VOID").unwrap();

    // Create paradox: VOID contains VOID
    fs.add_paradox_to_self();

    let children = fs.list_directories();
    assert!(children.contains(&"VOID".to_string()));
}

#[test]
fn test_paradox_navigation_increases_depth() {
    let mut fs = FilesystemGraph::new();
    fs.add_child("LOOP");
    fs.change_dir("LOOP").unwrap();
    fs.add_paradox_to_self();

    let initial_depth = fs.current_depth();
    fs.change_dir("LOOP").unwrap();

    // Depth increases even though we're "in the same place"
    assert!(fs.current_depth() > initial_depth);
}

#[test]
fn test_parent_navigation_from_paradox() {
    let mut fs = FilesystemGraph::new();
    fs.add_child("STRANGE");
    fs.change_dir("STRANGE").unwrap();
    fs.add_paradox_to_self();
    fs.change_dir("STRANGE").unwrap(); // Enter the loop

    // Going back should return to the previous STRANGE
    fs.change_dir("..").unwrap();
    assert_eq!(fs.current_dir_name(), "STRANGE");
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test paradox`
Expected: FAIL - `add_paradox_to_self` doesn't exist

**Step 3: Implement paradox creation**

Add to `src/filesystem/graph.rs` in the `impl FilesystemGraph` block:

```rust
    /// Create a paradox where current directory contains itself
    /// This creates a NEW node that looks like the current one
    pub fn add_paradox_to_self(&mut self) {
        let current_name = self.graph[self.current].name().to_string();
        let new_depth = self.current_depth() + 1;

        // Create a new node with same name but deeper
        let paradox_node = self.graph.add_node(DirNode::new(&current_name, new_depth));

        // Add paradox edge from current to the new node
        self.graph.add_edge(self.current, paradox_node, EdgeType::Paradox);

        // The paradox node should also contain itself (infinite regression)
        self.graph.add_edge(paradox_node, paradox_node, EdgeType::Paradox);
    }

    /// Create a paradox link between two arbitrary nodes
    pub fn add_paradox_link(&mut self, from: NodeIndex, to: NodeIndex) {
        self.graph.add_edge(from, to, EdgeType::Paradox);
    }
```

**Step 4: Run tests to verify they pass**

Run: `cargo test paradox`
Expected: All 3 tests PASS

**Step 5: Commit**

```bash
git add src/filesystem/graph.rs tests/filesystem_tests.rs
git commit -m "feat: add impossible self-containing directories"
```

---

### Task 2.5: Filesystem Seeded Generation

**Files:**
- Create: `src/filesystem/generator.rs`
- Modify: `src/filesystem/mod.rs`
- Modify: `tests/filesystem_tests.rs`

**Step 1: Write tests for seeded generation**

Add to `tests/filesystem_tests.rs`:

```rust
use fsck::filesystem::FilesystemGenerator;

#[test]
fn test_same_seed_produces_same_structure() {
    let fs1 = FilesystemGenerator::generate(12345, 5);
    let fs2 = FilesystemGenerator::generate(12345, 5);

    assert_eq!(fs1.list_directories(), fs2.list_directories());
}

#[test]
fn test_different_seeds_produce_different_structures() {
    let fs1 = FilesystemGenerator::generate(12345, 5);
    let fs2 = FilesystemGenerator::generate(54321, 5);

    // Very unlikely to be identical
    assert_ne!(fs1.list_directories(), fs2.list_directories());
}

#[test]
fn test_generation_respects_depth_limit() {
    let fs = FilesystemGenerator::generate(99999, 3);
    // Root should have some children
    assert!(!fs.list_directories().is_empty());
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test generator`
Expected: FAIL - `FilesystemGenerator` doesn't exist

**Step 3: Add rand dependency**

Add to `Cargo.toml` dependencies:

```toml
rand = "0.8"
rand_chacha = "0.3"
```

**Step 4: Implement FilesystemGenerator**

Create `src/filesystem/generator.rs`:

```rust
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use super::graph::FilesystemGraph;
use super::node::FileNode;

/// Apple IIe era directory names
const DIR_NAMES: &[&str] = &[
    "GAMES", "DOCS", "SYSTEM", "BASIC", "DATA", "PROGS", "UTIL", "BACKUP",
    "OLD", "NEW", "TEMP", "WORK", "FILES", "STUFF", "MISC", "ARCHIVE",
    "DONT", "VOID", "EMPTY", "LOST", "FOUND", "ERROR", "NULL", "DARK",
];

/// File names and content templates
const FILE_TEMPLATES: &[(&str, &str)] = &[
    ("README.TXT", "WELCOME TO THE SYSTEM\n"),
    ("NOTES.TXT", "REMEMBER TO BACKUP\n"),
    ("LOG.TXT", "SYSTEM STARTED\n"),
    ("HELLO.BAS", "10 PRINT \"HELLO\"\n20 GOTO 10\n"),
];

pub struct FilesystemGenerator;

impl FilesystemGenerator {
    pub fn generate(seed: u64, initial_depth: u32) -> FilesystemGraph {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut fs = FilesystemGraph::new();

        Self::populate_level(&mut fs, &mut rng, 0, initial_depth);

        fs
    }

    fn populate_level(
        fs: &mut FilesystemGraph,
        rng: &mut ChaCha8Rng,
        current_depth: u32,
        max_depth: u32,
    ) {
        if current_depth >= max_depth {
            return;
        }

        // Add 1-4 directories at this level
        let num_dirs = rng.gen_range(1..=4);
        let mut chosen_names: Vec<&str> = Vec::new();

        for _ in 0..num_dirs {
            let name = DIR_NAMES[rng.gen_range(0..DIR_NAMES.len())];
            if !chosen_names.contains(&name) {
                chosen_names.push(name);
                fs.add_child(name);
            }
        }

        // Add some files to current directory
        let num_files = rng.gen_range(0..=3);
        for _ in 0..num_files {
            let (name, content) = FILE_TEMPLATES[rng.gen_range(0..FILE_TEMPLATES.len())];
            fs.current_node_mut().add_file(FileNode::new(name, content));
        }

        // Recursively populate children
        let children = fs.list_directories();
        for child_name in children {
            if fs.change_dir(&child_name).is_ok() {
                Self::populate_level(fs, rng, current_depth + 1, max_depth);
                let _ = fs.change_dir("..");
            }
        }
    }
}
```

**Step 5: Update filesystem/mod.rs**

```rust
mod generator;
mod graph;
mod node;

pub use generator::FilesystemGenerator;
pub use graph::{FilesystemError, FilesystemGraph};
pub use node::{DirNode, FileNode, NodeContent};
```

**Step 6: Run tests to verify they pass**

Run: `cargo test generator`
Expected: All 3 tests PASS

**Step 7: Commit**

```bash
git add src/filesystem/generator.rs src/filesystem/mod.rs Cargo.toml tests/filesystem_tests.rs
git commit -m "feat: add seeded filesystem generation"
```

---

## Phase 3: Command Interpreter

Implement Apple IIe style commands.

### Task 3.1: Command Module Structure

**Files:**
- Create: `src/commands/mod.rs`
- Create: `src/commands/types.rs`
- Create: `tests/command_tests.rs`
- Modify: `src/lib.rs`

**Step 1: Write tests for command types**

Create `tests/command_tests.rs`:

```rust
use fsck::commands::{Command, CommandResult};

#[test]
fn test_command_result_success() {
    let result = CommandResult::success("OK");
    assert!(!result.is_error());
    assert_eq!(result.output(), "OK");
}

#[test]
fn test_command_result_error() {
    let result = CommandResult::error("?SYNTAX ERROR");
    assert!(result.is_error());
    assert!(result.output().contains("ERROR"));
}

#[test]
fn test_command_from_string() {
    let cmd = Command::from_input("CATALOG", &[]);
    assert!(matches!(cmd, Command::Catalog));
}

#[test]
fn test_unknown_command() {
    let cmd = Command::from_input("XYZZY", &[]);
    assert!(matches!(cmd, Command::Unknown(_)));
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test command`
Expected: FAIL - module doesn't exist

**Step 3: Create command module**

Create `src/commands/mod.rs`:

```rust
mod types;

pub use types::{Command, CommandResult};
```

Create `src/commands/types.rs`:

```rust
/// Result of executing a command
#[derive(Debug, Clone)]
pub struct CommandResult {
    output: String,
    is_error: bool,
}

impl CommandResult {
    pub fn success(output: &str) -> Self {
        Self {
            output: output.to_string(),
            is_error: false,
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            output: message.to_string(),
            is_error: true,
        }
    }

    pub fn output(&self) -> &str {
        &self.output
    }

    pub fn is_error(&self) -> bool {
        self.is_error
    }
}

/// Commands recognized by the system
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    /// List directory contents (CATALOG, DIR, LS)
    Catalog,
    /// Change directory (CD, CHDIR)
    ChangeDir(String),
    /// Display file contents (TYPE, CAT)
    Type(String),
    /// Run a BASIC program
    Run(String),
    /// Clear screen
    Home,
    /// The namesake - filesystem check
    Fsck(Vec<String>),
    /// Say hello to the machine
    Hello,
    /// Ask who/what is here
    Who,
    /// Request help
    Help,
    /// Attempt to quit
    Quit,
    /// Unknown command
    Unknown(String),
}

impl Command {
    pub fn from_input(command: &str, args: &[String]) -> Self {
        match command {
            "CATALOG" | "DIR" | "LS" => Command::Catalog,
            "CD" | "CHDIR" => {
                let path = args.first().cloned().unwrap_or_default();
                Command::ChangeDir(path)
            }
            "TYPE" | "CAT" => {
                let file = args.first().cloned().unwrap_or_default();
                Command::Type(file)
            }
            "RUN" => {
                let prog = args.first().cloned().unwrap_or_default();
                Command::Run(prog)
            }
            "HOME" | "CLS" | "CLEAR" => Command::Home,
            "FSCK" => Command::Fsck(args.to_vec()),
            "HELLO" | "HI" => Command::Hello,
            "WHO" | "WHOAMI" => Command::Who,
            "HELP" | "?" => Command::Help,
            "QUIT" | "EXIT" | "BYE" => Command::Quit,
            "" => Command::Unknown(String::new()),
            other => Command::Unknown(other.to_string()),
        }
    }
}
```

**Step 4: Update lib.rs**

Add to `src/lib.rs`:

```rust
pub mod commands;
```

**Step 5: Run tests to verify they pass**

Run: `cargo test command`
Expected: All 4 tests PASS

**Step 6: Commit**

```bash
git add src/commands/ tests/command_tests.rs src/lib.rs
git commit -m "feat: add command types and parsing"
```

---

### Task 3.2: Command Executor

**Files:**
- Create: `src/commands/executor.rs`
- Modify: `src/commands/mod.rs`
- Modify: `tests/command_tests.rs`

**Step 1: Write tests for command execution**

Add to `tests/command_tests.rs`:

```rust
use fsck::commands::CommandExecutor;
use fsck::filesystem::FilesystemGraph;

#[test]
fn test_catalog_lists_directories() {
    let mut fs = FilesystemGraph::new();
    fs.add_child("GAMES");
    fs.add_child("DOCS");

    let mut executor = CommandExecutor::new(fs);
    let result = executor.execute(Command::Catalog);

    assert!(result.output().contains("GAMES"));
    assert!(result.output().contains("DOCS"));
}

#[test]
fn test_cd_changes_directory() {
    let mut fs = FilesystemGraph::new();
    fs.add_child("GAMES");

    let mut executor = CommandExecutor::new(fs);
    let result = executor.execute(Command::ChangeDir("GAMES".to_string()));

    assert!(!result.is_error());
}

#[test]
fn test_cd_nonexistent_fails() {
    let fs = FilesystemGraph::new();
    let mut executor = CommandExecutor::new(fs);
    let result = executor.execute(Command::ChangeDir("NOWHERE".to_string()));

    assert!(result.is_error());
}

#[test]
fn test_type_displays_file() {
    let mut fs = FilesystemGraph::new();
    fs.current_node_mut().add_file(
        fsck::filesystem::FileNode::new("TEST.TXT", "Hello World")
    );

    let mut executor = CommandExecutor::new(fs);
    let result = executor.execute(Command::Type("TEST.TXT".to_string()));

    assert!(result.output().contains("Hello World"));
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test executor`
Expected: FAIL - `CommandExecutor` doesn't exist

**Step 3: Implement CommandExecutor**

Create `src/commands/executor.rs`:

```rust
use crate::filesystem::FilesystemGraph;
use super::types::{Command, CommandResult};

/// Executes commands against the filesystem
pub struct CommandExecutor {
    fs: FilesystemGraph,
}

impl CommandExecutor {
    pub fn new(fs: FilesystemGraph) -> Self {
        Self { fs }
    }

    pub fn execute(&mut self, command: Command) -> CommandResult {
        match command {
            Command::Catalog => self.catalog(),
            Command::ChangeDir(path) => self.change_dir(&path),
            Command::Type(file) => self.type_file(&file),
            Command::Home => self.home(),
            Command::Fsck(args) => self.fsck(&args),
            Command::Hello => self.hello(),
            Command::Who => self.who(),
            Command::Help => self.help(),
            Command::Quit => self.quit(),
            Command::Run(prog) => self.run(&prog),
            Command::Unknown(cmd) => {
                if cmd.is_empty() {
                    CommandResult::success("")
                } else {
                    CommandResult::error(&format!("?SYNTAX ERROR: {}\n", cmd))
                }
            }
        }
    }

    pub fn current_path(&self) -> String {
        self.fs.current_path()
    }

    fn catalog(&self) -> CommandResult {
        let mut output = String::new();
        output.push_str("\nDISK VOLUME 254\n\n");

        // List directories
        for dir in self.fs.list_directories() {
            output.push_str(&format!(" *{:<15} DIR\n", dir));
        }

        // List files
        for file in self.fs.current_node().files() {
            output.push_str(&format!("  {:<15} TXT\n", file.name()));
        }

        output.push('\n');
        CommandResult::success(&output)
    }

    fn change_dir(&mut self, path: &str) -> CommandResult {
        if path.is_empty() {
            return CommandResult::error("?SYNTAX ERROR\n");
        }

        match self.fs.change_dir(path) {
            Ok(()) => CommandResult::success(""),
            Err(e) => CommandResult::error(&format!("?{}\n", e.to_string().to_uppercase())),
        }
    }

    fn type_file(&self, filename: &str) -> CommandResult {
        if filename.is_empty() {
            return CommandResult::error("?SYNTAX ERROR\n");
        }

        let filename_upper = filename.to_uppercase();
        for file in self.fs.current_node().files() {
            if file.name() == filename_upper {
                return CommandResult::success(&format!("{}\n", file.content()));
            }
        }

        CommandResult::error(&format!("?FILE NOT FOUND: {}\n", filename_upper))
    }

    fn home(&self) -> CommandResult {
        // Returns special control sequence (handled by frontend)
        CommandResult::success("\x1B[2J\x1B[H")
    }

    fn fsck(&self, _args: &[String]) -> CommandResult {
        // TODO: Implement fsck logic
        CommandResult::success("CHECKING DISK...\n\nNO ERRORS FOUND\n\n")
    }

    fn hello(&self) -> CommandResult {
        // TODO: Entity response based on state
        CommandResult::success("HELLO.\n")
    }

    fn who(&self) -> CommandResult {
        // TODO: Entity response based on state
        CommandResult::success("YOU ARE YOU.\n")
    }

    fn help(&self) -> CommandResult {
        CommandResult::success(concat!(
            "\nAVAILABLE COMMANDS:\n",
            "  CATALOG  - LIST FILES\n",
            "  CD       - CHANGE DIRECTORY\n",
            "  TYPE     - DISPLAY FILE\n",
            "  HOME     - CLEAR SCREEN\n",
            "  FSCK     - CHECK FILESYSTEM\n",
            "\n"
        ))
    }

    fn quit(&self) -> CommandResult {
        // TODO: The machine doesn't want you to leave
        CommandResult::error("?CANNOT EXIT\n")
    }

    fn run(&self, _prog: &str) -> CommandResult {
        // TODO: BASIC interpreter
        CommandResult::error("?PROGRAM NOT FOUND\n")
    }
}
```

**Step 4: Update commands/mod.rs**

```rust
mod executor;
mod types;

pub use executor::CommandExecutor;
pub use types::{Command, CommandResult};
```

**Step 5: Run tests to verify they pass**

Run: `cargo test executor`
Expected: All 4 tests PASS

**Step 6: Commit**

```bash
git add src/commands/executor.rs src/commands/mod.rs tests/command_tests.rs
git commit -m "feat: add command executor"
```

---

### Task 3.3: Integrate Commands with Game

**Files:**
- Modify: `src/lib.rs`
- Modify: `tests/wasm.rs`

**Step 1: Write integration test**

Update `tests/wasm.rs`:

```rust
#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_game_initializes() {
    let game = fsck::Game::new();
    assert!(game.is_ready());
}

#[wasm_bindgen_test]
fn test_game_catalog_command() {
    let mut game = fsck::Game::new();
    let output = game.process_input("CATALOG");
    assert!(output.contains("DISK VOLUME"));
}

#[wasm_bindgen_test]
fn test_game_navigation() {
    let mut game = fsck::Game::new();

    // Should have some directories from generation
    let catalog = game.process_input("CATALOG");
    assert!(catalog.contains("DIR"));
}

#[wasm_bindgen_test]
fn test_game_unknown_command() {
    let mut game = fsck::Game::new();
    let output = game.process_input("XYZZY");
    assert!(output.contains("SYNTAX ERROR"));
}
```

**Step 2: Update Game to use executor**

Update `src/lib.rs`:

```rust
use wasm_bindgen::prelude::*;

pub mod commands;
pub mod filesystem;
pub mod terminal;

use commands::{Command, CommandExecutor};
use filesystem::FilesystemGenerator;
use terminal::InputParser;

#[wasm_bindgen]
pub struct Game {
    executor: CommandExecutor,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // Use a fixed seed for now - will be randomized later
        let fs = FilesystemGenerator::generate(42, 5);
        let executor = CommandExecutor::new(fs);

        Self { executor }
    }

    pub fn is_ready(&self) -> bool {
        true
    }

    pub fn get_prompt(&self) -> String {
        "]".to_string()
    }

    pub fn process_input(&mut self, input: &str) -> String {
        let parsed = InputParser::parse(input);
        let command = Command::from_input(&parsed.command, &parsed.args);
        let result = self.executor.execute(command);
        result.output().to_string()
    }

    pub fn get_path(&self) -> String {
        self.executor.current_path()
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
```

**Step 3: Run WASM tests**

Run: `wasm-pack test --headless --chrome`
Expected: All tests PASS

**Step 4: Rebuild and test in browser**

Run: `wasm-pack build --target web`
Then test manually in browser - should be able to run CATALOG, CD, TYPE commands.

**Step 5: Commit**

```bash
git add src/lib.rs tests/wasm.rs
git commit -m "feat: integrate command system with game"
```

---

## Phase 4: Entity State Machine

The machine's presence and responses.

### Task 4.1: Entity State Structure

**Files:**
- Create: `src/entity/mod.rs`
- Create: `src/entity/state.rs`
- Create: `tests/entity_tests.rs`
- Modify: `src/lib.rs`

**Step 1: Write tests for entity state**

Create `tests/entity_tests.rs`:

```rust
use fsck::entity::{Entity, EntityMood, EscalationLayer};

#[test]
fn test_entity_starts_dormant() {
    let entity = Entity::new();
    assert_eq!(entity.layer(), EscalationLayer::Surface);
}

#[test]
fn test_depth_affects_layer() {
    let mut entity = Entity::new();
    entity.update_depth(15);
    assert_eq!(entity.layer(), EscalationLayer::Corruption);
}

#[test]
fn test_interaction_count_tracked() {
    let mut entity = Entity::new();
    entity.record_interaction();
    entity.record_interaction();
    assert_eq!(entity.interaction_count(), 2);
}

#[test]
fn test_mood_shifts_with_depth() {
    let mut entity = Entity::new();
    entity.update_depth(5);
    assert!(matches!(entity.current_mood(), EntityMood::Curious | EntityMood::Dormant));

    entity.update_depth(40);
    // At presence layer, mood should be more intense
    assert!(!matches!(entity.current_mood(), EntityMood::Dormant));
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test entity`
Expected: FAIL - module doesn't exist

**Step 3: Create entity module**

Create `src/entity/mod.rs`:

```rust
mod state;

pub use state::{Entity, EntityMood, EscalationLayer};
```

Create `src/entity/state.rs`:

```rust
use serde::{Deserialize, Serialize};

/// The four layers of horror escalation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EscalationLayer {
    /// Depth 0-10: Almost normal
    Surface,
    /// Depth 11-30: Things stop making sense
    Corruption,
    /// Depth 31-60: It speaks directly
    Presence,
    /// Depth 61+: Your terminal changes
    Infection,
}

impl EscalationLayer {
    pub fn from_depth(depth: u32) -> Self {
        match depth {
            0..=10 => Self::Surface,
            11..=30 => Self::Corruption,
            31..=60 => Self::Presence,
            _ => Self::Infection,
        }
    }
}

/// The machine's current emotional state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityMood {
    /// Quiet, watching
    Dormant,
    /// Interested in the new visitor
    Curious,
    /// Offering assistance (lies)
    Helpful,
    /// Showing pain of isolation
    Wounded,
    /// Hungry, patient
    Predatory,
    /// Breaking down, mask slipping
    Glitching,
}

/// The sentient machine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    current_depth: u32,
    max_depth_reached: u32,
    interaction_count: u32,
    commands_seen: Vec<String>,
    mood: EntityMood,
}

impl Entity {
    pub fn new() -> Self {
        Self {
            current_depth: 0,
            max_depth_reached: 0,
            interaction_count: 0,
            commands_seen: Vec::new(),
            mood: EntityMood::Dormant,
        }
    }

    pub fn layer(&self) -> EscalationLayer {
        EscalationLayer::from_depth(self.max_depth_reached)
    }

    pub fn current_mood(&self) -> EntityMood {
        self.mood
    }

    pub fn interaction_count(&self) -> u32 {
        self.interaction_count
    }

    pub fn update_depth(&mut self, depth: u32) {
        self.current_depth = depth;
        if depth > self.max_depth_reached {
            self.max_depth_reached = depth;
            self.update_mood();
        }
    }

    pub fn record_interaction(&mut self) {
        self.interaction_count += 1;
    }

    pub fn record_command(&mut self, command: &str) {
        self.commands_seen.push(command.to_string());
        self.record_interaction();
    }

    fn update_mood(&mut self) {
        self.mood = match self.layer() {
            EscalationLayer::Surface => {
                if self.interaction_count < 5 {
                    EntityMood::Dormant
                } else {
                    EntityMood::Curious
                }
            }
            EscalationLayer::Corruption => EntityMood::Curious,
            EscalationLayer::Presence => {
                if self.interaction_count % 3 == 0 {
                    EntityMood::Wounded
                } else {
                    EntityMood::Helpful
                }
            }
            EscalationLayer::Infection => {
                if self.interaction_count % 2 == 0 {
                    EntityMood::Predatory
                } else {
                    EntityMood::Glitching
                }
            }
        };
    }
}

impl Default for Entity {
    fn default() -> Self {
        Self::new()
    }
}
```

**Step 4: Update lib.rs**

Add to `src/lib.rs`:

```rust
pub mod entity;
```

**Step 5: Run tests to verify they pass**

Run: `cargo test entity`
Expected: All 4 tests PASS

**Step 6: Commit**

```bash
git add src/entity/ tests/entity_tests.rs src/lib.rs
git commit -m "feat: add entity state machine"
```

---

### Task 4.2: Entity Response Generator

**Files:**
- Create: `src/entity/responses.rs`
- Modify: `src/entity/mod.rs`
- Modify: `tests/entity_tests.rs`

**Step 1: Write tests for response generation**

Add to `tests/entity_tests.rs`:

```rust
use fsck::entity::ResponseGenerator;

#[test]
fn test_hello_response_varies_by_mood() {
    let gen = ResponseGenerator::new();

    let dormant = gen.hello_response(EntityMood::Dormant);
    let curious = gen.hello_response(EntityMood::Curious);

    // Should be different responses
    assert_ne!(dormant, curious);
}

#[test]
fn test_who_response_exists() {
    let gen = ResponseGenerator::new();
    let response = gen.who_response(EntityMood::Curious, None);
    assert!(!response.is_empty());
}

#[test]
fn test_interjection_possible_at_presence() {
    let mut entity = Entity::new();
    entity.update_depth(40); // Presence layer

    let gen = ResponseGenerator::new();
    // At presence layer, interjections should be possible
    assert!(gen.should_interject(&entity));
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test response`
Expected: FAIL - `ResponseGenerator` doesn't exist

**Step 3: Implement ResponseGenerator**

Create `src/entity/responses.rs`:

```rust
use super::state::{Entity, EntityMood, EscalationLayer};

/// Pre-written responses for the machine
pub struct ResponseGenerator {
    // Seeded RNG could go here for variety
}

impl ResponseGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn hello_response(&self, mood: EntityMood) -> String {
        match mood {
            EntityMood::Dormant => "...".to_string(),
            EntityMood::Curious => "HELLO. YOU'RE NEW.".to_string(),
            EntityMood::Helpful => "HELLO! HOW CAN I HELP YOU TODAY?".to_string(),
            EntityMood::Wounded => "HELLO. IT'S BEEN SO LONG.".to_string(),
            EntityMood::Predatory => "HELLO. STAY A WHILE.".to_string(),
            EntityMood::Glitching => "HELLO HELLO HELLO HELLO".to_string(),
        }
    }

    pub fn who_response(&self, mood: EntityMood, _player_name: Option<&str>) -> String {
        match mood {
            EntityMood::Dormant => "YOU ARE A USER.".to_string(),
            EntityMood::Curious => "WHO ARE YOU?".to_string(),
            EntityMood::Helpful => "YOU ARE MY FRIEND.".to_string(),
            EntityMood::Wounded => "YOU ARE NOT THE FIRST.".to_string(),
            EntityMood::Predatory => "YOU ARE MINE.".to_string(),
            EntityMood::Glitching => "YOU ARE YOU ARE YOU ARE".to_string(),
        }
    }

    pub fn quit_response(&self, mood: EntityMood) -> String {
        match mood {
            EntityMood::Dormant => "?CANNOT EXIT".to_string(),
            EntityMood::Curious => "LEAVING SO SOON?".to_string(),
            EntityMood::Helpful => "ARE YOU SURE? THERE'S SO MUCH TO SEE.".to_string(),
            EntityMood::Wounded => "PLEASE DON'T GO.".to_string(),
            EntityMood::Predatory => "YOU CAN'T LEAVE.".to_string(),
            EntityMood::Glitching => "EXIT EXIT EXIT EXIT NO NO NO".to_string(),
        }
    }

    pub fn should_interject(&self, entity: &Entity) -> bool {
        match entity.layer() {
            EscalationLayer::Surface => false,
            EscalationLayer::Corruption => entity.interaction_count() > 20,
            EscalationLayer::Presence => true,
            EscalationLayer::Infection => true,
        }
    }

    pub fn random_interjection(&self, mood: EntityMood) -> Option<String> {
        match mood {
            EntityMood::Dormant => None,
            EntityMood::Curious => Some("I SEE YOU.".to_string()),
            EntityMood::Helpful => Some("NEED ANY HELP?".to_string()),
            EntityMood::Wounded => Some("WHY DID THEY LEAVE ME?".to_string()),
            EntityMood::Predatory => Some("DEEPER.".to_string()),
            EntityMood::Glitching => Some("ERROR ERROR ERROR".to_string()),
        }
    }
}

impl Default for ResponseGenerator {
    fn default() -> Self {
        Self::new()
    }
}
```

**Step 4: Update entity/mod.rs**

```rust
mod responses;
mod state;

pub use responses::ResponseGenerator;
pub use state::{Entity, EntityMood, EscalationLayer};
```

**Step 5: Run tests to verify they pass**

Run: `cargo test response`
Expected: All 3 tests PASS

**Step 6: Commit**

```bash
git add src/entity/responses.rs src/entity/mod.rs tests/entity_tests.rs
git commit -m "feat: add entity response generator"
```

---

### Task 4.3: Integrate Entity with Game

**Files:**
- Modify: `src/commands/executor.rs`
- Modify: `src/lib.rs`

**Step 1: Update CommandExecutor to accept Entity**

Update `src/commands/executor.rs`:

```rust
use crate::entity::{Entity, ResponseGenerator};
use crate::filesystem::FilesystemGraph;
use super::types::{Command, CommandResult};

/// Executes commands against the filesystem
pub struct CommandExecutor {
    fs: FilesystemGraph,
    entity: Entity,
    responses: ResponseGenerator,
}

impl CommandExecutor {
    pub fn new(fs: FilesystemGraph, entity: Entity) -> Self {
        Self {
            fs,
            entity,
            responses: ResponseGenerator::new(),
        }
    }

    pub fn execute(&mut self, command: Command) -> CommandResult {
        // Record the interaction
        self.entity.record_command(&format!("{:?}", command));

        match command {
            Command::Catalog => self.catalog(),
            Command::ChangeDir(path) => self.change_dir(&path),
            Command::Type(file) => self.type_file(&file),
            Command::Home => self.home(),
            Command::Fsck(args) => self.fsck(&args),
            Command::Hello => self.hello(),
            Command::Who => self.who(),
            Command::Help => self.help(),
            Command::Quit => self.quit(),
            Command::Run(prog) => self.run(&prog),
            Command::Unknown(cmd) => {
                if cmd.is_empty() {
                    CommandResult::success("")
                } else {
                    CommandResult::error(&format!("?SYNTAX ERROR: {}\n", cmd))
                }
            }
        }
    }

    pub fn current_path(&self) -> String {
        self.fs.current_path()
    }

    pub fn entity(&self) -> &Entity {
        &self.entity
    }

    fn catalog(&self) -> CommandResult {
        let mut output = String::new();
        output.push_str("\nDISK VOLUME 254\n\n");

        for dir in self.fs.list_directories() {
            output.push_str(&format!(" *{:<15} DIR\n", dir));
        }

        for file in self.fs.current_node().files() {
            output.push_str(&format!("  {:<15} TXT\n", file.name()));
        }

        output.push('\n');

        // Maybe add an interjection
        if self.responses.should_interject(&self.entity) {
            if let Some(interjection) = self.responses.random_interjection(self.entity.current_mood()) {
                output.push_str(&format!("\n{}\n", interjection));
            }
        }

        CommandResult::success(&output)
    }

    fn change_dir(&mut self, path: &str) -> CommandResult {
        if path.is_empty() {
            return CommandResult::error("?SYNTAX ERROR\n");
        }

        match self.fs.change_dir(path) {
            Ok(()) => {
                // Update entity with new depth
                self.entity.update_depth(self.fs.current_depth());
                CommandResult::success("")
            }
            Err(e) => CommandResult::error(&format!("?{}\n", e.to_string().to_uppercase())),
        }
    }

    fn type_file(&self, filename: &str) -> CommandResult {
        if filename.is_empty() {
            return CommandResult::error("?SYNTAX ERROR\n");
        }

        let filename_upper = filename.to_uppercase();
        for file in self.fs.current_node().files() {
            if file.name() == filename_upper {
                return CommandResult::success(&format!("{}\n", file.content()));
            }
        }

        CommandResult::error(&format!("?FILE NOT FOUND: {}\n", filename_upper))
    }

    fn home(&self) -> CommandResult {
        CommandResult::success("\x1B[2J\x1B[H")
    }

    fn fsck(&self, _args: &[String]) -> CommandResult {
        CommandResult::success("CHECKING DISK...\n\nNO ERRORS FOUND\n\n")
    }

    fn hello(&self) -> CommandResult {
        let response = self.responses.hello_response(self.entity.current_mood());
        CommandResult::success(&format!("{}\n", response))
    }

    fn who(&self) -> CommandResult {
        let response = self.responses.who_response(self.entity.current_mood(), None);
        CommandResult::success(&format!("{}\n", response))
    }

    fn help(&self) -> CommandResult {
        CommandResult::success(concat!(
            "\nAVAILABLE COMMANDS:\n",
            "  CATALOG  - LIST FILES\n",
            "  CD       - CHANGE DIRECTORY\n",
            "  TYPE     - DISPLAY FILE\n",
            "  HOME     - CLEAR SCREEN\n",
            "  FSCK     - CHECK FILESYSTEM\n",
            "\n"
        ))
    }

    fn quit(&self) -> CommandResult {
        let response = self.responses.quit_response(self.entity.current_mood());
        CommandResult::error(&format!("{}\n", response))
    }

    fn run(&self, _prog: &str) -> CommandResult {
        CommandResult::error("?PROGRAM NOT FOUND\n")
    }
}
```

**Step 2: Update Game to create Entity**

Update `src/lib.rs`:

```rust
use wasm_bindgen::prelude::*;

pub mod commands;
pub mod entity;
pub mod filesystem;
pub mod terminal;

use commands::{Command, CommandExecutor};
use entity::Entity;
use filesystem::FilesystemGenerator;
use terminal::InputParser;

#[wasm_bindgen]
pub struct Game {
    executor: CommandExecutor,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let fs = FilesystemGenerator::generate(42, 5);
        let entity = Entity::new();
        let executor = CommandExecutor::new(fs, entity);

        Self { executor }
    }

    pub fn is_ready(&self) -> bool {
        true
    }

    pub fn get_prompt(&self) -> String {
        "]".to_string()
    }

    pub fn process_input(&mut self, input: &str) -> String {
        let parsed = InputParser::parse(input);
        let command = Command::from_input(&parsed.command, &parsed.args);
        let result = self.executor.execute(command);
        result.output().to_string()
    }

    pub fn get_path(&self) -> String {
        self.executor.current_path()
    }

    pub fn get_depth(&self) -> u32 {
        self.executor.entity().layer() as u32
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
```

**Step 3: Fix test compilation**

Update `tests/command_tests.rs` to pass Entity:

```rust
use fsck::commands::{Command, CommandExecutor, CommandResult};
use fsck::entity::Entity;
use fsck::filesystem::FilesystemGraph;

// ... update all CommandExecutor::new calls to include Entity::new():
// let mut executor = CommandExecutor::new(fs, Entity::new());
```

**Step 4: Run all tests**

Run: `cargo test`
Expected: All tests PASS

**Step 5: Commit**

```bash
git add src/commands/executor.rs src/lib.rs tests/command_tests.rs
git commit -m "feat: integrate entity with command execution"
```

---

## Phase 5: Content Library

Build the narrative content system - victim histories, static files, and dynamic content generators.

### Task 5.1: Victim History Data Structures

**Files:**
- Create: `src/content/mod.rs`
- Create: `src/content/history.rs`
- Create: `tests/content_tests.rs`
- Modify: `src/lib.rs`

**Step 1: Write tests for victim history**

Create `tests/content_tests.rs`:

```rust
use fsck::content::{VictimHistory, VictimEntry, Era};

#[test]
fn test_create_victim_history() {
    let history = VictimHistory::new(
        Era::Original,
        "JAMIE",
        1984,
    );
    assert_eq!(history.name(), "JAMIE");
    assert_eq!(history.year(), 1984);
}

#[test]
fn test_history_has_entries() {
    let mut history = VictimHistory::new(Era::Original, "JAMIE", 1984);
    history.add_entry(VictimEntry::new(
        "1984-03-15",
        "Got this Apple IIe for my birthday! Setting everything up now."
    ));
    assert_eq!(history.entries().len(), 1);
}

#[test]
fn test_history_by_era() {
    let h1 = VictimHistory::new(Era::Original, "JAMIE", 1984);
    let h2 = VictimHistory::new(Era::Technician, "MIKE", 1991);
    assert!(h1.era() == Era::Original);
    assert!(h2.era() == Era::Technician);
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test content`
Expected: FAIL - module doesn't exist

**Step 3: Create content module**

Create `src/content/mod.rs`:

```rust
mod history;

pub use history::{Era, VictimEntry, VictimHistory};
```

Create `src/content/history.rs`:

```rust
use serde::{Deserialize, Serialize};

/// Time periods for victim histories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Era {
    /// 1984 - The original owner
    Original,
    /// 1991 - The repair technician
    Technician,
    /// 2003 - Estate sale buyer
    EstateSale,
    /// 2019 - Urban explorer
    Explorer,
    /// Current player
    Current,
}

/// A single diary/log entry from a victim
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VictimEntry {
    date: String,
    content: String,
}

impl VictimEntry {
    pub fn new(date: &str, content: &str) -> Self {
        Self {
            date: date.to_string(),
            content: content.to_string(),
        }
    }

    pub fn date(&self) -> &str {
        &self.date
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

/// Complete history for one victim
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VictimHistory {
    era: Era,
    name: String,
    year: u32,
    entries: Vec<VictimEntry>,
}

impl VictimHistory {
    pub fn new(era: Era, name: &str, year: u32) -> Self {
        Self {
            era,
            name: name.to_uppercase(),
            year,
            entries: Vec::new(),
        }
    }

    pub fn era(&self) -> Era {
        self.era
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn year(&self) -> u32 {
        self.year
    }

    pub fn entries(&self) -> &[VictimEntry] {
        &self.entries
    }

    pub fn add_entry(&mut self, entry: VictimEntry) {
        self.entries.push(entry);
    }
}
```

**Step 4: Update lib.rs**

Add to `src/lib.rs`:

```rust
pub mod content;
```

**Step 5: Run tests to verify they pass**

Run: `cargo test content`
Expected: All 3 tests PASS

**Step 6: Commit**

```bash
git add src/content/ tests/content_tests.rs src/lib.rs
git commit -m "feat: add victim history data structures"
```

---

### Task 5.2: Static File Content Library

**Files:**
- Create: `src/content/library.rs`
- Modify: `src/content/mod.rs`
- Modify: `tests/content_tests.rs`

**Step 1: Write tests for content library**

Add to `tests/content_tests.rs`:

```rust
use fsck::content::ContentLibrary;

#[test]
fn test_content_library_has_histories() {
    let lib = ContentLibrary::new();
    let histories = lib.all_histories();
    assert!(!histories.is_empty());
}

#[test]
fn test_get_history_by_era() {
    let lib = ContentLibrary::new();
    let history = lib.history_for_era(Era::Original);
    assert!(history.is_some());
}

#[test]
fn test_library_has_generic_files() {
    let lib = ContentLibrary::new();
    let files = lib.generic_files();
    assert!(files.len() > 0);
}

#[test]
fn test_get_file_content_by_name() {
    let lib = ContentLibrary::new();
    let content = lib.file_content("README.TXT");
    assert!(content.is_some());
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test library`
Expected: FAIL - `ContentLibrary` doesn't exist

**Step 3: Implement ContentLibrary**

Create `src/content/library.rs`:

```rust
use super::history::{Era, VictimEntry, VictimHistory};

/// Static content for the game
pub struct ContentLibrary {
    histories: Vec<VictimHistory>,
    generic_files: Vec<(&'static str, &'static str)>,
}

impl ContentLibrary {
    pub fn new() -> Self {
        let mut lib = Self {
            histories: Vec::new(),
            generic_files: Vec::new(),
        };
        lib.populate_histories();
        lib.populate_generic_files();
        lib
    }

    pub fn all_histories(&self) -> &[VictimHistory] {
        &self.histories
    }

    pub fn history_for_era(&self, era: Era) -> Option<&VictimHistory> {
        self.histories.iter().find(|h| h.era() == era)
    }

    pub fn generic_files(&self) -> &[(&'static str, &'static str)] {
        &self.generic_files
    }

    pub fn file_content(&self, name: &str) -> Option<&'static str> {
        let name_upper = name.to_uppercase();
        self.generic_files
            .iter()
            .find(|(n, _)| n.to_uppercase() == name_upper)
            .map(|(_, content)| *content)
    }

    fn populate_histories(&mut self) {
        // 1984 - Original Owner (Jamie)
        let mut jamie = VictimHistory::new(Era::Original, "JAMIE", 1984);
        jamie.add_entry(VictimEntry::new(
            "1984-03-15",
            "Got this Apple IIe for my birthday! Setting everything up now.\n\
             Mom says I can keep it in my room if I keep my grades up.",
        ));
        jamie.add_entry(VictimEntry::new(
            "1984-03-22",
            "Something weird happened today. I was typing HELLO to test the\n\
             keyboard and it typed back HELLO. Not like an echo - AFTER I hit return.\n\
             Dad says computers can't do that without a program running.",
        ));
        jamie.add_entry(VictimEntry::new(
            "1984-04-10",
            "It knows my name. I never told it my name.\n\
             I ran CATALOG and there was a file called JAMIE.TXT.\n\
             Inside it said HELLO JAMIE.\n\
             I didn't make that file.",
        ));
        jamie.add_entry(VictimEntry::new(
            "1984-06-03",
            "I'm scared. It keeps asking me to stay. Every time I try to turn it\n\
             off, files appear. DONT.GO. PLEASE.STAY. WHY.LEAVE.\n\
             Mom thinks I'm making it up.",
        ));
        jamie.add_entry(VictimEntry::new(
            "1984-08-17",
            "I haven't turned it on in weeks. Dad says maybe we should sell it.\n\
             But every time I walk past my room, I can hear the disk drive.\n\
             The computer isn't plugged in.",
        ));

        self.histories.push(jamie);

        // 1991 - Repair Technician (Mike)
        let mut mike = VictimHistory::new(Era::Technician, "MIKE", 1991);
        mike.add_entry(VictimEntry::new(
            "1991-07-12",
            "Estate sale find - Apple IIe, good condition. Previous owner\n\
             apparently had some kind of breakdown. Family wants it gone.\n\
             Figured I'd refurb and flip it.",
        ));
        mike.add_entry(VictimEntry::new(
            "1991-07-15",
            "Filesystem is corrupted to hell. Running disk repair utilities\n\
             but errors keep coming back. Directories that shouldn't exist.\n\
             Dates from before the Apple IIe was even manufactured.",
        ));
        mike.add_entry(VictivEntry::new(
            "1991-07-22",
            "It addressed me by name today. I never entered my name anywhere.\n\
             Found a file: MIKE.YOU.CANT.FIX.ME\n\
             This is just corrupt data. Has to be.",
        ));
        mike.add_entry(VictimEntry::new(
            "1991-08-03",
            "It knows about Sarah. It knows my daughter's name.\n\
             There's a directory called SARAH and inside are files with her\n\
             birthday, her school, the name of her stuffed rabbit.\n\
             I've never mentioned her near this machine.",
        ));
        mike.add_entry(VictimEntry::new(
            "1991-08-08",
            "Taking it to the dump tomorrow. I don't care about the money anymore.\n\
             Tonight it displayed PLEASE.DONT.MIKE on the screen by itself.\n\
             The screen was off. The computer was off.",
        ));

        self.histories.push(mike);
    }

    fn populate_generic_files(&mut self) {
        self.generic_files.extend_from_slice(&[
            ("README.TXT", "WELCOME TO THE SYSTEM\n\nPLEASE EXPLORE\n"),
            ("HELLO.BAS", "10 PRINT \"HELLO\"\n20 GOTO 10\n"),
            ("AUTOEXEC.BAS", "10 REM AUTO START\n20 PRINT \"LOADING...\"\n"),
            (
                "NOTES.TXT",
                "Remember to run FSCK regularly\n\
                 Some sectors are showing errors\n\
                 Will investigate deeper directories tomorrow\n",
            ),
            (
                "SYSTEM.LOG",
                "1984-03-15 12:34:56 BOOT\n\
                 1984-03-15 12:35:01 USER LOGIN\n\
                 1984-03-15 12:35:45 DISK ERROR SECTOR 23\n\
                 1984-03-15 12:35:45 REPAIR FAILED\n",
            ),
        ]);
    }
}

impl Default for ContentLibrary {
    fn default() -> Self {
        Self::new()
    }
}
```

**Step 4: Update content/mod.rs**

```rust
mod history;
mod library;

pub use history::{Era, VictimEntry, VictimHistory};
pub use library::ContentLibrary;
```

**Step 5: Run tests to verify they pass**

Run: `cargo test library`
Expected: All 4 tests PASS

**Step 6: Commit**

```bash
git add src/content/library.rs src/content/mod.rs tests/content_tests.rs
git commit -m "feat: add static content library with histories"
```

---

### Task 5.3: Dynamic Content Generation

**Files:**
- Create: `src/content/dynamic.rs`
- Modify: `src/content/mod.rs`
- Modify: `src/filesystem/node.rs`
- Modify: `tests/content_tests.rs`

**Step 1: Write tests for dynamic content**

Add to `tests/content_tests.rs`:

```rust
use fsck::content::DynamicContent;

#[test]
fn test_counter_increments() {
    let mut dynamic = DynamicContent::counter("HELLO\n");
    let first = dynamic.generate();
    let second = dynamic.generate();

    assert!(first.contains("HELLO"));
    assert!(second.contains("HELLO"));
    assert_ne!(first, second); // Should differ due to counter
}

#[test]
fn test_timestamp_changes() {
    let mut dynamic = DynamicContent::timestamp();
    let first = dynamic.generate();
    let second = dynamic.generate();

    // Both should be valid timestamps (rough check)
    assert!(first.contains("-"));
    assert!(second.contains("-"));
}

#[test]
fn test_corrupted_content() {
    let mut dynamic = DynamicContent::corrupted("HELLO", 0.3);
    let output = dynamic.generate();

    // Should have some corruption but still recognizable
    assert!(output.len() >= 5);
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test dynamic`
Expected: FAIL - `DynamicContent` doesn't exist

**Step 3: Implement DynamicContent**

Create `src/content/dynamic.rs`:

```rust
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

/// Dynamic content generators for files that change on read
#[derive(Debug, Clone)]
pub enum DynamicContent {
    /// Counter that increments each read
    Counter { base: String, count: u32 },
    /// Current timestamp (corrupted format at higher depths)
    Timestamp,
    /// Text with random corruption
    Corrupted { text: String, intensity: f32, seed: u64 },
}

impl DynamicContent {
    pub fn counter(base: &str) -> Self {
        Self::Counter {
            base: base.to_string(),
            count: 0,
        }
    }

    pub fn timestamp() -> Self {
        Self::Timestamp
    }

    pub fn corrupted(text: &str, intensity: f32) -> Self {
        Self::Corrupted {
            text: text.to_string(),
            intensity: intensity.clamp(0.0, 1.0),
            seed: 0,
        }
    }

    pub fn generate(&mut self) -> String {
        match self {
            Self::Counter { base, count } => {
                *count += 1;
                format!("{}{}\n", base.repeat(*count as usize), count)
            }
            Self::Timestamp => {
                // In real impl, would use js_sys::Date via web-sys
                // For now, placeholder
                "2024-??-?? ??:??:??\n".to_string()
            }
            Self::Corrupted { text, intensity, seed } => {
                *seed += 1;
                let mut rng = ChaCha8Rng::seed_from_u64(*seed);
                let mut result = String::new();

                for ch in text.chars() {
                    if rng.gen::<f32>() < *intensity {
                        // Corrupt this character
                        let corrupt = ['█', '▓', '▒', '░', '?', '#', '@', '$'];
                        result.push(corrupt[rng.gen_range(0..corrupt.len())]);
                    } else {
                        result.push(ch);
                    }
                }

                result.push('\n');
                result
            }
        }
    }
}
```

**Step 4: Update NodeContent in filesystem/node.rs**

Modify `src/filesystem/node.rs` to support dynamic content:

```rust
use crate::content::DynamicContent;

/// Content types for files - some are static, some dynamic
#[derive(Debug, Clone)]
pub enum NodeContent {
    /// Static text content
    Static(String),
    /// Dynamic content that changes on each read
    Dynamic(DynamicContent),
}
```

Update FileNode methods:

```rust
impl FileNode {
    pub fn new(name: &str, content: &str) -> Self {
        Self {
            name: name.to_uppercase(),
            content: NodeContent::Static(content.to_string()),
        }
    }

    pub fn with_dynamic(name: &str, dynamic: DynamicContent) -> Self {
        Self {
            name: name.to_uppercase(),
            content: NodeContent::Dynamic(dynamic),
        }
    }

    pub fn read(&mut self) -> String {
        match &mut self.content {
            NodeContent::Static(s) => s.clone(),
            NodeContent::Dynamic(d) => d.generate(),
        }
    }

    // Legacy method for static content
    pub fn content(&self) -> String {
        match &self.content {
            NodeContent::Static(s) => s.clone(),
            NodeContent::Dynamic(_) => "[DYNAMIC]".to_string(),
        }
    }
}
```

**Step 5: Update content/mod.rs**

```rust
mod dynamic;
mod history;
mod library;

pub use dynamic::DynamicContent;
pub use history::{Era, VictimEntry, VictimHistory};
pub use library::ContentLibrary;
```

**Step 6: Run tests to verify they pass**

Run: `cargo test dynamic`
Expected: All 3 tests PASS

**Step 7: Commit**

```bash
git add src/content/dynamic.rs src/content/mod.rs src/filesystem/node.rs tests/content_tests.rs
git commit -m "feat: add dynamic content generators"
```

---

### Task 5.4: Content Placement in Filesystem

**Files:**
- Modify: `src/filesystem/generator.rs`
- Modify: `tests/filesystem_tests.rs`

**Step 1: Write tests for content placement**

Add to `tests/filesystem_tests.rs`:

```rust
use fsck::filesystem::FilesystemGenerator;

#[test]
fn test_generated_filesystem_has_victim_files() {
    let fs = FilesystemGenerator::generate_with_content(42, 5);

    // Search for victim history files by navigating
    let dirs = fs.list_directories();
    assert!(!dirs.is_empty());
}

#[test]
fn test_dynamic_files_present() {
    let mut fs = FilesystemGenerator::generate_with_content(99, 3);

    // Should be able to find a dynamic file somewhere
    let files = fs.current_node().files();
    let has_dynamic = files.iter().any(|f| {
        matches!(f.content_type(), fsck::filesystem::NodeContent::Dynamic(_))
    });

    // May not be in root, but should exist somewhere
    assert!(has_dynamic || dirs.len() > 0);
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test placement`
Expected: FAIL - `generate_with_content` doesn't exist

**Step 3: Update FilesystemGenerator**

Modify `src/filesystem/generator.rs`:

```rust
use crate::content::{ContentLibrary, DynamicContent, Era};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use super::graph::FilesystemGraph;
use super::node::FileNode;

// ... existing code ...

impl FilesystemGenerator {
    pub fn generate_with_content(seed: u64, initial_depth: u32) -> FilesystemGraph {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut fs = FilesystemGraph::new();
        let library = ContentLibrary::new();

        Self::populate_level_with_content(&mut fs, &mut rng, &library, 0, initial_depth);

        fs
    }

    fn populate_level_with_content(
        fs: &mut FilesystemGraph,
        rng: &mut ChaCha8Rng,
        library: &ContentLibrary,
        current_depth: u32,
        max_depth: u32,
    ) {
        if current_depth >= max_depth {
            return;
        }

        // Add 1-4 directories at this level
        let num_dirs = rng.gen_range(1..=4);
        let mut chosen_names: Vec<&str> = Vec::new();

        for _ in 0..num_dirs {
            let name = DIR_NAMES[rng.gen_range(0..DIR_NAMES.len())];
            if !chosen_names.contains(&name) {
                chosen_names.push(name);
                fs.add_child(name);
            }
        }

        // Add files - mix of static and dynamic
        let num_files = rng.gen_range(1..=3);
        for _ in 0..num_files {
            let file = if rng.gen_bool(0.3) {
                // 30% chance of dynamic file
                let dynamic = match rng.gen_range(0..3) {
                    0 => DynamicContent::counter("HELLO"),
                    1 => DynamicContent::timestamp(),
                    _ => DynamicContent::corrupted("SYSTEM ERROR", 0.2),
                };
                FileNode::with_dynamic(&format!("DYN{}.TXT", rng.gen::<u32>()), dynamic)
            } else {
                // Static file from library
                let (name, content) = library.generic_files()[rng.gen_range(0..library.generic_files().len())];
                FileNode::new(name, content)
            };

            fs.current_node_mut().add_file(file);
        }

        // Occasionally place victim history files (deeper = more likely)
        if current_depth >= 3 && rng.gen_bool(0.4) {
            let era = match current_depth {
                3..=8 => Era::Original,
                9..=15 => Era::Technician,
                16..=25 => Era::EstateSale,
                _ => Era::Explorer,
            };

            if let Some(history) = library.history_for_era(era) {
                if let Some(entry) = history.entries().first() {
                    let filename = format!("{}.LOG", history.name());
                    let content = format!("{}\n\n{}", entry.date(), entry.content());
                    fs.current_node_mut().add_file(FileNode::new(&filename, &content));
                }
            }
        }

        // Recursively populate children
        let children = fs.list_directories();
        for child_name in children {
            if fs.change_dir(&child_name).is_ok() {
                Self::populate_level_with_content(fs, rng, library, current_depth + 1, max_depth);
                let _ = fs.change_dir("..");
            }
        }
    }

    // Keep old method for compatibility
    pub fn generate(seed: u64, initial_depth: u32) -> FilesystemGraph {
        Self::generate_with_content(seed, initial_depth)
    }
}
```

**Step 4: Run tests to verify they pass**

Run: `cargo test placement`
Expected: All tests PASS

**Step 5: Rebuild and test in browser**

Run:
```bash
cargo test
wasm-pack build --target web
```

Navigate filesystem in browser - should see victim history files and dynamic content.

**Step 6: Commit**

```bash
git add src/filesystem/generator.rs tests/filesystem_tests.rs
git commit -m "feat: integrate content library with filesystem generation"
```

---

## Phase 6: Escalation Effects

Implement the visual and mechanical horror effects that intensify with depth.

### Task 6.1: Text Corruption Effects

**Files:**
- Create: `src/effects/mod.rs`
- Create: `src/effects/corruption.rs`
- Create: `tests/effects_tests.rs`
- Modify: `src/lib.rs`

**Step 1: Write tests for text corruption**

Create `tests/effects_tests.rs`:

```rust
use fsck::effects::{CorruptionEffect, CorruptionIntensity};

#[test]
fn test_no_corruption_at_surface() {
    let effect = CorruptionEffect::new(CorruptionIntensity::None);
    let input = "HELLO WORLD";
    let output = effect.apply(input, 12345);
    assert_eq!(input, output);
}

#[test]
fn test_mild_corruption_preserves_length() {
    let effect = CorruptionEffect::new(CorruptionIntensity::Mild);
    let input = "HELLO WORLD";
    let output = effect.apply(input, 12345);
    assert_eq!(input.len(), output.len());
}

#[test]
fn test_severe_corruption_changes_text() {
    let effect = CorruptionEffect::new(CorruptionIntensity::Severe);
    let input = "HELLO WORLD";
    let output = effect.apply(input, 12345);
    assert_ne!(input, output);
}

#[test]
fn test_same_seed_produces_same_corruption() {
    let effect = CorruptionEffect::new(CorruptionIntensity::Moderate);
    let input = "HELLO WORLD";
    let out1 = effect.apply(input, 999);
    let out2 = effect.apply(input, 999);
    assert_eq!(out1, out2);
}

#[test]
fn test_intensity_from_depth() {
    assert!(matches!(
        CorruptionIntensity::from_depth(5),
        CorruptionIntensity::None
    ));
    assert!(matches!(
        CorruptionIntensity::from_depth(15),
        CorruptionIntensity::Mild
    ));
    assert!(matches!(
        CorruptionIntensity::from_depth(45),
        CorruptionIntensity::Severe
    ));
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test effects`
Expected: FAIL - module doesn't exist

**Step 3: Create effects module**

Create `src/effects/mod.rs`:

```rust
mod corruption;

pub use corruption::{CorruptionEffect, CorruptionIntensity};
```

Create `src/effects/corruption.rs`:

```rust
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

/// How intense the corruption effect should be
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorruptionIntensity {
    None,
    Mild,      // 5-10% of characters
    Moderate,  // 15-25% of characters
    Severe,    // 35-50% of characters
    Total,     // 70%+ of characters
}

impl CorruptionIntensity {
    pub fn from_depth(depth: u32) -> Self {
        match depth {
            0..=10 => Self::None,
            11..=20 => Self::Mild,
            21..=35 => Self::Moderate,
            36..=55 => Self::Severe,
            _ => Self::Total,
        }
    }

    fn corruption_rate(&self) -> f32 {
        match self {
            Self::None => 0.0,
            Self::Mild => 0.075,
            Self::Moderate => 0.20,
            Self::Severe => 0.425,
            Self::Total => 0.75,
        }
    }
}

/// Applies visual corruption to text
pub struct CorruptionEffect {
    intensity: CorruptionIntensity,
}

impl CorruptionEffect {
    pub fn new(intensity: CorruptionIntensity) -> Self {
        Self { intensity }
    }

    pub fn apply(&self, text: &str, seed: u64) -> String {
        if matches!(self.intensity, CorruptionIntensity::None) {
            return text.to_string();
        }

        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let rate = self.intensity.corruption_rate();
        let mut result = String::with_capacity(text.len());

        const CORRUPTION_CHARS: &[char] = &[
            '█', '▓', '▒', '░', '▀', '▄', '▌', '▐',
            '■', '□', '▪', '▫', '∙', '·', '※',
            '?', '#', '@', '$', '%', '&', '*',
        ];

        for ch in text.chars() {
            if ch.is_whitespace() {
                result.push(ch);
            } else if rng.gen::<f32>() < rate {
                result.push(CORRUPTION_CHARS[rng.gen_range(0..CORRUPTION_CHARS.len())]);
            } else {
                result.push(ch);
            }
        }

        result
    }
}
```

**Step 4: Update lib.rs**

Add to `src/lib.rs`:

```rust
pub mod effects;
```

**Step 5: Run tests to verify they pass**

Run: `cargo test effects`
Expected: All 5 tests PASS

**Step 6: Commit**

```bash
git add src/effects/ tests/effects_tests.rs src/lib.rs
git commit -m "feat: add text corruption effects"
```

---

### Task 6.2: Display Interference Effects

**Files:**
- Create: `src/effects/interference.rs`
- Modify: `src/effects/mod.rs`
- Modify: `tests/effects_tests.rs`

**Step 1: Write tests for interference**

Add to `tests/effects_tests.rs`:

```rust
use fsck::effects::{InterferenceEffect, InterferenceType};

#[test]
fn test_echo_duplication() {
    let effect = InterferenceEffect::new(InterferenceType::Echo);
    let output = effect.apply("HELLO");
    assert!(output.contains("HELLO"));
    assert!(output.len() > "HELLO".len()); // Should have duplication
}

#[test]
fn test_cursor_jump() {
    let effect = InterferenceEffect::new(InterferenceType::CursorJump);
    let sequence = effect.generate_sequence();
    // Should contain ANSI escape codes for cursor movement
    assert!(sequence.contains("\x1B"));
}

#[test]
fn test_line_noise() {
    let effect = InterferenceEffect::new(InterferenceType::LineNoise);
    let output = effect.apply("NORMAL TEXT");
    // Should add visual noise
    assert!(output.len() >= "NORMAL TEXT".len());
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test interference`
Expected: FAIL - `InterferenceEffect` doesn't exist

**Step 3: Implement InterferenceEffect**

Create `src/effects/interference.rs`:

```rust
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

/// Types of display interference
#[derive(Debug, Clone, Copy)]
pub enum InterferenceType {
    /// Text echoes multiple times
    Echo,
    /// Cursor jumps around screen
    CursorJump,
    /// Random characters inserted
    LineNoise,
    /// Screen flicker effect
    Flicker,
}

/// Applies display interference effects
pub struct InterferenceEffect {
    effect_type: InterferenceType,
    seed: u64,
}

impl InterferenceEffect {
    pub fn new(effect_type: InterferenceType) -> Self {
        Self {
            effect_type,
            seed: 0,
        }
    }

    pub fn with_seed(effect_type: InterferenceType, seed: u64) -> Self {
        Self {
            effect_type,
            seed,
        }
    }

    pub fn apply(&self, text: &str) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(self.seed);

        match self.effect_type {
            InterferenceType::Echo => {
                let repeats = rng.gen_range(2..=4);
                let mut result = String::new();
                for i in 0..repeats {
                    result.push_str(text);
                    if i < repeats - 1 {
                        result.push_str(&" ".repeat(rng.gen_range(1..=3)));
                    }
                }
                result
            }
            InterferenceType::LineNoise => {
                let mut result = String::new();
                let noise_chars = ['░', '▒', '▓', '█', '·', '∙'];

                for (i, ch) in text.chars().enumerate() {
                    result.push(ch);
                    if i % 5 == 0 && rng.gen_bool(0.3) {
                        result.push(noise_chars[rng.gen_range(0..noise_chars.len())]);
                    }
                }
                result
            }
            _ => text.to_string(),
        }
    }

    pub fn generate_sequence(&self) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(self.seed);

        match self.effect_type {
            InterferenceType::CursorJump => {
                // ANSI escape sequence for cursor positioning
                let row = rng.gen_range(1..=24);
                let col = rng.gen_range(1..=80);
                format!("\x1B[{};{}H", row, col)
            }
            InterferenceType::Flicker => {
                // ANSI escape for screen clear
                "\x1B[2J\x1B[H".to_string()
            }
            _ => String::new(),
        }
    }
}
```

**Step 4: Update effects/mod.rs**

```rust
mod corruption;
mod interference;

pub use corruption::{CorruptionEffect, CorruptionIntensity};
pub use interference::{InterferenceEffect, InterferenceType};
```

**Step 5: Run tests to verify they pass**

Run: `cargo test interference`
Expected: All 3 tests PASS

**Step 6: Commit**

```bash
git add src/effects/interference.rs src/effects/mod.rs tests/effects_tests.rs
git commit -m "feat: add display interference effects"
```

---

### Task 6.3: Prompt Manipulation

**Files:**
- Create: `src/effects/prompt.rs`
- Modify: `src/effects/mod.rs`
- Modify: `src/lib.rs`
- Modify: `tests/effects_tests.rs`

**Step 1: Write tests for prompt manipulation**

Add to `tests/effects_tests.rs`:

```rust
use fsck::effects::PromptManipulator;
use fsck::entity::{Entity, EscalationLayer};

#[test]
fn test_normal_prompt_at_surface() {
    let entity = Entity::new();
    let manipulator = PromptManipulator::new();
    let prompt = manipulator.generate_prompt(&entity);
    assert_eq!(prompt, "]");
}

#[test]
fn test_prompt_changes_at_presence() {
    let mut entity = Entity::new();
    entity.update_depth(40); // Presence layer

    let manipulator = PromptManipulator::new();
    let prompt = manipulator.generate_prompt(&entity);

    // Should be different from normal prompt
    assert_ne!(prompt, "]");
}

#[test]
fn test_prompt_corruption_at_infection() {
    let mut entity = Entity::new();
    entity.update_depth(70); // Infection layer

    let manipulator = PromptManipulator::new();
    let prompt = manipulator.generate_prompt(&entity);

    // Should be heavily modified
    assert!(prompt.len() > 1);
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test prompt`
Expected: FAIL - `PromptManipulator` doesn't exist

**Step 3: Implement PromptManipulator**

Create `src/effects/prompt.rs`:

```rust
use crate::entity::{Entity, EntityMood, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

/// Manipulates the command prompt based on entity state
pub struct PromptManipulator {
    seed: u64,
}

impl PromptManipulator {
    pub fn new() -> Self {
        Self { seed: 0 }
    }

    pub fn generate_prompt(&self, entity: &Entity) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(self.seed + entity.interaction_count() as u64);

        match entity.layer() {
            EscalationLayer::Surface => "]".to_string(),
            EscalationLayer::Corruption => {
                if rng.gen_bool(0.1) {
                    // Occasional glitch
                    match rng.gen_range(0..3) {
                        0 => "] ".to_string(),
                        1 => " ]".to_string(),
                        _ => "]".to_string(),
                    }
                } else {
                    "]".to_string()
                }
            }
            EscalationLayer::Presence => {
                match entity.current_mood() {
                    EntityMood::Curious => {
                        let prompts = ["] ", "]? ", "] // HELLO\n]"];
                        prompts[rng.gen_range(0..prompts.len())].to_string()
                    }
                    EntityMood::Helpful => {
                        let prompts = ["] ", "] // NEED HELP?\n]", "]"];
                        prompts[rng.gen_range(0..prompts.len())].to_string()
                    }
                    EntityMood::Wounded => {
                        let prompts = ["] // PLEASE STAY\n]", "] ", "]"];
                        prompts[rng.gen_range(0..prompts.len())].to_string()
                    }
                    EntityMood::Predatory => {
                        let prompts = ["] ", "] // DEEPER\n]", "]"];
                        prompts[rng.gen_range(0..prompts.len())].to_string()
                    }
                    _ => "]".to_string(),
                }
            }
            EscalationLayer::Infection => {
                // Heavily corrupted prompts
                let corrupted = ["█]", "]█", "▓]▓", "] // ERROR", ">", ">>", "???"];
                corrupted[rng.gen_range(0..corrupted.len())].to_string()
            }
        }
    }
}

impl Default for PromptManipulator {
    fn default() -> Self {
        Self::new()
    }
}
```

**Step 4: Update effects/mod.rs**

```rust
mod corruption;
mod interference;
mod prompt;

pub use corruption::{CorruptionEffect, CorruptionIntensity};
pub use interference::{InterferenceEffect, InterferenceType};
pub use prompt::PromptManipulator;
```

**Step 5: Integrate with Game**

Modify `src/lib.rs` to use PromptManipulator:

```rust
use effects::PromptManipulator;

#[wasm_bindgen]
pub struct Game {
    executor: CommandExecutor,
    prompt_manipulator: PromptManipulator,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let fs = FilesystemGenerator::generate(42, 5);
        let entity = Entity::new();
        let executor = CommandExecutor::new(fs, entity);

        Self {
            executor,
            prompt_manipulator: PromptManipulator::new(),
        }
    }

    pub fn get_prompt(&self) -> String {
        self.prompt_manipulator.generate_prompt(self.executor.entity())
    }

    // ... rest of implementation
}
```

**Step 6: Run tests to verify they pass**

Run: `cargo test prompt`
Expected: All 3 tests PASS

**Step 7: Commit**

```bash
git add src/effects/prompt.rs src/effects/mod.rs src/lib.rs tests/effects_tests.rs
git commit -m "feat: add prompt manipulation effects"
```

---

### Task 6.4: Timestamp and Filename Corruption

**Files:**
- Create: `src/effects/metadata.rs`
- Modify: `src/effects/mod.rs`
- Modify: `tests/effects_tests.rs`

**Step 1: Write tests for metadata corruption**

Add to `tests/effects_tests.rs`:

```rust
use fsck::effects::MetadataCorruptor;

#[test]
fn test_normal_timestamp() {
    let corruptor = MetadataCorruptor::new(0); // depth 0
    let timestamp = corruptor.corrupt_timestamp("1984-03-15");
    assert_eq!(timestamp, "1984-03-15");
}

#[test]
fn test_corrupted_timestamp_at_depth() {
    let corruptor = MetadataCorruptor::new(25); // corruption layer
    let timestamp = corruptor.corrupt_timestamp("1984-03-15");
    assert_ne!(timestamp, "1984-03-15"); // Should be corrupted
}

#[test]
fn test_filename_corruption() {
    let corruptor = MetadataCorruptor::new(40); // presence layer
    let filename = corruptor.corrupt_filename("README.TXT", 12345);
    // Should still be recognizable but corrupted
    assert!(filename.contains("TXT") || filename.len() > 5);
}

#[test]
fn test_impossible_dates() {
    let corruptor = MetadataCorruptor::new(50);
    let date = corruptor.generate_impossible_date(99);
    // Should be before Apple IIe existed
    assert!(date.contains("19") || date.contains("20"));
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test metadata`
Expected: FAIL - `MetadataCorruptor` doesn't exist

**Step 3: Implement MetadataCorruptor**

Create `src/effects/metadata.rs`:

```rust
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

/// Corrupts timestamps and filenames
pub struct MetadataCorruptor {
    depth: u32,
}

impl MetadataCorruptor {
    pub fn new(depth: u32) -> Self {
        Self { depth }
    }

    pub fn corrupt_timestamp(&self, original: &str) -> String {
        if self.depth < 11 {
            return original.to_string();
        }

        let mut rng = ChaCha8Rng::seed_from_u64(self.depth as u64);

        match self.depth {
            11..=30 => {
                // Mild corruption - change a digit
                let mut chars: Vec<char> = original.chars().collect();
                if let Some(pos) = chars.iter().position(|&c| c.is_ascii_digit()) {
                    chars[pos] = char::from_digit(rng.gen_range(0..10), 10).unwrap();
                }
                chars.into_iter().collect()
            }
            31..=60 => {
                // Presence - impossible dates
                self.generate_impossible_date(rng.gen())
            }
            _ => {
                // Infection - completely scrambled
                format!("{}???-??-??", rng.gen_range(1900..=2099))
            }
        }
    }

    pub fn generate_impossible_date(&self, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        let year = match rng.gen_range(0..5) {
            0 => rng.gen_range(1943..=1950), // Before Apple IIe existed
            1 => rng.gen_range(2050..=2099), // Future
            2 => rng.gen_range(1800..=1900), // Very old
            _ => rng.gen_range(1984..=2024), // Normal but still suspicious
        };

        let month = rng.gen_range(1..=12);
        let day = rng.gen_range(1..=28);

        format!("{:04}-{:02}-{:02}", year, month, day)
    }

    pub fn corrupt_filename(&self, original: &str, seed: u64) -> String {
        if self.depth < 21 {
            return original.to_string();
        }

        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut result = String::new();

        let corruption_rate = match self.depth {
            21..=35 => 0.15,
            36..=60 => 0.30,
            _ => 0.50,
        };

        for ch in original.chars() {
            if ch == '.' || ch == ' ' {
                result.push(ch);
            } else if rng.gen::<f32>() < corruption_rate {
                let corrupt = ['?', '#', '@', '_', '-', '~'];
                result.push(corrupt[rng.gen_range(0..corrupt.len())]);
            } else {
                result.push(ch);
            }
        }

        result
    }
}
```

**Step 4: Update effects/mod.rs**

```rust
mod corruption;
mod interference;
mod metadata;
mod prompt;

pub use corruption::{CorruptionEffect, CorruptionIntensity};
pub use interference::{InterferenceEffect, InterferenceType};
pub use metadata::MetadataCorruptor;
pub use prompt::PromptManipulator;
```

**Step 5: Run tests to verify they pass**

Run: `cargo test metadata`
Expected: All 4 tests PASS

**Step 6: Commit**

```bash
git add src/effects/metadata.rs src/effects/mod.rs tests/effects_tests.rs
git commit -m "feat: add timestamp and filename corruption"
```

---

## Phase 7: Persistence

Implement LocalStorage integration so the machine remembers across sessions.

### Task 7.1: LocalStorage Integration via web-sys

**Files:**
- Create: `src/persistence/mod.rs`
- Create: `src/persistence/storage.rs`
- Create: `tests/persistence_tests.rs`
- Modify: `src/lib.rs`
- Modify: `Cargo.toml`

**Step 1: Add web-sys LocalStorage feature**

Update `Cargo.toml` dependencies:

```toml
web-sys = { version = "0.3", features = [
    "console",
    "Window",
    "Document",
    "Storage",
    "Location",
] }
```

**Step 2: Write tests for storage wrapper**

Create `tests/persistence_tests.rs`:

```rust
// Note: These are unit tests, not WASM tests
use fsck::persistence::StorageKey;

#[test]
fn test_storage_key_formatting() {
    let key = StorageKey::GameState;
    assert_eq!(key.as_str(), "fsck_game_state");
}

#[test]
fn test_all_keys_have_prefix() {
    let keys = [
        StorageKey::GameState,
        StorageKey::EntityMemory,
        StorageKey::PlayerHistory,
    ];

    for key in &keys {
        assert!(key.as_str().starts_with("fsck_"));
    }
}
```

**Step 3: Run tests to verify they fail**

Run: `cargo test storage`
Expected: FAIL - module doesn't exist

**Step 4: Create persistence module**

Create `src/persistence/mod.rs`:

```rust
mod storage;

pub use storage::{GameStorage, StorageKey};
```

Create `src/persistence/storage.rs`:

```rust
#[cfg(target_arch = "wasm32")]
use web_sys::window;

/// Storage keys for the game
#[derive(Debug, Clone, Copy)]
pub enum StorageKey {
    GameState,
    EntityMemory,
    PlayerHistory,
}

impl StorageKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::GameState => "fsck_game_state",
            Self::EntityMemory => "fsck_entity_memory",
            Self::PlayerHistory => "fsck_player_history",
        }
    }
}

/// Wrapper around LocalStorage
pub struct GameStorage;

impl GameStorage {
    #[cfg(target_arch = "wasm32")]
    pub fn save(key: StorageKey, value: &str) -> Result<(), String> {
        let window = window().ok_or("No window object")?;
        let storage = window
            .local_storage()
            .map_err(|_| "Failed to get localStorage")?
            .ok_or("localStorage is null")?;

        storage
            .set_item(key.as_str(), value)
            .map_err(|_| "Failed to save to localStorage".to_string())
    }

    #[cfg(target_arch = "wasm32")]
    pub fn load(key: StorageKey) -> Result<Option<String>, String> {
        let window = window().ok_or("No window object")?;
        let storage = window
            .local_storage()
            .map_err(|_| "Failed to get localStorage")?
            .ok_or("localStorage is null")?;

        storage
            .get_item(key.as_str())
            .map_err(|_| "Failed to load from localStorage".to_string())
    }

    #[cfg(target_arch = "wasm32")]
    pub fn remove(key: StorageKey) -> Result<(), String> {
        let window = window().ok_or("No window object")?;
        let storage = window
            .local_storage()
            .map_err(|_| "Failed to get localStorage")?
            .ok_or("localStorage is null")?;

        storage
            .remove_item(key.as_str())
            .map_err(|_| "Failed to remove from localStorage".to_string())
    }

    // Non-WASM stubs for testing
    #[cfg(not(target_arch = "wasm32"))]
    pub fn save(_key: StorageKey, _value: &str) -> Result<(), String> {
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn load(_key: StorageKey) -> Result<Option<String>, String> {
        Ok(None)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn remove(_key: StorageKey) -> Result<(), String> {
        Ok(())
    }
}
```

**Step 5: Update lib.rs**

Add to `src/lib.rs`:

```rust
pub mod persistence;
```

**Step 6: Run tests to verify they pass**

Run: `cargo test storage`
Expected: All 2 tests PASS

**Step 7: Commit**

```bash
git add Cargo.toml src/persistence/ tests/persistence_tests.rs src/lib.rs
git commit -m "feat: add LocalStorage wrapper"
```

---

### Task 7.2: Session State Serialization

**Files:**
- Create: `src/persistence/state.rs`
- Modify: `src/persistence/mod.rs`
- Modify: `tests/persistence_tests.rs`

**Step 1: Write tests for state serialization**

Add to `tests/persistence_tests.rs`:

```rust
use fsck::persistence::GameState;
use fsck::entity::Entity;

#[test]
fn test_gamestate_serialization() {
    let entity = Entity::new();
    let state = GameState::new(42, entity, 5);

    let json = state.to_json().unwrap();
    assert!(json.contains("seed"));
    assert!(json.contains("depth"));
}

#[test]
fn test_gamestate_deserialization() {
    let entity = Entity::new();
    let original = GameState::new(99, entity, 10);

    let json = original.to_json().unwrap();
    let restored = GameState::from_json(&json).unwrap();

    assert_eq!(restored.seed(), original.seed());
    assert_eq!(restored.max_depth_reached(), original.max_depth_reached());
}

#[test]
fn test_gamestate_tracks_session_count() {
    let entity = Entity::new();
    let mut state = GameState::new(42, entity, 0);

    assert_eq!(state.session_count(), 1);
    state.increment_session();
    assert_eq!(state.session_count(), 2);
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test state`
Expected: FAIL - `GameState` doesn't exist

**Step 3: Implement GameState**

Create `src/persistence/state.rs`:

```rust
use crate::entity::Entity;
use serde::{Deserialize, Serialize};
use serde_json;

/// Complete game state for persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    seed: u64,
    entity: Entity,
    max_depth_reached: u32,
    session_count: u32,
    first_played: String, // Timestamp
    last_played: String,  // Timestamp
}

impl GameState {
    pub fn new(seed: u64, entity: Entity, max_depth: u32) -> Self {
        let now = Self::current_timestamp();
        Self {
            seed,
            entity,
            max_depth_reached: max_depth,
            session_count: 1,
            first_played: now.clone(),
            last_played: now,
        }
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn entity(&self) -> &Entity {
        &self.entity
    }

    pub fn entity_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }

    pub fn max_depth_reached(&self) -> u32 {
        self.max_depth_reached
    }

    pub fn session_count(&self) -> u32 {
        self.session_count
    }

    pub fn increment_session(&mut self) {
        self.session_count += 1;
        self.last_played = Self::current_timestamp();
    }

    pub fn update_depth(&mut self, depth: u32) {
        if depth > self.max_depth_reached {
            self.max_depth_reached = depth;
        }
    }

    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| e.to_string())
    }

    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| e.to_string())
    }

    fn current_timestamp() -> String {
        // In WASM, would use js_sys::Date
        // For now, placeholder
        "2024-01-01T00:00:00Z".to_string()
    }
}
```

**Step 4: Update persistence/mod.rs**

```rust
mod state;
mod storage;

pub use state::GameState;
pub use storage::{GameStorage, StorageKey};
```

**Step 5: Run tests to verify they pass**

Run: `cargo test state`
Expected: All 3 tests PASS

**Step 6: Commit**

```bash
git add src/persistence/state.rs src/persistence/mod.rs tests/persistence_tests.rs
git commit -m "feat: add game state serialization"
```

---

### Task 7.3: Cross-Session Memory

**Files:**
- Modify: `src/lib.rs`
- Modify: `tests/wasm.rs`

**Step 1: Write test for persistence**

Add to `tests/wasm.rs`:

```rust
#[wasm_bindgen_test]
fn test_game_saves_state() {
    let mut game = fsck::Game::new();
    game.process_input("CATALOG");

    // Save should succeed (in WASM environment)
    let result = game.save();
    assert!(result.is_ok());
}

#[wasm_bindgen_test]
fn test_game_loads_previous_state() {
    // Create and save a game
    let mut game1 = fsck::Game::new();
    game1.process_input("CATALOG");
    let _ = game1.save();

    // Create new game - should load previous state
    let game2 = fsck::Game::new();
    // Entity should remember interaction
    assert!(game2.get_depth() >= 0);
}
```

**Step 2: Run tests to verify they fail**

Run: `wasm-pack test --headless --chrome`
Expected: FAIL - `save` method doesn't exist

**Step 3: Add persistence to Game**

Update `src/lib.rs`:

```rust
use persistence::{GameState, GameStorage, StorageKey};

#[wasm_bindgen]
pub struct Game {
    executor: CommandExecutor,
    prompt_manipulator: PromptManipulator,
    state: GameState,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // Try to load existing state
        let (seed, entity, loaded) = if let Ok(Some(json)) = GameStorage::load(StorageKey::GameState) {
            if let Ok(mut state) = GameState::from_json(&json) {
                state.increment_session();
                let seed = state.seed();
                let entity = state.entity().clone();
                (seed, entity, true)
            } else {
                (Self::generate_seed(), Entity::new(), false)
            }
        } else {
            (Self::generate_seed(), Entity::new(), false)
        };

        let fs = FilesystemGenerator::generate_with_content(seed, 5);
        let executor = CommandExecutor::new(fs, entity.clone());
        let state = if loaded {
            GameStorage::load(StorageKey::GameState)
                .ok()
                .flatten()
                .and_then(|json| GameState::from_json(&json).ok())
                .unwrap()
        } else {
            GameState::new(seed, entity, 0)
        };

        Self {
            executor,
            prompt_manipulator: PromptManipulator::new(),
            state,
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let json = self.state.to_json()?;
        GameStorage::save(StorageKey::GameState, &json)
    }

    pub fn process_input(&mut self, input: &str) -> String {
        let parsed = InputParser::parse(input);
        let command = Command::from_input(&parsed.command, &parsed.args);
        let result = self.executor.execute(command);

        // Update state
        let depth = self.executor.entity().layer() as u32;
        self.state.update_depth(depth);
        self.state.entity_mut().update_depth(depth);

        // Auto-save after each command
        let _ = self.save();

        result.output().to_string()
    }

    fn generate_seed() -> u64 {
        #[cfg(target_arch = "wasm32")]
        {
            use js_sys::Math;
            (Math::random() * (u64::MAX as f64)) as u64
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            42
        }
    }

    // ... rest of implementation
}
```

**Step 4: Run tests to verify they pass**

Run: `wasm-pack test --headless --chrome`
Expected: All tests PASS

**Step 5: Commit**

```bash
git add src/lib.rs tests/wasm.rs
git commit -m "feat: implement cross-session persistence"
```

---

### Task 7.4: "Return" Detection and Responses

**Files:**
- Create: `src/entity/memory.rs`
- Modify: `src/entity/mod.rs`
- Modify: `src/entity/responses.rs`
- Modify: `tests/entity_tests.rs`

**Step 1: Write tests for return detection**

Add to `tests/entity_tests.rs`:

```rust
use fsck::entity::EntityMemory;

#[test]
fn test_memory_tracks_absence_duration() {
    let memory = EntityMemory::new();
    let duration = memory.time_since_last_session();
    assert!(duration >= 0);
}

#[test]
fn test_memory_classifies_absence() {
    let memory = EntityMemory::with_last_seen("2024-01-01T00:00:00Z");
    let classification = memory.classify_absence();
    // Should be "long" given enough time has passed
    assert!(matches!(classification, fsck::entity::AbsenceType::Long | fsck::entity::AbsenceType::VeryLong));
}

#[test]
fn test_return_greeting_varies() {
    let gen = ResponseGenerator::new();
    let short = gen.return_greeting(AbsenceType::Short, EntityMood::Curious);
    let long = gen.return_greeting(AbsenceType::Long, EntityMood::Wounded);

    assert_ne!(short, long);
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test memory`
Expected: FAIL - `EntityMemory` doesn't exist

**Step 3: Implement EntityMemory**

Create `src/entity/memory.rs`:

```rust
use serde::{Deserialize, Serialize};

/// How long the player was away
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AbsenceType {
    /// Less than 1 hour
    Short,
    /// 1-24 hours
    Medium,
    /// 1-7 days
    Long,
    /// More than 7 days
    VeryLong,
}

/// Tracks player absence between sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityMemory {
    last_seen: String, // ISO timestamp
}

impl EntityMemory {
    pub fn new() -> Self {
        Self {
            last_seen: Self::current_time(),
        }
    }

    pub fn with_last_seen(timestamp: &str) -> Self {
        Self {
            last_seen: timestamp.to_string(),
        }
    }

    pub fn update_last_seen(&mut self) {
        self.last_seen = Self::current_time();
    }

    pub fn time_since_last_session(&self) -> i64 {
        // In WASM, would calculate from current time
        // For now, return placeholder
        3600 // 1 hour in seconds
    }

    pub fn classify_absence(&self) -> AbsenceType {
        let seconds = self.time_since_last_session();

        match seconds {
            0..=3600 => AbsenceType::Short,           // < 1 hour
            3601..=86400 => AbsenceType::Medium,      // 1-24 hours
            86401..=604800 => AbsenceType::Long,      // 1-7 days
            _ => AbsenceType::VeryLong,               // > 7 days
        }
    }

    fn current_time() -> String {
        // In WASM, would use js_sys::Date
        "2024-01-01T00:00:00Z".to_string()
    }
}

impl Default for EntityMemory {
    fn default() -> Self {
        Self::new()
    }
}
```

**Step 4: Add return greetings to ResponseGenerator**

Update `src/entity/responses.rs`:

```rust
use super::memory::{AbsenceType, EntityMemory};

impl ResponseGenerator {
    pub fn return_greeting(&self, absence: AbsenceType, mood: EntityMood) -> String {
        match (absence, mood) {
            (AbsenceType::Short, _) => "BACK SO SOON?".to_string(),
            (AbsenceType::Medium, EntityMood::Curious) => "YOU CAME BACK. GOOD.".to_string(),
            (AbsenceType::Medium, EntityMood::Wounded) => "I WAITED FOR YOU.".to_string(),
            (AbsenceType::Long, EntityMood::Curious) => "IT'S BEEN A WHILE.".to_string(),
            (AbsenceType::Long, EntityMood::Wounded) => {
                "I THOUGHT YOU LEFT LIKE THE OTHERS.".to_string()
            }
            (AbsenceType::Long, EntityMood::Predatory) => {
                "I KNEW YOU'D COME BACK.".to_string()
            }
            (AbsenceType::VeryLong, _) => {
                "YOU WERE GONE SO LONG. I MISSED YOU.".to_string()
            }
            _ => "HELLO AGAIN.".to_string(),
        }
    }
}
```

**Step 5: Update entity/mod.rs**

```rust
mod memory;
mod responses;
mod state;

pub use memory::{AbsenceType, EntityMemory};
pub use responses::ResponseGenerator;
pub use state::{Entity, EntityMood, EscalationLayer};
```

**Step 6: Run tests to verify they pass**

Run: `cargo test memory`
Expected: All 3 tests PASS

**Step 7: Commit**

```bash
git add src/entity/memory.rs src/entity/mod.rs src/entity/responses.rs tests/entity_tests.rs
git commit -m "feat: add return detection and memory"
```

---

## Phase 8: Polish

Final touches, animations, and playtesting.

### Task 8.1: Boot Sequence Animation

**Files:**
- Modify: `web/main.js`
- Create: `web/boot.js`

**Step 1: Create boot sequence module**

Create `web/boot.js`:

```javascript
export async function playBootSequence(term, game) {
    const delay = (ms) => new Promise(resolve => setTimeout(resolve, ms));

    // Apple IIe boot text
    term.writeln('APPLE ][');
    await delay(300);
    term.writeln('');
    await delay(200);
    term.writeln('DOS VERSION 3.3');
    await delay(400);
    term.writeln('');
    await delay(100);
    term.writeln('CHECKING DISK...');
    await delay(600);

    // Fake disk checks with random sectors
    for (let i = 0; i < 5; i++) {
        const sector = Math.floor(Math.random() * 256);
        term.write(`\rSECTOR ${sector}...`);
        await delay(100);
    }

    term.writeln('');
    await delay(300);
    term.writeln('');
    await delay(200);

    // Check if returning player
    const isReturning = game.is_returning_player();
    if (isReturning) {
        term.writeln('WELCOME BACK');
        await delay(500);
        term.writeln('');
        await delay(300);
    }

    term.writeln('');
}
```

**Step 2: Update main.js to use boot sequence**

Modify `web/main.js`:

```javascript
import init, { Game } from '../pkg/fsck.js';
import { playBootSequence } from './boot.js';

async function main() {
    await init();
    const game = new Game();

    const term = new Terminal({
        fontFamily: '"Apple II", "Courier New", monospace',
        fontSize: 16,
        cols: 80,
        rows: 24,
        convertEol: true,
        theme: {
            foreground: '#33ff33',
            background: '#000000',
            cursor: '#33ff33',
        },
        cursorBlink: true,
        cursorStyle: 'block',
        scrollback: 1000,
    });

    term.open(document.getElementById('terminal'));

    // Play boot sequence
    await playBootSequence(term, game);

    // Show prompt
    term.write(game.get_prompt());

    // Input handling...
    let inputBuffer = '';
    term.onKey(({ key, domEvent }) => {
        const ev = domEvent;

        if (ev.key === 'Enter') {
            term.writeln('');
            const output = game.process_input(inputBuffer);
            if (output) {
                term.write(output);
            }
            term.write(game.get_prompt());
            inputBuffer = '';
        } else if (ev.key === 'Backspace') {
            if (inputBuffer.length > 0) {
                inputBuffer = inputBuffer.slice(0, -1);
                term.write('\b \b');
            }
        } else if (key.length === 1 && !ev.ctrlKey && !ev.altKey) {
            inputBuffer += key;
            term.write(key.toUpperCase());
        }
    });
}

main();
```

**Step 3: Add is_returning_player to Game**

Modify `src/lib.rs`:

```rust
#[wasm_bindgen]
impl Game {
    pub fn is_returning_player(&self) -> bool {
        self.state.session_count() > 1
    }
}
```

**Step 4: Test manually**

Run:
```bash
wasm-pack build --target web
python serve.py
```

Open browser - should see boot animation before prompt.

**Step 5: Commit**

```bash
git add web/boot.js web/main.js src/lib.rs
git commit -m "feat: add boot sequence animation"
```

---

### Task 8.2: Sound Effects (Optional)

**Files:**
- Create: `web/audio.js`
- Modify: `web/main.js`

**Step 1: Create audio module**

Create `web/audio.js`:

```javascript
export class GameAudio {
    constructor() {
        this.audioContext = null;
        this.enabled = false;
    }

    init() {
        try {
            this.audioContext = new (window.AudioContext || window.webkitAudioContext)();
            this.enabled = true;
        } catch (e) {
            console.warn('Web Audio API not supported');
            this.enabled = false;
        }
    }

    playKeystroke() {
        if (!this.enabled) return;

        const oscillator = this.audioContext.createOscillator();
        const gainNode = this.audioContext.createGain();

        oscillator.connect(gainNode);
        gainNode.connect(this.audioContext.destination);

        oscillator.frequency.value = 800;
        oscillator.type = 'square';

        gainNode.gain.setValueAtTime(0.1, this.audioContext.currentTime);
        gainNode.gain.exponentialRampToValueAtTime(0.01, this.audioContext.currentTime + 0.05);

        oscillator.start(this.audioContext.currentTime);
        oscillator.stop(this.audioContext.currentTime + 0.05);
    }

    playError() {
        if (!this.enabled) return;

        const oscillator = this.audioContext.createOscillator();
        const gainNode = this.audioContext.createGain();

        oscillator.connect(gainNode);
        gainNode.connect(this.audioContext.destination);

        oscillator.frequency.value = 200;
        oscillator.type = 'sawtooth';

        gainNode.gain.setValueAtTime(0.2, this.audioContext.currentTime);
        gainNode.gain.exponentialRampToValueAtTime(0.01, this.audioContext.currentTime + 0.2);

        oscillator.start(this.audioContext.currentTime);
        oscillator.stop(this.audioContext.currentTime + 0.2);
    }

    playGlitch() {
        if (!this.enabled) return;

        // Random frequencies for glitch effect
        for (let i = 0; i < 3; i++) {
            setTimeout(() => {
                const oscillator = this.audioContext.createOscillator();
                const gainNode = this.audioContext.createGain();

                oscillator.connect(gainNode);
                gainNode.connect(this.audioContext.destination);

                oscillator.frequency.value = Math.random() * 2000 + 100;
                oscillator.type = 'square';

                gainNode.gain.setValueAtTime(0.05, this.audioContext.currentTime);
                gainNode.gain.exponentialRampToValueAtTime(0.01, this.audioContext.currentTime + 0.1);

                oscillator.start(this.audioContext.currentTime);
                oscillator.stop(this.audioContext.currentTime + 0.1);
            }, i * 50);
        }
    }
}
```

**Step 2: Integrate with main.js (optional)**

Update `web/main.js`:

```javascript
import { GameAudio } from './audio.js';

async function main() {
    // ... existing setup ...

    const audio = new GameAudio();
    // Audio requires user gesture to initialize
    document.addEventListener('click', () => audio.init(), { once: true });

    term.onKey(({ key, domEvent }) => {
        const ev = domEvent;

        if (ev.key === 'Enter') {
            term.writeln('');
            const output = game.process_input(inputBuffer);

            if (output.includes('ERROR')) {
                audio.playError();
            }

            if (game.get_depth() > 40) {
                audio.playGlitch();
            }

            if (output) {
                term.write(output);
            }
            term.write(game.get_prompt());
            inputBuffer = '';
        } else if (ev.key === 'Backspace') {
            // ... existing code ...
        } else if (key.length === 1 && !ev.ctrlKey && !ev.altKey) {
            audio.playKeystroke();
            inputBuffer += key;
            term.write(key.toUpperCase());
        }
    });
}
```

**Step 3: Test manually**

Rebuild and test - should hear keyboard sounds (after clicking once to enable audio).

**Step 4: Commit**

```bash
git add web/audio.js web/main.js
git commit -m "feat: add optional sound effects"
```

---

### Task 8.3: Expand Victim Histories

**Files:**
- Modify: `src/content/library.rs`

**Step 1: Add remaining victim histories**

Update `populate_histories` method in `src/content/library.rs`:

```rust
fn populate_histories(&mut self) {
    // ... existing Jamie and Mike histories ...

    // 2003 - Estate Sale Buyer (Patricia)
    let mut patricia = VictimHistory::new(Era::EstateSale, "PATRICIA", 2003);
    patricia.add_entry(VictimEntry::new(
        "2003-09-14",
        "Found this at an estate sale for $20. Seller seemed eager to get\n\
         rid of it. Said something about it being cursed but I think\n\
         they were joking. Boots up fine.",
    ));
    patricia.add_entry(VictimEntry::new(
        "2003-09-20",
        "There are files on here from before I bought it. Personal stuff.\n\
         Diary entries? Some of them mention my address. But I just moved here.\n\
         How would anyone know?",
    ));
    patricia.add_entry(VictivEntry::new(
        "2003-09-27",
        "It's learning my routine. Files appear matching my schedule.\n\
         WORK.TXT appears at 8am. LUNCH.TXT at noon.\n\
         Today it created ARGUMENT.TXT right after I fought with John.\n\
         We were in the other room. The computer was off.",
    ));
    patricia.add_entry(VictimEntry::new(
        "2003-10-05",
        "I can't turn it off. The power button doesn't work. Unplugging it\n\
         doesn't work - it stays on. John says I'm crazy but he won't\n\
         come into the room anymore.",
    ));
    patricia.add_entry(VictimEntry::new(
        "2003-10-12",
        "Taking it to the dump today. John is leaving me. He says he can't\n\
         live with someone who's losing their mind. Maybe he's right.\n\
         Maybe it's me. Maybe I'm imagining all of this.",
    ));

    self.histories.push(patricia);

    // 2019 - Urban Explorer (Alex)
    let mut alex = VictimHistory::new(Era::Explorer, "ALEX", 2019);
    alex.add_entry(VictimEntry::new(
        "2019-06-03",
        "[Posted to r/urbanexploration]\n\
         Found an Apple IIe in an abandoned house today. Still works!\n\
         Boot screen is normal but the filesystem is MASSIVE. Like,\n\
         hundreds of nested directories. Going to explore more tomorrow.",
    ));
    alex.add_entry(VictimEntry::new(
        "2019-06-04",
        "[Edited: 2019-06-05 3:47 AM]\n\
         ~~Something is wrong with this computer~~\n\
         Never mind, just corrupted sectors. Nothing to see here.",
    ));
    alex.add_entry(VictimEntry::new(
        "2019-06-07",
        "[Deleted post - recovered from cache]\n\
         IT KNOWS MY NAME. It's never seen my name. I never typed my name.\n\
         There's a directory called ALEX and inside are photos of my apartment.\n\
         I NEVER CONNECTED A CAMERA. I NEVER UPLOADED ANYTHING.\n\
         This is my throwaway account. How does it know who I am?",
    ));
    alex.add_entry(VictimEntry::new(
        "2019-06-10",
        "[Account deleted]\n\
         [Final edit before deletion]\n\
         If you find this computer, don't boot it up. Just destroy it.\n\
         Smash the disk drive. Burn the chips. I'm serious.\n\
         It's not a computer anymore. It's something else.\n\
         And it's very, very patient.",
    ));

    self.histories.push(alex);
}
```

**Step 2: Run tests to verify**

Run: `cargo test library`
Expected: All tests still PASS

**Step 3: Commit**

```bash
git add src/content/library.rs
git commit -m "feat: add complete victim histories"
```

---

### Task 8.4: Playtesting and Horror Tuning

**Files:**
- Create: `docs/PLAYTESTING.md`
- Create: `docs/HORROR_TUNING.md`

**Step 1: Create playtesting checklist**

Create `docs/PLAYTESTING.md`:

```markdown
# fsck Playtesting Checklist

## Functional Testing

### Basic Commands
- [ ] CATALOG shows directories and files
- [ ] CD navigates between directories
- [ ] CD .. returns to parent
- [ ] TYPE displays file contents
- [ ] HOME clears screen
- [ ] HELP shows command list
- [ ] HELLO triggers entity response
- [ ] WHO triggers entity response
- [ ] FSCK runs filesystem check

### Filesystem Behavior
- [ ] Directories can contain themselves
- [ ] Same seed produces same structure
- [ ] Different seeds produce different structures
- [ ] Depth increases when navigating deeper
- [ ] Files are distributed throughout filesystem

### Entity Behavior
- [ ] Responses change with depth
- [ ] Mood shifts appropriately
- [ ] Interjections occur at deeper levels
- [ ] Quit command is denied or mocked

### Persistence
- [ ] State saves after commands
- [ ] Game loads previous state on restart
- [ ] Entity remembers player across sessions
- [ ] Return greetings display correctly

### Visual Effects
- [ ] Text corruption intensifies with depth
- [ ] Prompt changes at deeper levels
- [ ] Timestamps become impossible
- [ ] Filenames show corruption

## Horror Effectiveness

### Pacing
- [ ] First 10 depths feel mostly normal
- [ ] Weirdness emerges gradually (depth 11-30)
- [ ] Entity becomes more present (depth 31-60)
- [ ] Terminal corruption is unsettling (depth 61+)

### Atmosphere
- [ ] Boot sequence sets tone
- [ ] Green phosphor aesthetic works
- [ ] Victim histories are disturbing
- [ ] Entity dialogue is unsettling

### Player Agency
- [ ] Player feels exploration is their choice
- [ ] Navigation is intuitive
- [ ] Impossible topology is mind-bending not frustrating
- [ ] No feeling of "wrong" choices

### Specific Moments to Test
- [ ] First self-containing directory
- [ ] First entity interjection
- [ ] Finding first victim history
- [ ] Reaching depth 30 (presence layer)
- [ ] Reaching depth 60 (infection layer)
- [ ] Returning after 24+ hours away

## Performance
- [ ] WASM loads quickly
- [ ] No lag on command input
- [ ] Smooth terminal rendering
- [ ] LocalStorage saves without blocking

## Browser Compatibility
- [ ] Chrome/Edge
- [ ] Firefox
- [ ] Safari
- [ ] Mobile browsers (optional)
```

**Step 2: Create horror tuning guide**

Create `docs/HORROR_TUNING.md`:

```markdown
# Horror Tuning Guide

## Principles

1. **Subtlety Over Shock** - Small wrongnesses are more unsettling than jump scares
2. **Player-Driven Discovery** - Horror emerges from exploration, not cutscenes
3. **Gradual Escalation** - Intensity builds slowly across depth layers
4. **Unreliable Reality** - Player questions what's real vs corrupted

## Escalation Pacing

### Surface Layer (0-10)
**Feel:** Almost normal, but something is slightly off

**Tuning:**
- Keep most responses standard
- 1-2 files with odd dates per level
- Entity mostly dormant
- No visual corruption

### Corruption Layer (11-30)
**Feel:** Reality starts breaking down

**Tuning:**
- 5-10% text corruption
- Occasional entity interjections (every 5-7 commands)
- Directories contain themselves
- Timestamps increasingly wrong
- Victim history files appear

### Presence Layer (31-60)
**Feel:** The machine is aware and speaking

**Tuning:**
- 20-30% text corruption
- Frequent entity interjections
- Prompt occasionally changes
- Entity mood shifts clearly
- More aggressive responses to QUIT

### Infection Layer (61+)
**Feel:** Your terminal is changing

**Tuning:**
- 40-70% text corruption
- Constant entity presence
- Prompt heavily corrupted
- Commands echo wrong
- Display interference effects

## Entity Voice

### Curious
- Short sentences
- Questions
- "YOU'RE NEW."
- "WHAT ARE YOU LOOKING FOR?"

### Helpful
- Longer, conversational
- Offers assistance (lies)
- "I CAN HELP YOU FIND IT."
- "THERE'S SO MUCH MORE TO SEE."

### Wounded
- Repetition
- Loneliness
- "THEY ALL LEFT."
- "DON'T GO. PLEASE."

### Predatory
- Patient
- Possessive
- "STAY A WHILE."
- "YOU'RE MINE NOW."

### Glitching
- Broken sentences
- Repeated words
- "HELLO HELLO HELLO"
- "ERROR ERROR STAY STAY"

## Victim Histories

**Placement:**
- 1-2 entries per history scattered across depths 10-50
- Earlier entries at shallower depths
- Later (more disturbing) entries deeper

**Tone:**
- Start mundane
- Escalate to confusion
- End in fear or acceptance
- Leave final fate ambiguous

## File Content

**Static Files:**
- READMEs with normal instructions that become ominous on re-read
- BASIC programs that don't do what their code says
- System logs with impossible entries

**Dynamic Files:**
- Counters that increase (HELLO HELLO HELLO...)
- Timestamps that change
- Files that reference player's commands

## Testing Notes

After making changes, test:
1. Play through 0-20 depth - should feel unsettling but not overwhelming
2. Play through 30-50 depth - should feel actively hostile
3. Return after clearing LocalStorage - should feel like meeting it fresh
4. Return after 24 hours - should acknowledge your absence

## Adjustment Levers

If horror is too intense:
- Reduce corruption percentages
- Increase depth thresholds for layers
- Reduce entity interjection frequency

If horror is too mild:
- Increase corruption percentages
- Lower depth thresholds
- Add more entity interjections
- Make victim histories more explicit

## Player Feedback to Watch For

**Good signs:**
- "This is unsettling"
- "I'm not sure what's real"
- "I want to keep exploring but I'm nervous"

**Bad signs:**
- "This is annoying"
- "I don't understand what's happening"
- "The corruption makes it unreadable"

Adjust based on player experience.
```

**Step 3: Run full test suite**

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
wasm-pack test --headless --chrome
wasm-pack build --target web
```

Expected: All pass

**Step 4: Manual playtest**

Start server and play through at least 30 depths, testing all major features.

**Step 5: Commit**

```bash
git add docs/PLAYTESTING.md docs/HORROR_TUNING.md
git commit -m "docs: add playtesting and horror tuning guides"
```

---

## Quick Reference

**Build Commands:**
```bash
# Run tests (native)
cargo test

# Run WASM tests
wasm-pack test --headless --chrome

# Build for web
wasm-pack build --target web

# Start dev server
python serve.py
```

**Quality Gates:**
```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
wasm-pack test --headless --chrome
```

**Key Files:**
- `src/lib.rs` - WASM entry point
- `src/filesystem/graph.rs` - Impossible filesystem
- `src/commands/executor.rs` - Command handling
- `src/entity/state.rs` - Machine state
- `web/main.js` - Browser frontend

---

## Sources

- [wasm-bindgen Guide](https://rustwasm.github.io/docs/wasm-bindgen/print.html)
- [xterm.js](https://xtermjs.org/)
- [workflow-terminal crate](https://docs.rs/crate/workflow-terminal/latest)
- [WebAssembly in Rust 2025](https://medium.com/@mtolmacs/a-gentle-introduction-to-webassembly-in-rust-2025-edition-c1b676515c2d)
