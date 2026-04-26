#![cfg(feature = "nova")]
use fsck::Game;

#[test]
fn test_tree_command() {
    let mut game = Game::new();
    let output = game.process_input("TREE");
    assert!(output.contains("FILESYSTEM TOPOLOGY MAPPER"));
    assert!(output.contains("ROOT: /"));
}
