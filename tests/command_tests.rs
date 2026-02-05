use fsck::commands::{Command, CommandResult};

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
