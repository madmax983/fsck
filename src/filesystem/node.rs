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
