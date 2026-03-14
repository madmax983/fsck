use crate::entity::Entity;
use serde::{Deserialize, Serialize};

/// Complete game state for persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    seed: u64,
    entity: Entity,
    #[serde(default)]
    command_history: Vec<String>,
    #[serde(default)]
    notable_actions: Vec<String>,
    max_depth_reached: u32,
    session_count: u32,
    first_played: String, // Timestamp
    last_played: String,  // Timestamp
}

impl GameState {
    #[must_use]
    pub fn new(seed: u64, entity: Entity, max_depth: u32) -> Self {
        let now = Self::current_timestamp();
        let first_played = now.clone();
        Self {
            seed,
            entity,
            command_history: Vec::new(),
            notable_actions: Vec::new(),
            max_depth_reached: max_depth,
            session_count: 1,
            first_played,
            last_played: now,
        }
    }

    #[must_use]
    pub const fn seed(&self) -> u64 {
        self.seed
    }

    #[must_use]
    pub const fn entity(&self) -> &Entity {
        &self.entity
    }

    #[must_use]
    pub fn command_history(&self) -> &[String] {
        &self.command_history
    }

    #[must_use]
    pub fn notable_actions(&self) -> &[String] {
        &self.notable_actions
    }

    /// Records a command into the game history buffer.
    /// ⚡ Bolt Optimization: Removes unnecessary `.clone()` allocation on every user input.
    pub fn record_command(&mut self, cmd: &str) {
        let cmd_string = cmd.to_string();

        let cmd_upper = cmd.to_uppercase();
        if cmd_upper.contains("FSCK")
            || cmd_upper.contains("QUIT")
            || cmd_upper.contains("RUN ESCAPE")
            || cmd_upper.contains("CD ..")
            || cmd_upper.contains("HELP")
            || cmd_upper.contains("HELLO")
            || cmd_upper.contains("WHO")
        {
            self.notable_actions.push(cmd_string.clone());
            if self.notable_actions.len() > 20 {
                self.notable_actions.remove(0);
            }
        }

        self.command_history.push(cmd_string);
        if self.command_history.len() > 100 {
            self.command_history.remove(0);
        }
    }

    #[must_use]
    pub const fn entity_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }

    #[must_use]
    pub const fn max_depth_reached(&self) -> u32 {
        self.max_depth_reached
    }

    #[must_use]
    pub const fn session_count(&self) -> u32 {
        self.session_count
    }

    pub fn increment_session(&mut self) {
        self.session_count += 1;
        self.last_played = Self::current_timestamp();
    }

    pub const fn update_depth(&mut self, depth: u32) {
        if depth > self.max_depth_reached {
            self.max_depth_reached = depth;
        }
    }

    /// # Errors
    /// Returns an error if serialization fails
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| e.to_string())
    }

    /// # Errors
    /// Returns an error if deserialization fails
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| e.to_string())
    }

    fn current_timestamp() -> String {
        // In WASM, would use js_sys::Date
        // For now, placeholder
        "2024-01-01T00:00:00Z".to_string()
    }
}
