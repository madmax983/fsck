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
    is_hidden: bool,
}

impl FileNode {
    pub fn new(name: &str, content: &str) -> Self {
        Self {
            name: name.to_uppercase(),
            content: NodeContent::Static(content.to_string()),
            is_hidden: false,
        }
    }

    pub fn with_dynamic(name: &str, dynamic: DynamicContent) -> Self {
        Self {
            name: name.to_uppercase(),
            content: NodeContent::Dynamic(RefCell::new(dynamic)),
            is_hidden: false,
        }
    }

    /// Create a hidden file — invisible until fsck reveals it
    #[must_use]
    pub fn hidden(name: &str, content: &str) -> Self {
        Self {
            name: name.to_uppercase(),
            content: NodeContent::Static(content.to_string()),
            is_hidden: true,
        }
    }

    #[must_use]
    pub const fn is_hidden(&self) -> bool {
        self.is_hidden
    }

    pub const fn reveal(&mut self) {
        self.is_hidden = false;
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
    is_hidden: bool,
}

impl DirNode {
    pub fn new(name: &str, depth: u32) -> Self {
        Self {
            name: name.to_uppercase(),
            depth,
            files: Vec::new(),
            is_hidden: false,
        }
    }

    #[must_use]
    pub fn new_hidden(name: &str, depth: u32) -> Self {
        Self {
            name: name.to_uppercase(),
            depth,
            files: Vec::new(),
            is_hidden: true,
        }
    }

    #[must_use]
    pub const fn is_hidden(&self) -> bool {
        self.is_hidden
    }

    pub const fn reveal(&mut self) {
        self.is_hidden = false;
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

    /// All files including hidden ones (for internal use)
    pub fn visible_files(&self) -> impl Iterator<Item = &FileNode> {
        self.files.iter().filter(|f| !f.is_hidden())
    }

    /// Reveal all hidden files, returning their names
    pub fn reveal_hidden_files(&mut self) -> Vec<String> {
        let mut revealed = Vec::new();
        for file in &mut self.files {
            if file.is_hidden() {
                revealed.push(file.name().to_string());
                file.reveal();
            }
        }
        revealed
    }

    pub fn add_file(&mut self, file: FileNode) {
        self.files.push(file);
    }
}
