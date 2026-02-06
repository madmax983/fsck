use wasm_bindgen::prelude::*;

pub mod commands;
pub mod content;
pub mod effects;
pub mod entity;
pub mod filesystem;
pub mod terminal;

use commands::{Command, CommandExecutor};
use effects::PromptManipulator;
use entity::Entity;
use filesystem::FilesystemGenerator;
use terminal::InputParser;

#[wasm_bindgen]
pub struct Game {
    executor: CommandExecutor,
    prompt_manipulator: PromptManipulator,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let fs = FilesystemGenerator::generate(42, 5);
        let entity = Entity::new();
        let executor = CommandExecutor::new(fs, entity);

        Self {
            executor,
            prompt_manipulator: PromptManipulator::new(),
        }
    }

    pub fn is_ready(&self) -> bool {
        true
    }

    pub fn get_prompt(&self) -> String {
        self.prompt_manipulator
            .generate_prompt(self.executor.entity())
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

    pub fn get_depth(&self) -> u32 {
        self.executor.entity().layer() as u32
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
