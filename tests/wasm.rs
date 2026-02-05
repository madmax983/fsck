#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_game_initializes() {
    let game = fsck::Game::new();
    assert!(game.is_ready());
}

#[wasm_bindgen_test]
fn test_game_processes_input() {
    let mut game = fsck::Game::new();
    let output = game.process_input("CATALOG");
    assert!(!output.is_empty());
}

#[wasm_bindgen_test]
fn test_game_shows_prompt() {
    let mut game = fsck::Game::new();
    let output = game.get_prompt();
    assert_eq!(output, "]");
}
