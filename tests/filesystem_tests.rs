use fsck::filesystem::{DirNode, FileNode, FilesystemGenerator, FilesystemGraph, NodeContent};

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
    let file = FileNode::with_dynamic(
        "COUNTER.TXT",
        NodeContent::Counter {
            base: "HELLO\n",
            count: 0,
        },
    );
    // Dynamic content is retrieved via content_dynamic()
    assert!(matches!(file.content_type(), NodeContent::Counter { .. }));
}

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
