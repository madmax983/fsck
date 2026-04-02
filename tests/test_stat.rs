#![cfg(feature = "nova")]

use fsck::commands::{Command, CommandExecutor};
use fsck::entity::Entity;
use fsck::filesystem::{FileNode, FilesystemGraph};

#[test]
fn test_stat_command_found() {
    let mut fs = FilesystemGraph::new();
    fs.current_node_mut()
        .add_file(FileNode::new("TEST.TXT", "Some Content"));
    let mut executor = CommandExecutor::new(fs, Entity::new());

    let result = executor.execute(Command::Unknown("STAT TEST.TXT".to_string()));

    assert!(!result.is_error());
    let output = result.output();
    assert!(output.contains("File: TEST.TXT"));
    assert!(output.contains("Size: 12 bytes"));
    assert!(output.contains("Owner:  SYSTEM"));
}

#[test]
fn test_stat_command_not_found() {
    let fs = FilesystemGraph::new();
    let mut executor = CommandExecutor::new(fs, Entity::new());

    let result = executor.execute(Command::Unknown("STAT MISSING.TXT".to_string()));

    assert!(result.is_error());
    assert!(result.output().contains("FILE NOT FOUND: MISSING.TXT"));
}

#[test]
fn test_stat_command_corruption_depth() {
    let mut fs = FilesystemGraph::new();
    fs.current_node_mut()
        .add_file(FileNode::new("CORRUPT.TXT", "Data"));
    let mut entity = Entity::new();
    entity.add_depth(30); // Into presence/infection layer

    let mut executor = CommandExecutor::new(fs, entity);
    let result = executor.execute(Command::Unknown("STAT CORRUPT.TXT".to_string()));

    assert!(!result.is_error());
    let output = result.output();

    // Ownership changes at higher depth
    assert!(output.contains("Owner:  UNKNOWN"));

    // We expect the original file name to be corrupted or at least we can verify it doesn't just cleanly say "File: CORRUPT.TXT"
    // actually, let's just assert the command runs and provides output. The corruptor's exact output varies by seed and depth.
    assert!(output.contains("Size: 4 bytes"));
}
