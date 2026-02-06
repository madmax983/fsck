use crate::content::DynamicContent;
use std::cell::RefCell;

/// Content types for files - some are static, some dynamic
#[derive(Debug, Clone)]
pub enum NodeContent {
    /// Static text content
    Static(String),
    /// Dynamic content that changes on each read (uses RefCell for interior mutability)
    Dynamic(RefCell<DynamicContent>),
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

    pub fn with_dynamic(name: &str, dynamic: DynamicContent) -> Self {
        Self {
            name: name.to_uppercase(),
            content: NodeContent::Dynamic(RefCell::new(dynamic)),
        }
    }

    /// Read file content - generates dynamic content on each read
    pub fn read(&self) -> String {
        match &self.content {
            NodeContent::Static(s) => s.clone(),
            NodeContent::Dynamic(d) => d.borrow_mut().generate(),
        }
    }

    // Legacy method for static content - deprecated, use read() instead
    pub fn content(&self) -> String {
        self.read()
    }

    pub fn name(&self) -> &str {
        &self.name
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
