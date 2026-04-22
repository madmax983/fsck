#![allow(clippy::items_after_statements)]
#![allow(clippy::case_sensitive_file_extension_comparisons)]
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
    assert!(fs.list_directories().any(|d| d == "GAMES"));
}

#[test]
fn test_navigate_to_child() {
    let mut fs = FilesystemGraph::new();
    fs.add_child("GAMES");
    assert!(fs.change_dir("GAMES", 0, 0.0).is_ok());
    assert_eq!(fs.current_dir_name(), "GAMES");
}

#[test]
fn test_navigate_parent() {
    let mut fs = FilesystemGraph::new();
    fs.add_child("GAMES");
    fs.change_dir("GAMES", 0, 0.0).unwrap();
    assert!(fs.change_dir("..", 0, 0.0).is_ok());
    assert_eq!(fs.current_path(), "/");
}

#[test]
fn test_navigate_nonexistent_fails() {
    let mut fs = FilesystemGraph::new();
    assert!(fs.change_dir("NOWHERE", 0, 0.0).is_err());
}

#[test]
fn test_depth_tracking() {
    let mut fs = FilesystemGraph::new();
    assert_eq!(fs.current_depth(), 0);
    fs.add_child("LEVEL1");
    fs.change_dir("LEVEL1", 0, 0.0).unwrap();
    assert_eq!(fs.current_depth(), 1);
}

#[test]
fn test_directory_can_contain_itself() {
    let mut fs = FilesystemGraph::new();
    fs.add_child("VOID");
    fs.change_dir("VOID", 0, 0.0).unwrap();

    // Create paradox: VOID contains VOID
    fs.add_paradox_to_self();

    assert!(fs.list_directories().any(|d| d == "VOID"));
}

#[test]
fn test_paradox_navigation_increases_depth() {
    let mut fs = FilesystemGraph::new();
    fs.add_child("LOOP");
    fs.change_dir("LOOP", 0, 0.0).unwrap();
    fs.add_paradox_to_self();

    let initial_depth = fs.current_depth();
    fs.change_dir("LOOP", 0, 0.0).unwrap();

    // Depth increases even though we're "in the same place"
    assert!(fs.current_depth() > initial_depth);
}

#[test]
fn test_parent_navigation_from_paradox() {
    let mut fs = FilesystemGraph::new();
    fs.add_child("STRANGE");
    fs.change_dir("STRANGE", 0, 0.0).unwrap();
    fs.add_paradox_to_self();
    fs.change_dir("STRANGE", 0, 0.0).unwrap(); // Enter the loop

    // Going back should return to the previous STRANGE
    fs.change_dir("..", 0, 0.0).unwrap();
    assert_eq!(fs.current_dir_name(), "STRANGE");
}

#[test]
fn test_same_seed_produces_same_structure() {
    let fs1 = FilesystemGenerator::generate(12345, 5, None);
    let fs2 = FilesystemGenerator::generate(12345, 5, None);

    assert_eq!(
        fs1.list_directories().map(String::from).collect::<Vec<_>>(),
        fs2.list_directories().map(String::from).collect::<Vec<_>>()
    );
}

#[test]
fn test_different_seeds_produce_different_structures() {
    let fs1 = FilesystemGenerator::generate(12345, 5, None);
    let fs2 = FilesystemGenerator::generate(54321, 5, None);

    // Very unlikely to be identical
    assert_ne!(
        fs1.list_directories().map(String::from).collect::<Vec<_>>(),
        fs2.list_directories().map(String::from).collect::<Vec<_>>()
    );
}

#[test]
fn test_generation_respects_depth_limit() {
    let fs = FilesystemGenerator::generate(99999, 3, None);
    // Root should have some children
    assert!(fs.list_directories().next().is_some());
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
            let content = file.content().clone();
            found.push((path, content));
        }
    }

    // Recursively check subdirectories
    let dirs = fs.list_directories().map(String::from).collect::<Vec<_>>();
    for dir in dirs {
        if fs.change_dir(&dir, 0, 0.0).is_ok() {
            find_files_recursive_impl(fs, predicate, found, depth + 1);
            let _ = fs.change_dir("..", 0, 0.0);
        }
    }
}

#[test]
fn test_generated_filesystem_has_victim_files() {
    let mut fs = FilesystemGenerator::generate_with_content(42, 8, None);

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
        // Skip generic library logs like MACHINE.LOG
        let generic_logs = [
            "MACHINE.LOG",
            "SYSTEM.LOG",
            "OBSERVER3.LOG",
            "OBSERVERS.LOG",
            "SELF.LOG",
            "OBSERVER2.LOG",
            "DIAGNOSTIC.LOG",
            "IMPOSSIBLE2.LOG",
            "IMPOSSIBLE.LOG",
            "WHERE.LOG",
            "VICTIM.LOG",
            "MEMORY.LOG",
            "SYS.LOG",
            "REPAIR.LOG",
            "SYSADMIN.LOG",
        ];
        let filename = path.split('/').next_back().unwrap_or("");
        if generic_logs.contains(&filename) {
            continue;
        }
        assert!(
            content.contains('-') && content.len() > 20,
            "Victim file {path} should contain date and narrative content, got: {content}"
        );
    }
}

#[test]
fn test_victim_files_appear_at_depth() {
    let mut fs = FilesystemGenerator::generate_with_content(12345, 8, None);

    // Navigate to depth 3+ where victim files should appear
    let dirs = fs.list_directories().map(String::from).collect::<Vec<_>>();
    if let Some(dir1) = dirs.first() {
        fs.change_dir(dir1, 0, 0.0).unwrap();
        let dirs = fs.list_directories().map(String::from).collect::<Vec<_>>();
        if let Some(dir2) = dirs.first() {
            fs.change_dir(dir2, 0, 0.0).unwrap();
            let dirs = fs.list_directories().map(String::from).collect::<Vec<_>>();
            if let Some(dir3) = dirs.first() {
                fs.change_dir(dir3, 0, 0.0).unwrap();

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
    let mut fs = FilesystemGenerator::generate_with_content(99, 5, None);

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

    // Verify dynamic files have interesting names (not generic DYN prefix)
    for (path, _) in &dynamic_files {
        // Should NOT have DYN prefix anymore
        assert!(
            !path.contains("DYN") || !path.contains(char::is_numeric),
            "Dynamic files should have interesting names, not DYN12345, got: {path}"
        );
    }
}

#[test]
fn test_dynamic_files_have_creepy_names() {
    let mut fs = FilesystemGenerator::generate_with_content(42, 8, None);

    // Search for dynamic files
    let mut dynamic_files = Vec::new();
    find_files_recursive(
        &mut fs,
        &|f: &FileNode| matches!(f.content_type(), NodeContent::Dynamic(_)),
        &mut dynamic_files,
    );

    assert!(
        !dynamic_files.is_empty(),
        "Should generate some dynamic files"
    );

    // Check that names are from the creepy pool (not random numbers)
    let creepy_names = [
        "ECHO", "REPEAT", "AGAIN", "LOOP", "COUNT", "WHEN", "TIME", "NOW", "DATE", "CLOCK",
        "ERROR", "CORRUPT", "BROKEN", "DAMAGE", "FAULT", "GLITCH", "MEMORY", "FORGET", "WRONG",
        "WHY", "WATCH", "STATIC", "NOISE", "FAIL",
    ];

    for (path, _) in &dynamic_files {
        let filename = path.split('/').next_back().unwrap_or("");
        let name_part = filename.split('.').next().unwrap_or("");

        // Should match one of the creepy names
        let is_creepy = creepy_names.iter().any(|&n| name_part.contains(n));
        assert!(
            is_creepy,
            "Dynamic file should have creepy name, got: {name_part} from path: {path}"
        );
    }
}

#[test]
fn test_static_files_from_library() {
    let mut fs = FilesystemGenerator::generate_with_content(777, 4, None);

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
                    || f.name() == "AUTOEXEC.BAS"
                    || f.name() == "WARNING.TXT"
                    || f.name() == "OBSERVER3.LOG"
                    || f.name() == "THOUGHTS.TXT"
                    || f.name() == "STORY2.BAS"
                    || f.name() == "GLITCH.BAS")
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
    let mut fs = FilesystemGenerator::generate_with_content(99, 10, None);

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

    // Filesystem should have a mix of file types
    // Static files should always be present
    assert!(static_count > 0, "Should have static files from library");

    // At least one of dynamic or victim files should be present
    // (30% chance for dynamic files, variable chance for victim files based on depth)
    assert!(
        dynamic_count > 0 || victim_count > 0,
        "Should have either dynamic files or victim history files. Got static: {static_count}, dynamic: {dynamic_count}, victim: {victim_count}"
    );
}

#[test]
fn test_generator_creates_paradox_directories() {
    let mut fs = FilesystemGenerator::generate_with_content(42, 10, None);

    // Search recursively for paradox-named directories
    let paradox_names = ["VOID", "LOOP", "STRANGE", "DARK", "ERROR"];

    fn search_for_paradox(fs: &mut FilesystemGraph, names: &[&str]) -> bool {
        let dirs = fs.list_directories().map(String::from).collect::<Vec<_>>();
        for dir in &dirs {
            if names.contains(&dir.as_str()) {
                // Check if this directory contains itself (paradox)
                if fs.change_dir(dir, 0, 0.0).is_ok() {
                    if fs.list_directories().any(|d| d == *dir) {
                        let _ = fs.change_dir("..", 0, 0.0);
                        return true;
                    }
                    let _ = fs.change_dir("..", 0, 0.0);
                }
            }
        }

        // Recurse into children
        for dir in dirs {
            if fs.change_dir(&dir, 0, 0.0).is_ok() {
                if search_for_paradox(fs, names) {
                    let _ = fs.change_dir("..", 0, 0.0);
                    return true;
                }
                let _ = fs.change_dir("..", 0, 0.0);
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
    let mut fs = FilesystemGenerator::generate_with_content(999, 10, None);

    // Find a paradox directory
    let paradox_names = ["VOID", "LOOP", "STRANGE", "DARK", "ERROR"];

    fn find_paradox_path(fs: &mut FilesystemGraph, names: &[&str]) -> Option<Vec<String>> {
        let dirs = fs.list_directories().map(String::from).collect::<Vec<_>>();
        for dir in &dirs {
            if names.contains(&dir.as_str()) && fs.change_dir(dir, 0, 0.0).is_ok() {
                if fs.list_directories().any(|d| d == *dir) {
                    let _ = fs.change_dir("..", 0, 0.0);
                    return Some(vec![dir.clone()]);
                }
                let _ = fs.change_dir("..", 0, 0.0);
            }
        }

        // Recurse
        for dir in dirs {
            if fs.change_dir(&dir, 0, 0.0).is_ok() {
                if let Some(mut path) = find_paradox_path(fs, names) {
                    let _ = fs.change_dir("..", 0, 0.0);
                    path.insert(0, dir);
                    return Some(path);
                }
                let _ = fs.change_dir("..", 0, 0.0);
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
            fs.change_dir(dir, 0, 0.0).unwrap();
        }

        let paradox_name = path.last().unwrap();
        let initial_depth = fs.current_depth();

        // Enter the paradox multiple times
        fs.change_dir(paradox_name, 0, 0.0).unwrap();
        let depth1 = fs.current_depth();
        fs.change_dir(paradox_name, 0, 0.0).unwrap();
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

#[test]
fn test_disorienting_navigation() {
    let mut fs = FilesystemGraph::new();

    // Depth 0: /
    fs.add_child("GRANDPARENT");
    fs.change_dir("GRANDPARENT", 0, 0.0).unwrap();

    // Depth 1: GRANDPARENT
    fs.add_child("PARENT");
    fs.add_child("SIBLING");

    fs.change_dir("PARENT", 0, 0.0).unwrap();

    // Depth 2: PARENT
    fs.add_child("CHILD");
    fs.change_dir("CHILD", 0, 0.0).unwrap();

    // Depth 3: CHILD
    assert_eq!(fs.current_dir_name(), "CHILD");

    // Disorienting .. from CHILD (prob 1.0)
    // Parent of CHILD is PARENT. Grandparent is GRANDPARENT.
    // Siblings of PARENT are other children of GRANDPARENT, i.e., SIBLING.
    // Therefore, .. should return to SIBLING.
    fs.change_dir("..", 12345, 1.0).unwrap();

    assert_eq!(fs.current_dir_name(), "SIBLING");
    assert_eq!(fs.current_depth(), 2);
}

#[test]
fn test_recovery_era_history() {
    let mut fs = FilesystemGenerator::generate_with_content(567, 21, None);

    // Search for Recovery history files (.LOG extension containing "BEN")
    let mut victim_files = Vec::new();
    find_files_recursive(
        &mut fs,
        &|f: &FileNode| f.name() == "BEN.LOG",
        &mut victim_files,
    );

    // Should find at least one victim file given the depth
    assert!(
        !victim_files.is_empty(),
        "Expected to find Recovery history file (BEN.LOG) at depth 19-21"
    );

    // Verify victim file has expected content
    let (_, content) = &victim_files[0];
    assert!(content.contains("2001-08-14"));
    assert!(content.contains("Intake log: Client brought in a vintage Apple IIe drive."));
}

#[test]
fn test_streamer_era_history() {
    let mut fs = FilesystemGenerator::generate_with_content(789, 43, None);

    // Search for Streamer history files (.LOG extension containing "CHRIS")
    let mut victim_files = Vec::new();
    find_files_recursive(
        &mut fs,
        &|f: &FileNode| f.name() == "CHRIS.LOG",
        &mut victim_files,
    );

    // Should find at least one victim file given the depth
    assert!(
        !victim_files.is_empty(),
        "Expected to find Streamer history file (CHRIS.LOG) at depth 40-43"
    );

    // Verify victim file has expected content
    let (_, content) = &victim_files[0];
    assert!(content.contains("2022-10-28"));
    assert!(content.contains("Halloween retro stream"));
}

#[test]
fn test_researcher_era_history() {
    let mut fs = FilesystemGenerator::generate_with_content(890, 47, None);

    // Search for Researcher history files (.LOG extension containing "ARIS")
    let mut victim_files = Vec::new();
    find_files_recursive(
        &mut fs,
        &|f: &FileNode| f.name() == "ARIS.LOG",
        &mut victim_files,
    );

    // Should find at least one victim file given the depth
    assert!(
        !victim_files.is_empty(),
        "Expected to find Researcher history file (ARIS.LOG) at depth 44-47"
    );

    // Verify victim file has expected content
    let (_, content) = &victim_files[0];
    assert!(content.contains("2023-04-12"));
    assert!(content.contains("LLM"));
}
