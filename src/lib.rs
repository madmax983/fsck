use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
    initialized: bool,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { initialized: true }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
