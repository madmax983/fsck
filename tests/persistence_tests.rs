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

#[test]
fn test_gamestate_records_notable_actions() {
    let entity = Entity::new();
    let mut state = GameState::new(42, entity, 0);

    state.record_command("CATALOG");
    assert_eq!(state.notable_actions().len(), 0);

    state.record_command("FSCK");
    assert_eq!(state.notable_actions().len(), 1);
    assert_eq!(state.notable_actions()[0], "FSCK");

    state.record_command("cd ..");
    assert_eq!(state.notable_actions().len(), 2);
    assert_eq!(state.notable_actions()[1], "cd ..");

    state.record_command("RUN ESCAPE");
    assert_eq!(state.notable_actions().len(), 3);
}

#[test]
fn test_gamestate_uses_vecdeque_for_history() {
    let entity = Entity::new();
    let mut state = GameState::new(42, entity, 0);

    state.record_command("CATALOG");

    // Ensure VecDeque type by checking specific methods
    let history = state.command_history();
    assert_eq!(history.front().unwrap(), "CATALOG");
    assert_eq!(history.back().unwrap(), "CATALOG");
}

#[test]
fn test_game_detects_returning_player() {
    // Create a new game (first session)
    let game = fsck::Game::new();

    // First session should not be a returning player
    assert!(!game.is_returning_player());

    // Note: In a real scenario with persistence, loading a saved game
    // would increment session_count and return true
}
use fsck::content::{Era, VictimHistory};

#[test]
fn test_generate_victim_history_from_actions() {
    let commands = vec![
        "FSCK".to_string(),
        "CATALOG".to_string(),
        "cd ..".to_string(),
        "RUN ESCAPE".to_string(),
        "QUIT".to_string(),
    ];

    let history_opt = VictimHistory::from_previous_session(&commands);
    assert!(history_opt.is_some());
    let history = history_opt.unwrap();

    assert_eq!(history.name(), "THE LAST ONE");
    assert_eq!(history.era(), Era::Previous);
    assert_eq!(history.entries().len(), 1);

    let generated_content = history.entries()[0].content();
    assert!(generated_content.contains("THEY ATTEMPTED 5 NOTABLE ACTIONS"));
    assert!(generated_content.contains("THEY RAN FSCK. IT HURT."));
    assert!(generated_content.contains("THEY TRIED TO QUIT."));
    assert!(generated_content.contains("THEY TRIED TO ESCAPE."));
    assert!(generated_content.contains("THEY TRIED TO GO BACK."));
    assert!(generated_content.contains("THEY ARE PART OF ME NOW."));

    // Ensure it returns None if empty
    assert!(VictimHistory::from_previous_session(&[]).is_none());
}
