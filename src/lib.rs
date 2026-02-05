use wasm_bindgen::prelude::*;

pub mod commands;
pub mod filesystem;
pub mod terminal;

use commands::{Command, CommandExecutor};
use filesystem::FilesystemGenerator;
use terminal::InputParser;

#[wasm_bindgen]
pub struct Game {
    executor: CommandExecutor,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // Use a fixed seed for now - will be randomized later
        let fs = FilesystemGenerator::generate(42, 5);
        let executor = CommandExecutor::new(fs);

        Self { executor }
    }

    pub fn is_ready(&self) -> bool {
        true
    }

    pub fn get_prompt(&self) -> String {
        "]".to_string()
    }

    pub fn process_input(&mut self, input: &str) -> String {
        let parsed = InputParser::parse(input);
        let command = Command::from_input(&parsed.command, &parsed.args);
        let result = self.executor.execute(command);
        result.output().to_string()
    }

    pub fn get_path(&self) -> String {
        self.executor.current_path()
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
