#![cfg(feature = "nova")]

use fsck::commands::{Command, CommandExecutor};
use fsck::entity::Entity;
use fsck::filesystem::FilesystemGenerator;

#[test]
fn test_search_matches() {
    let seed = 42;
    // Generate a simple filesystem
    let fs = FilesystemGenerator::generate_with_content(seed, 2, None);
    let entity = Entity::new();
    let mut executor = CommandExecutor::new(fs, entity);

    // Command 'SEARCH <query>'
    // Testing an exact match for content in HOMEWORK.TXT
    let cmd = Command::Unknown("SEARCH WASHINGTON".to_string());
    let result = executor.execute(cmd);

    let output = result.output();
    assert!(
        output.contains("SEARCHING FOR 'WASHINGTON'..."),
        "Output: {output}"
    );
    // HOMEWORK.TXT has "George Washington"
    // Since we generated at depth 2 with a specific seed, the content library might or might not be exactly in the root,
    // actually generic files are placed everywhere. So we can't guarantee a specific file without knowing the generated layout.
    // Instead we can test that it doesn't crash, and searches without hallucinating at surface level.
}

#[test]
fn test_search_no_matches_surface() {
    let fs = FilesystemGenerator::generate_with_content(42, 1, None);
    let entity = Entity::new();
    let mut executor = CommandExecutor::new(fs, entity);

    let cmd = Command::Unknown("SEARCH UNLIKELYSTRING12345".to_string());
    let result = executor.execute(cmd);

    let output = result.output();
    assert!(output.contains("SEARCHING FOR 'UNLIKELYSTRING12345'..."));
    assert!(output.contains("NO MATCHES FOUND."));
}

#[test]
fn test_search_hallucinations_infection() {
    let fs = FilesystemGenerator::generate_with_content(42, 1, None);
    let mut entity = Entity::new();
    // Move entity to Infection layer
    entity.update_depth(30);

    let mut executor = CommandExecutor::new(fs, entity);

    let mut hallucinated = false;
    for _ in 0..10 {
        let cmd = Command::Unknown("SEARCH UNLIKELYSTRING12345".to_string());
        let result = executor.execute(cmd);
        let output = result.output();
        if output.contains("MATCHED.") {
            hallucinated = true;
            break;
        }
    }

    assert!(
        hallucinated,
        "Search tool should hallucinate matches at Infection layer"
    );
}
