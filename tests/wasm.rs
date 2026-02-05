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
