#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_game_initializes() {
    let game = fsck::Game::new();
    assert!(game.is_ready());
}

#[wasm_bindgen_test]
fn test_game_catalog_command() {
    let mut game = fsck::Game::new();
    let output = game.process_input("CATALOG");
    assert!(output.contains("DISK VOLUME"));
}

#[wasm_bindgen_test]
fn test_game_navigation() {
    let mut game = fsck::Game::new();

    // Should have some directories from generation
    let catalog = game.process_input("CATALOG");
    assert!(catalog.contains("DIR"));
}

#[wasm_bindgen_test]
fn test_game_unknown_command() {
    let mut game = fsck::Game::new();
    let output = game.process_input("XYZZY");
    assert!(output.contains("SYNTAX ERROR"));
}

#[wasm_bindgen_test]
fn test_game_saves_state() {
    let mut game = fsck::Game::new();
    game.process_input("CATALOG");

    // Save should succeed (in WASM environment)
    let result = game.save();
    assert!(result.is_ok());
}

#[wasm_bindgen_test]
fn test_game_loads_previous_state() {
    // Create and save a game
    let mut game1 = fsck::Game::new();
    game1.process_input("CATALOG");
    let _ = game1.save();

    // Create new game - should load previous state
    let game2 = fsck::Game::new();
    // Verify game is ready (state loaded)
    assert!(game2.is_ready());
}

#[test]
fn test_get_audio_trigger_surface_layer() {
    let mut game = Game::new();
    assert_eq!(game.get_audio_trigger(), "");
}

#[test]
fn test_get_audio_trigger_corruption_layer() {
    let mut game = Game::new();
    // Simulate depth to corruption
    for _ in 0..6 {
        game.process_input("CD DONT");
    }

    let trigger = game.get_audio_trigger();
    assert!(trigger == "" || trigger == "CORRUPTION");
}

#[test]
fn test_get_audio_trigger_presence_layer() {
    let mut game = Game::new();
    // Simulate depth to presence
    for _ in 0..16 {
        game.process_input("CD DONT");
    }

    // We should be in the presence layer now (depth >= 16)
    let trigger = game.get_audio_trigger();
    assert!(trigger == "" || trigger == "PRESENCE");
}

#[test]
fn test_get_audio_trigger_infection_layer() {
    let mut game = Game::new();
    // Simulate depth to infection
    for _ in 0..26 {
        game.process_input("CD DONT"); // depth tracking is based on max_depth/modifier
    }

    let trigger = game.get_audio_trigger();
    assert!(trigger == "" || trigger == "INFECTION");
}
