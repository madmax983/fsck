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

## Phase 5-8: Summary

The remaining phases follow the same TDD pattern. Here's the outline:

### Phase 5: Content Library
- Task 5.1: Victim history data structures
- Task 5.2: Static file content library
- Task 5.3: Dynamic content generation (counter files, etc.)
- Task 5.4: Content placement in filesystem

### Phase 6: Escalation Effects
- Task 6.1: Corruption visual effects (text scrambling)
- Task 6.2: Display interference (cursor jumping, echo errors)
- Task 6.3: Prompt manipulation (entity speaking through prompt)
- Task 6.4: Timestamp/filename corruption

### Phase 7: Persistence
- Task 7.1: LocalStorage integration via web-sys
- Task 7.2: Session state serialization
- Task 7.3: Cross-session memory (entity remembers)
- Task 7.4: "Return" detection and responses

### Phase 8: Polish
- Task 8.1: Boot sequence animation
- Task 8.2: Sound effects (optional, Web Audio API)
- Task 8.3: More victim histories
- Task 8.4: Playtesting and horror tuning

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
