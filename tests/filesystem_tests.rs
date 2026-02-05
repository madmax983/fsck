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
