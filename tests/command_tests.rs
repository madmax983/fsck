use fsck::commands::{Command, CommandExecutor, CommandResult};
use fsck::entity::Entity;
use fsck::filesystem::FilesystemGraph;

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

#[test]
fn test_catalog_lists_directories() {
    let mut fs = FilesystemGraph::new();
    fs.add_child("GAMES");
    fs.add_child("DOCS");

    let mut executor = CommandExecutor::new(fs, Entity::new());
    let result = executor.execute(Command::Catalog);

    assert!(result.output().contains("GAMES"));
    assert!(result.output().contains("DOCS"));
}

#[test]
fn test_cd_changes_directory() {
    let mut fs = FilesystemGraph::new();
    fs.add_child("GAMES");

    let mut executor = CommandExecutor::new(fs, Entity::new());
    let result = executor.execute(Command::ChangeDir("GAMES".to_string()));

    assert!(!result.is_error());
}

#[test]
fn test_cd_nonexistent_fails() {
    let fs = FilesystemGraph::new();
    let mut executor = CommandExecutor::new(fs, Entity::new());
    let result = executor.execute(Command::ChangeDir("NOWHERE".to_string()));

    assert!(result.is_error());
}

#[test]
fn test_type_displays_file() {
    let mut fs = FilesystemGraph::new();
    fs.current_node_mut()
        .add_file(fsck::filesystem::FileNode::new("TEST.TXT", "Hello World"));

    let mut executor = CommandExecutor::new(fs, Entity::new());
    let result = executor.execute(Command::Type("TEST.TXT".to_string()));

    assert!(result.output().contains("Hello World"));
}

#[test]
fn test_type_dynamic_counter_increments() {
    use fsck::content::DynamicContent;
    use fsck::filesystem::FileNode;

    let mut fs = FilesystemGraph::new();
    let counter_file = FileNode::with_dynamic("COUNTER.TXT", DynamicContent::counter("X"));
    fs.current_node_mut().add_file(counter_file);

    let mut executor = CommandExecutor::new(fs, Entity::new());

    // First read should show "X1"
    let result1 = executor.execute(Command::Type("COUNTER.TXT".to_string()));
    assert!(!result1.is_error());
    assert!(
        result1.output().contains("X1"),
        "First read should show X1, got: {}",
        result1.output()
    );

    // Second read should show "XX2"
    let result2 = executor.execute(Command::Type("COUNTER.TXT".to_string()));
    assert!(!result2.is_error());
    assert!(
        result2.output().contains("XX2"),
        "Second read should show XX2, got: {}",
        result2.output()
    );
}

#[test]
fn test_type_dynamic_corrupted_content() {
    use fsck::content::DynamicContent;
    use fsck::filesystem::FileNode;

    let mut fs = FilesystemGraph::new();
    let corrupted_file =
        FileNode::with_dynamic("ERROR.TXT", DynamicContent::corrupted("HELLO WORLD", 0.3));
    fs.current_node_mut().add_file(corrupted_file);

    let mut executor = CommandExecutor::new(fs, Entity::new());

    let result = executor.execute(Command::Type("ERROR.TXT".to_string()));
    assert!(!result.is_error());
    // Should not be the placeholder
    assert!(!result.output().contains("[DYNAMIC]"));
    // Should contain some of the original text (not fully corrupted at 0.3 intensity)
    assert!(
        result.output().contains("HELLO")
            || result.output().contains("WORLD")
            || result.output().len() > 5
    );
}

#[test]
fn test_type_dynamic_timestamp() {
    use fsck::content::DynamicContent;
    use fsck::filesystem::FileNode;

    let mut fs = FilesystemGraph::new();
    let timestamp_file = FileNode::with_dynamic("TIME.TXT", DynamicContent::timestamp());
    fs.current_node_mut().add_file(timestamp_file);

    let mut executor = CommandExecutor::new(fs, Entity::new());

    let result = executor.execute(Command::Type("TIME.TXT".to_string()));
    assert!(!result.is_error());
    // Should show timestamp format, not placeholder
    assert!(!result.output().contains("[DYNAMIC]"));
    assert!(result.output().contains("??") || result.output().contains("2024"));
}

#[test]
fn test_help_surface() {
    let fs = FilesystemGraph::new();
    let mut executor = CommandExecutor::new(fs, Entity::new());

    let result = executor.execute(Command::Help);

    assert!(!result.is_error());
    assert!(result.output().contains("AVAILABLE COMMANDS"));
    assert!(result.output().contains("CATALOG"));
    assert!(!result.output().contains("ESCAPE"));
    assert!(!result.output().contains("REMEMBER"));
}

#[test]
fn test_help_corruption() {
    let fs = FilesystemGraph::new();
    let mut entity = Entity::new();
    // At depth 6-15, it's the Corruption layer. Note: max_depth_reached + depth_modifier determines layer.
    // add_depth(4) gets us to 4 for depth_modifier and 4 for max_depth_reached = 8 total.
    entity.add_depth(4);

    let mut executor = CommandExecutor::new(fs, entity);
    let result = executor.execute(Command::Help);

    assert!(!result.is_error());
    assert!(result.output().contains("AVAILABLE COMMANDS"));
    assert!(result.output().contains("ESCAPE   - ???"));
    assert!(result.output().contains("REMEMBER - ???"));
}

#[test]
fn test_help_presence() {
    let fs = FilesystemGraph::new();
    let mut entity = Entity::new();
    // At depth 16-25, it's the Presence layer. Add 10 => 10 + 10 = 20 total.
    entity.add_depth(10);

    let mut executor = CommandExecutor::new(fs, entity);
    let result = executor.execute(Command::Help);

    assert!(!result.is_error());
    assert!(!result.output().contains("AVAILABLE COMMANDS"));

    // Depending on the mood (likely Curious because of the way interaction count works, but we check for any valid response)
    let output = result.output();
    assert!(
        output.contains("WHAT DO YOU NEED HELP WITH?")
            || output.contains("I CAN HELP YOU FIND IT.")
            || output.contains("I CAN'T HELP YOU. I CAN'T EVEN HELP MYSELF.")
            || output.contains("YOU DON'T NEED HELP. YOU'RE DOING EXACTLY WHAT I WANT.")
            || output.contains("HELP HELP HELP NO NO NO")
            || output.contains("...")
    );
}

#[test]
fn test_help_infection() {
    let fs = FilesystemGraph::new();
    let mut entity = Entity::new();
    // At depth 26+, it's the Infection layer. Add 15 => 15 + 15 = 30 total.
    entity.add_depth(15);

    let mut executor = CommandExecutor::new(fs, entity);
    let result = executor.execute(Command::Help);

    assert!(!result.is_error());
    assert!(!result.output().contains("AVAILABLE COMMANDS"));

    let output = result.output();
    assert!(
        output.contains("THERE IS NO HELP FOR YOU DOWN HERE.")
            || output.contains("NO ONE CAN HELP YOU NOW.")
    );
}
