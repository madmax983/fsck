// Note: These are unit tests, not WASM tests
use fsck::entity::Entity;
use fsck::persistence::{GameState, StorageKey};

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

#[test]
fn test_gamestate_serialization() {
    let entity = Entity::new();
    let state = GameState::new(42, entity, 5);

    let json = state.to_json().unwrap();
    assert!(json.contains("seed"));
    assert!(json.contains("depth"));
}

#[test]
fn test_gamestate_deserialization() {
    let entity = Entity::new();
    let original = GameState::new(99, entity, 10);

    let json = original.to_json().unwrap();
    let restored = GameState::from_json(&json).unwrap();

    assert_eq!(restored.seed(), original.seed());
    assert_eq!(restored.max_depth_reached(), original.max_depth_reached());
}

#[test]
fn test_gamestate_tracks_session_count() {
    let entity = Entity::new();
    let mut state = GameState::new(42, entity, 0);

    assert_eq!(state.session_count(), 1);
    state.increment_session();
    assert_eq!(state.session_count(), 2);
}
