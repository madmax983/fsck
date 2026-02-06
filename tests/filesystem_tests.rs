use fsck::content::DynamicContent;
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
    let file = FileNode::with_dynamic("COUNTER.TXT", DynamicContent::counter("HELLO\n"));
    // Dynamic content is retrieved via content_dynamic()
    assert!(matches!(file.content_type(), NodeContent::Dynamic(_)));
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

/// Helper function to recursively search for files matching a predicate
fn find_files_recursive<F>(
    fs: &mut FilesystemGraph,
    predicate: &F,
    found: &mut Vec<(String, String)>,
) where
    F: Fn(&FileNode) -> bool,
{
    find_files_recursive_impl(fs, predicate, found, 0);
}

/// Implementation with depth limit to prevent infinite recursion in paradox directories
fn find_files_recursive_impl<F>(
    fs: &mut FilesystemGraph,
    predicate: &F,
    found: &mut Vec<(String, String)>,
    depth: u32,
) where
    F: Fn(&FileNode) -> bool,
{
    // Limit recursion depth to prevent stack overflow in paradox directories
    const MAX_DEPTH: u32 = 20;
    if depth >= MAX_DEPTH {
        return;
    }

    // Check files in current directory
    for file in fs.current_node().files() {
        if predicate(file) {
            let path = format!("{}/{}", fs.current_path(), file.name());
            let content = file.content().to_string();
            found.push((path, content));
        }
    }

    // Recursively check subdirectories
    let dirs = fs.list_directories();
    for dir in dirs {
        if fs.change_dir(&dir).is_ok() {
            find_files_recursive_impl(fs, predicate, found, depth + 1);
            let _ = fs.change_dir("..");
        }
    }
}

#[test]
fn test_generated_filesystem_has_victim_files() {
    let mut fs = FilesystemGenerator::generate_with_content(42, 8);

    // Search for victim history files (.LOG extension)
    let mut victim_files = Vec::new();
    find_files_recursive(
        &mut fs,
        &|f: &FileNode| f.name().ends_with(".LOG"),
        &mut victim_files,
    );

    // Should find at least one victim file
    assert!(
        !victim_files.is_empty(),
        "Expected to find victim history files (.LOG) in generated filesystem"
    );

    // Verify victim files have expected content format (date + content)
    for (path, content) in &victim_files {
        assert!(
            content.contains("-") && content.len() > 20,
            "Victim file {} should contain date and narrative content, got: {}",
            path,
            content
        );
    }
}

#[test]
fn test_victim_files_appear_at_depth() {
    let mut fs = FilesystemGenerator::generate_with_content(12345, 8);

    // Navigate to depth 3+ where victim files should appear
    let dirs = fs.list_directories();
    if let Some(dir1) = dirs.first() {
        fs.change_dir(dir1).unwrap();
        let dirs = fs.list_directories();
        if let Some(dir2) = dirs.first() {
            fs.change_dir(dir2).unwrap();
            let dirs = fs.list_directories();
            if let Some(dir3) = dirs.first() {
                fs.change_dir(dir3).unwrap();

                // At depth 3+, search for victim files in subtree
                let mut victim_files = Vec::new();
                find_files_recursive(
                    &mut fs,
                    &|f: &FileNode| f.name().ends_with(".LOG"),
                    &mut victim_files,
                );

                // Should find victim files at deeper levels
                // (not guaranteed at every depth due to RNG, but with depth 8 we should find some)
                assert!(
                    !victim_files.is_empty(),
                    "Expected victim files to appear at depth 3+ in generated filesystem"
                );
            }
        }
    }
}

#[test]
fn test_dynamic_files_present() {
    let mut fs = FilesystemGenerator::generate_with_content(99, 5);

    // Search for dynamic files
    let mut dynamic_files = Vec::new();
    find_files_recursive(
        &mut fs,
        &|f: &FileNode| matches!(f.content_type(), NodeContent::Dynamic(_)),
        &mut dynamic_files,
    );

    // Should find at least one dynamic file (30% chance per file, depth 5 = many files)
    assert!(
        !dynamic_files.is_empty(),
        "Expected to find dynamic files in generated filesystem"
    );

    // Verify dynamic files have DYN prefix
    for (path, _) in &dynamic_files {
        assert!(
            path.contains("DYN") && path.ends_with(".TXT"),
            "Dynamic files should be named DYN*.TXT, got: {}",
            path
        );
    }
}

#[test]
fn test_static_files_from_library() {
    let mut fs = FilesystemGenerator::generate_with_content(777, 4);

    // Search for known static files from the library
    let mut static_files = Vec::new();
    find_files_recursive(
        &mut fs,
        &|f: &FileNode| {
            matches!(f.content_type(), NodeContent::Static(_))
                && (f.name() == "README.TXT"
                    || f.name() == "HELLO.BAS"
                    || f.name() == "NOTES.TXT"
                    || f.name() == "SYSTEM.LOG"
                    || f.name() == "AUTOEXEC.BAS")
        },
        &mut static_files,
    );

    // Should find at least some static library files
    assert!(
        !static_files.is_empty(),
        "Expected to find static files from ContentLibrary"
    );
}

#[test]
fn test_content_mix_in_filesystem() {
    let mut fs = FilesystemGenerator::generate_with_content(555, 6);

    let mut static_count = 0;
    let mut dynamic_count = 0;
    let mut victim_count = 0;

    let mut all_files = Vec::new();
    find_files_recursive(&mut fs, &|_| true, &mut all_files);

    for (path, _) in &all_files {
        if path.ends_with(".LOG") {
            victim_count += 1;
        } else if path.contains("DYN") {
            dynamic_count += 1;
        } else {
            static_count += 1;
        }
    }

    // Filesystem should have a mix of all three types
    assert!(static_count > 0, "Should have static files from library");
    assert!(dynamic_count > 0, "Should have dynamic generated files");
    // Victim files may not always appear due to RNG and depth requirements,
    // but with depth 6 and seed 555 we should get some
    assert!(
        victim_count > 0,
        "Should have victim history files at depth 3+"
    );
}

#[test]
fn test_generator_creates_paradox_directories() {
    let mut fs = FilesystemGenerator::generate_with_content(42, 10);

    // Search recursively for paradox-named directories
    let paradox_names = ["VOID", "LOOP", "STRANGE", "DARK", "ERROR"];

    fn search_for_paradox(fs: &mut FilesystemGraph, names: &[&str]) -> bool {
        let dirs = fs.list_directories();
        for dir in &dirs {
            if names.contains(&dir.as_str()) {
                // Check if this directory contains itself (paradox)
                if fs.change_dir(dir).is_ok() {
                    let children = fs.list_directories();
                    if children.contains(dir) {
                        let _ = fs.change_dir("..");
                        return true;
                    }
                    let _ = fs.change_dir("..");
                }
            }
        }

        // Recurse into children
        for dir in dirs {
            if fs.change_dir(&dir).is_ok() {
                if search_for_paradox(fs, names) {
                    let _ = fs.change_dir("..");
                    return true;
                }
                let _ = fs.change_dir("..");
            }
        }
        false
    }

    assert!(
        search_for_paradox(&mut fs, &paradox_names),
        "Generator should create at least one paradox directory (VOID, LOOP, STRANGE, DARK, ERROR)"
    );
}

#[test]
fn test_paradox_enables_infinite_descent() {
    let mut fs = FilesystemGenerator::generate_with_content(999, 10);

    // Find a paradox directory
    let paradox_names = ["VOID", "LOOP", "STRANGE", "DARK", "ERROR"];

    fn find_paradox_path(fs: &mut FilesystemGraph, names: &[&str]) -> Option<Vec<String>> {
        let dirs = fs.list_directories();
        for dir in &dirs {
            if names.contains(&dir.as_str()) {
                if fs.change_dir(dir).is_ok() {
                    let children = fs.list_directories();
                    if children.contains(dir) {
                        let _ = fs.change_dir("..");
                        return Some(vec![dir.clone()]);
                    }
                    let _ = fs.change_dir("..");
                }
            }
        }

        // Recurse
        for dir in dirs {
            if fs.change_dir(&dir).is_ok() {
                if let Some(mut path) = find_paradox_path(fs, names) {
                    let _ = fs.change_dir("..");
                    path.insert(0, dir);
                    return Some(path);
                }
                let _ = fs.change_dir("..");
            }
        }
        None
    }

    let path = find_paradox_path(&mut fs, &paradox_names);
    assert!(
        path.is_some(),
        "Should find at least one paradox directory in generated filesystem"
    );

    if let Some(path) = path {
        // Navigate to the paradox
        for dir in &path {
            fs.change_dir(dir).unwrap();
        }

        let paradox_name = path.last().unwrap();
        let initial_depth = fs.current_depth();

        // Enter the paradox multiple times
        fs.change_dir(paradox_name).unwrap();
        let depth1 = fs.current_depth();
        fs.change_dir(paradox_name).unwrap();
        let depth2 = fs.current_depth();

        assert!(
            depth1 > initial_depth,
            "First paradox descent should increase depth"
        );
        assert!(
            depth2 > depth1,
            "Second paradox descent should increase depth further"
        );
    }
}
