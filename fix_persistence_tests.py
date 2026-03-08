with open("tests/persistence_tests.rs", "a") as f:
    f.write("""

#[test]
fn test_player_history_saving_and_loading() {
    use fsck::Game;

    // Simulate a first session
    let mut game1 = Game::new();
    game1.process_input("CATALOG");
    game1.process_input("CD GAMES");
    game1.process_input("TYPE PACMAN.BAS");

    // The history should be saved via auto-save
    // We can simulate creating a second session
    let mut game2 = Game::new();

    // Check if the history was passed through correctly
    // Since we don't have direct access to ContentLibrary from Game,
    // we can check if it loaded correctly by checking the gamestate
    // or just checking the storage key directly.
    use fsck::persistence::{GameStorage, StorageKey};
    let stored = GameStorage::load(StorageKey::PlayerHistory).unwrap().unwrap();
    assert!(stored.contains("CATALOG"));
    assert!(stored.contains("CD GAMES"));
    assert!(stored.contains("TYPE PACMAN.BAS"));
}
""")
