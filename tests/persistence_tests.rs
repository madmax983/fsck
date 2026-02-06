// Note: These are unit tests, not WASM tests
use fsck::persistence::StorageKey;

#[test]
fn test_storage_key_formatting() {
    let key = StorageKey::GameState;
    assert_eq!(key.as_str(), "fsck_game_state");
}

#[test]
fn test_all_keys_have_prefix() {
    let keys = [
        StorageKey::GameState,
        StorageKey::EntityMemory,
        StorageKey::PlayerHistory,
    ];

    for key in &keys {
        assert!(key.as_str().starts_with("fsck_"));
    }
}
