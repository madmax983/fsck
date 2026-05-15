use crate::entity::Entity;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Complete game state for persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    seed: u64,
    entity: Entity,
    #[serde(default)]
    command_history: VecDeque<String>,
    #[serde(default)]
    notable_actions: VecDeque<String>,
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
            command_history: VecDeque::new(),
            notable_actions: VecDeque::new(),
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
    pub const fn command_history(&self) -> &VecDeque<String> {
        &self.command_history
    }

    #[must_use]
    pub const fn notable_actions(&self) -> &VecDeque<String> {
        &self.notable_actions
    }

    /// Records a command into the game history buffer.
    /// ⚡ Bolt Optimization: Uses `VecDeque` instead of `Vec` for rolling history buffers.
    /// This turns O(n) `.remove(0)` calls into O(1) `.pop_front()` calls, eliminating memory shifts and allocations on every input when buffers are full.
    /// ⚡ Bolt Optimization: Removes unnecessary `.clone()` allocation on every user input.
    /// ⚡ Bolt Optimization: Uses case-insensitive substring search to avoid `cmd.to_uppercase()` heap allocation.
    pub fn record_command(&mut self, cmd: &str) {
        let cmd_string = cmd.to_string();

        let is_notable = cmd
            .as_bytes()
            .windows(4)
            .any(|w| w.eq_ignore_ascii_case(b"FSCK"))
            || cmd
                .as_bytes()
                .windows(4)
                .any(|w| w.eq_ignore_ascii_case(b"QUIT"))
            || cmd
                .as_bytes()
                .windows(10)
                .any(|w| w.eq_ignore_ascii_case(b"RUN ESCAPE"))
            || cmd
                .as_bytes()
                .windows(5)
                .any(|w| w.eq_ignore_ascii_case(b"CD .."))
            || cmd
                .as_bytes()
                .windows(4)
                .any(|w| w.eq_ignore_ascii_case(b"HELP"))
            || cmd
                .as_bytes()
                .windows(5)
                .any(|w| w.eq_ignore_ascii_case(b"HELLO"))
            || cmd
                .as_bytes()
                .windows(3)
                .any(|w| w.eq_ignore_ascii_case(b"WHO"));

        if is_notable {
            self.notable_actions.push_back(cmd_string.clone());
            if self.notable_actions.len() > 20 {
                self.notable_actions.pop_front();
            }
        }

        self.command_history.push_back(cmd_string);
        if self.command_history.len() > 100 {
            self.command_history.pop_front();
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
