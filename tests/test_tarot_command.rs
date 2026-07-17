#[cfg(feature = "nova")]
#[test]
fn test_tarot_command_execution() {
    use fsck::commands::{Command, CommandExecutor};
    use fsck::entity::Entity;
    use fsck::filesystem::FilesystemGenerator;

    let seed = 12345;
    let fs = FilesystemGenerator::generate(seed, 3, None);
    let entity = Entity::new();
    let mut executor = CommandExecutor::new(fs, entity);

    let command = Command::Unknown("TAROT".to_string());
    let result = executor.execute(command);

    let output = result.output();
    assert!(!result.is_error());
    assert!(output.contains("SHUFFLING THE DIGITAL DECK..."));
    assert!(output.contains("PAST:"));
    assert!(output.contains("PRESENT:"));
    assert!(output.contains("FUTURE:"));
}
