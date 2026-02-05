use wasm_bindgen::prelude::*;

pub mod filesystem;
pub mod terminal;

use terminal::{InputParser, OutputBuffer};

#[wasm_bindgen]
pub struct Game {
    initialized: bool,
    output: OutputBuffer,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            initialized: true,
            output: OutputBuffer::new(),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn get_prompt(&self) -> String {
        "]".to_string()
    }

    pub fn process_input(&mut self, input: &str) -> String {
        let parsed = InputParser::parse(input);

        // Temporary: echo back the command
        self.output.write(&format!("?SYNTAX ERROR: {}\n", parsed.command));

        self.output.drain()
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
