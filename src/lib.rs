use wasm_bindgen::prelude::*;

pub mod commands;
pub mod content;
pub mod effects;
pub mod entity;
pub mod filesystem;
pub mod persistence;
pub mod terminal;

#[cfg(feature = "nova")]
pub mod experimental;

use commands::{Command, CommandExecutor};
use content::VictimHistory;
use effects::PromptManipulator;
use entity::Entity;
use filesystem::FilesystemGenerator;
use persistence::{GameState, GameStorage, StorageKey};
use terminal::InputParser;

#[allow(clippy::collapsible_if)]
#[wasm_bindgen]
pub struct Game {
    executor: CommandExecutor,
    prompt_manipulator: PromptManipulator,
    state: GameState,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    #[must_use]
    pub fn new() -> Self {
        // Try to load existing state
        let (seed, entity, state) = if let Ok(Some(json)) = GameStorage::load(StorageKey::GameState)
        {
            GameState::from_json(&json).map_or_else(
                |_| {
                    let seed = Self::generate_seed();
                    let entity = Entity::new();
                    let state = GameState::new(seed, entity.clone(), 0);
                    (seed, entity, state)
                },
                |mut state| {
                    state.increment_session();
                    let seed = state.seed();
                    let entity = state.entity().clone();
                    (seed, entity, state)
                },
            )
        } else {
            let seed = Self::generate_seed();
            let entity = Entity::new();
            let state = GameState::new(seed, entity.clone(), 0);
            (seed, entity, state)
        };

        let mut prev_history = None;
        #[allow(clippy::collapsible_if)]
        if let Ok(Some(json)) = GameStorage::load(StorageKey::PlayerHistory) {
            #[allow(clippy::collapsible_if)]
            if let Ok(commands) = serde_json::from_str::<Vec<String>>(&json) {
                prev_history = VictimHistory::from_previous_session(&commands);
            }
        }

        let fs = FilesystemGenerator::generate(seed, 5, prev_history);
        let executor = CommandExecutor::new(fs, entity);

        Self {
            executor,
            prompt_manipulator: PromptManipulator::new(),
            state,
        }
    }

    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn is_ready(&self) -> bool {
        true
    }

    #[must_use]
    pub fn get_prompt(&self) -> String {
        self.prompt_manipulator
            .generate_prompt(self.executor.entity())
            .to_string()
    }

    /// Save the current game state to storage
    ///
    /// # Errors
    /// Returns an error if serialization or storage operations fail
    pub fn save(&self) -> Result<(), String> {
        let json = self.state.to_json()?;
        GameStorage::save(StorageKey::GameState, &json)?;
        let history_json =
            serde_json::to_string(self.state.notable_actions()).map_err(|e| e.to_string())?;
        GameStorage::save(StorageKey::PlayerHistory, &history_json)
    }

    pub fn process_input(&mut self, input: &str) -> String {
        let parsed = InputParser::parse(input);
        let command = Command::from_input(parsed.command, parsed.args);
        let result = self.executor.execute(command);

        // Update state
        let depth = self.executor.entity().layer() as u32;
        self.state.update_depth(depth);
        self.state.entity_mut().clone_from(self.executor.entity());

        self.state.record_command(input);

        // Auto-save after each command
        let _ = self.save();

        result.into_output()
    }

    #[must_use]
    pub fn get_path(&self) -> String {
        self.executor.current_path()
    }

    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn get_depth(&self) -> u32 {
        self.executor.entity().layer() as u32
    }

    /// Returns true if this is not the player's first session
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn is_returning_player(&self) -> bool {
        self.state.session_count() > 1
    }

    #[must_use]
    pub fn get_audio_trigger(&self) -> String {
        use rand::{Rng, SeedableRng};
        use rand_chacha::ChaCha8Rng;

        let depth = self.get_depth();
        if depth < 16 {
            return String::new();
        }

        // Use seed + interaction count for deterministic RNG
        let seed = self
            .state
            .seed()
            .wrapping_add(u64::from(self.executor.entity().interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        if depth >= 26 {
            // Infection layer (20% chance)
            if rng.gen_bool(0.2) {
                return "INFECTION".to_string();
            }
        } else if depth >= 16 {
            // Presence layer (10% chance)
            if rng.gen_bool(0.1) {
                return "PRESENCE".to_string();
            }
        } else if depth >= 6 {
            // Corruption layer (15% chance)
            if rng.gen_bool(0.15) {
                return "CORRUPTION".to_string();
            }
        }
        String::new()
    }

    #[cfg(not(target_arch = "wasm32"))]
    const fn generate_seed() -> u64 {
        42
    }

    #[cfg(target_arch = "wasm32")]
    fn generate_seed() -> u64 {
        use js_sys::Math;
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        {
            (Math::random() * (u64::MAX as f64)) as u64
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
