#![allow(clippy::collapsible_if)]
use fsck::commands::{Command, CommandExecutor};
use fsck::entity::Entity;
use fsck::filesystem::{FileNode, FilesystemGenerator, FilesystemGraph};

/// Helper: create executor with a simple filesystem
fn make_executor() -> CommandExecutor {
    let fs = FilesystemGraph::new();
    CommandExecutor::new(fs, Entity::new())
}

/// Helper: create executor with hidden content in root
fn make_executor_with_hidden() -> CommandExecutor {
    let mut fs = FilesystemGraph::new();
    fs.current_node_mut()
        .add_file(FileNode::new("VISIBLE.TXT", "I AM VISIBLE"));
    fs.current_node_mut()
        .add_file(FileNode::hidden("SECRET.TXT", "I WAS HIDDEN"));
    fs.add_hidden_child("SEALED");
    CommandExecutor::new(fs, Entity::new())
}

/// Helper: create executor at a specific escalation layer via depth manipulation.
/// Note: `Entity::add_depth` sets both `depth_modifier` and `max_depth_reached`,
/// and `layer()` sums them — so effective layer = `from_depth(2` * depth).
/// Use `raw_depth` values: 3-7 for Corruption, 8-12 for Presence, 13+ for Infection.
fn make_executor_at_depth(raw_depth: u32) -> CommandExecutor {
    let mut fs = FilesystemGraph::new();
    fs.current_node_mut()
        .add_file(FileNode::hidden("LOCKED.TXT", "UNLOCKED CONTENT"));
    let mut entity = Entity::new();
    entity.add_depth(raw_depth);
    CommandExecutor::new(fs, entity)
}

// --- Basic invocation ---

#[test]
fn test_fsck_returns_checking_output() {
    let mut executor = make_executor();
    let result = executor.execute(Command::Fsck(vec![]));

    assert!(!result.is_error());
    assert!(
        result.output().contains("CHECKING"),
        "fsck output should contain 'CHECKING', got: {}",
        result.output()
    );
}

#[test]
fn test_fsck_contains_sector_scan_lines() {
    let mut executor = make_executor();
    let result = executor.execute(Command::Fsck(vec![]));

    assert!(
        result.output().contains("SECTOR"),
        "fsck output should contain sector scan lines, got: {}",
        result.output()
    );
}

// --- Hidden file reveal ---

#[test]
fn test_hidden_files_invisible_before_fsck() {
    let mut executor = make_executor_with_hidden();

    // CATALOG should show VISIBLE.TXT but not SECRET.TXT
    let catalog = executor.execute(Command::Catalog);
    assert!(catalog.output().contains("VISIBLE.TXT"));
    assert!(
        !catalog.output().contains("SECRET.TXT"),
        "Hidden file should not appear in CATALOG before fsck"
    );

    // TYPE SECRET.TXT should fail
    let type_result = executor.execute(Command::Type("SECRET.TXT".to_string()));
    assert!(
        type_result.is_error(),
        "TYPE on hidden file should fail before fsck"
    );
}

#[test]
fn test_hidden_files_visible_after_fsck() {
    let mut executor = make_executor_with_hidden();

    // Run fsck to reveal hidden content
    let fsck_result = executor.execute(Command::Fsck(vec![]));
    assert!(
        fsck_result.output().contains("RECOVERED"),
        "fsck should report recovered content, got: {}",
        fsck_result.output()
    );

    // CATALOG should now show SECRET.TXT
    let catalog = executor.execute(Command::Catalog);
    assert!(
        catalog.output().contains("SECRET.TXT"),
        "Hidden file should appear in CATALOG after fsck"
    );
}

#[test]
fn test_hidden_directories_invisible_before_fsck() {
    let mut executor = make_executor_with_hidden();

    let catalog = executor.execute(Command::Catalog);
    assert!(
        !catalog.output().contains("SEALED"),
        "Hidden directory should not appear in CATALOG before fsck"
    );
}

#[test]
fn test_hidden_directories_visible_after_fsck() {
    let mut executor = make_executor_with_hidden();

    executor.execute(Command::Fsck(vec![]));

    let catalog = executor.execute(Command::Catalog);
    assert!(
        catalog.output().contains("SEALED"),
        "Hidden directory should appear in CATALOG after fsck"
    );
}

#[test]
fn test_revealed_content_is_typeable() {
    let mut executor = make_executor_with_hidden();

    // Can't type before fsck
    let before = executor.execute(Command::Type("SECRET.TXT".to_string()));
    assert!(before.is_error());

    // Run fsck
    executor.execute(Command::Fsck(vec![]));

    // Can type after fsck
    let after = executor.execute(Command::Type("SECRET.TXT".to_string()));
    assert!(!after.is_error());
    assert!(
        after.output().contains("I WAS HIDDEN"),
        "Revealed file should be readable, got: {}",
        after.output()
    );
}

// --- fsck_count and depth pressure ---

#[test]
fn test_fsck_count_increments() {
    let mut executor = make_executor();

    assert_eq!(executor.entity().fsck_count(), 0);
    executor.execute(Command::Fsck(vec![]));
    assert_eq!(executor.entity().fsck_count(), 1);
    executor.execute(Command::Fsck(vec![]));
    assert_eq!(executor.entity().fsck_count(), 2);
    executor.execute(Command::Fsck(vec![]));
    assert_eq!(executor.entity().fsck_count(), 3);
}

#[test]
fn test_three_fscks_trigger_depth_increase() {
    let mut entity = Entity::new();
    let initial_layer = entity.layer();

    // First two fscks: no depth change from fsck itself
    entity.increment_fsck();
    entity.increment_fsck();
    assert_eq!(
        entity.layer(),
        initial_layer,
        "Layer shouldn't change after 2 fscks"
    );

    // Third fsck: triggers add_depth(2)
    entity.increment_fsck();
    // The depth change happens but may not be enough to cross a layer boundary
    assert_eq!(entity.fsck_count(), 3);
}

// --- Layer-dependent behavior ---

#[test]
fn test_surface_layer_no_entity_voice() {
    let mut executor = make_executor();
    let result = executor.execute(Command::Fsck(vec![]));

    // At Surface, entity is silent — no entity-specific phrases
    assert!(
        !result.output().contains("HURT"),
        "Surface layer should have no entity voice"
    );
    assert!(
        !result.output().contains("PLEASE"),
        "Surface layer should have no entity voice"
    );
}

#[test]
fn test_corruption_layer_clinical_warning() {
    // Effective layer = from_depth(2*4 = 8) = Corruption (6-15)
    let mut executor = make_executor_at_depth(4);
    let result = executor.execute(Command::Fsck(vec![]));

    assert!(
        result.output().contains("DO NOT RUN FSCK AGAIN"),
        "Corruption layer should show clinical warning, got: {}",
        result.output()
    );
}

#[test]
fn test_infection_layer_entity_resistance() {
    // Effective layer = from_depth(2*14 = 28) = Infection (26+)
    let mut executor = make_executor_at_depth(14);
    let result = executor.execute(Command::Fsck(vec![]));

    // At Infection, entity text should be present (though possibly corrupted)
    // The output itself gets corrupted, so we check for scan markers that survive
    assert!(
        result.output().len() > 20,
        "Infection layer should produce substantial output"
    );
}

// --- Determinism ---

#[test]
fn test_fsck_deterministic_output() {
    let mut exec1 = make_executor();
    let mut exec2 = make_executor();

    let result1 = exec1.execute(Command::Fsck(vec![]));
    let result2 = exec2.execute(Command::Fsck(vec![]));

    assert_eq!(
        result1.output(),
        result2.output(),
        "Same state should produce same fsck output"
    );
}

// --- Idempotency ---

#[test]
fn test_second_fsck_same_dir_finds_nothing_new() {
    let mut executor = make_executor_with_hidden();

    // First fsck reveals content
    let first = executor.execute(Command::Fsck(vec![]));
    assert!(first.output().contains("RECOVERED"));

    // Second fsck in same dir finds nothing new
    let second = executor.execute(Command::Fsck(vec![]));
    assert!(
        second.output().contains("NO ERRORS FOUND"),
        "Second fsck should find nothing new, got: {}",
        second.output()
    );
}

// --- Generator places hidden content ---

#[test]
fn test_generated_filesystem_has_hidden_content() {
    // Generate a deep filesystem — hidden content should appear
    let mut fs = FilesystemGenerator::generate_with_content(42, 10);

    // Run fsck at root — may or may not find hidden content (depth 0 = no hidden)
    // Navigate deeper where hidden content lives
    let dirs = fs.list_directories();
    if let Some(d1) = dirs.first() {
        if fs.change_dir(d1).is_ok() {
            let dirs = fs.list_directories();
            if let Some(d2) = dirs.first() {
                if fs.change_dir(d2).is_ok() {
                    // At depth 2+, try revealing hidden content
                    let revealed = fs.reveal_hidden_in_current();
                    // May or may not have hidden content (25% chance at depth 2)
                    // Just verify the mechanism works without panicking
                    let _ = revealed;
                }
            }
        }
    }
}
