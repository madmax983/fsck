use serde::{Deserialize, Serialize};

/// The four layers of horror escalation (tightened for better pacing)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EscalationLayer {
    /// Depth 0-5: Almost normal
    Surface,
    /// Depth 6-15: Things stop making sense
    Corruption,
    /// Depth 16-25: It speaks directly
    Presence,
    /// Depth 26+: Your terminal changes
    Infection,
}

impl EscalationLayer {
    pub fn from_depth(depth: u32) -> Self {
        match depth {
            0..=5 => Self::Surface,
            6..=15 => Self::Corruption,
            16..=25 => Self::Presence,
            _ => Self::Infection,
        }
    }
}

/// The machine's current emotional state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityMood {
    /// Quiet, watching
    Dormant,
    /// Interested in the new visitor
    Curious,
    /// Offering assistance (lies)
    Helpful,
    /// Showing pain of isolation
    Wounded,
    /// Hungry, patient
    Predatory,
    /// Breaking down, mask slipping
    Glitching,
}

/// The sentient machine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    current_depth: u32,
    max_depth_reached: u32,
    depth_modifier: u32, // Extra depth from reading certain files
    interaction_count: u32,
    commands_seen: Vec<String>,
    mood: EntityMood,
    #[serde(default)]
    fsck_count: u32,
}

impl Entity {
    pub fn new() -> Self {
        Self {
            current_depth: 0,
            max_depth_reached: 0,
            depth_modifier: 0,
            interaction_count: 0,
            commands_seen: Vec::new(),
            mood: EntityMood::Dormant,
            fsck_count: 0,
        }
    }

    pub fn layer(&self) -> EscalationLayer {
        EscalationLayer::from_depth(self.max_depth_reached + self.depth_modifier)
    }

    /// Increase depth modifier (from reading special files)
    pub fn add_depth(&mut self, amount: u32) {
        self.depth_modifier += amount;
        // Treat as reaching new depth for mood updates
        let effective_depth = self.max_depth_reached + self.depth_modifier;
        if effective_depth > self.max_depth_reached {
            self.max_depth_reached = effective_depth;
            self.update_mood();
        }
    }

    pub fn current_mood(&self) -> EntityMood {
        self.mood
    }

    pub fn interaction_count(&self) -> u32 {
        self.interaction_count
    }

    pub fn update_depth(&mut self, depth: u32) {
        self.current_depth = depth;
        if depth > self.max_depth_reached {
            self.max_depth_reached = depth;
            self.update_mood();
        }
    }

    pub fn record_interaction(&mut self) {
        self.interaction_count += 1;
    }

    pub fn record_command(&mut self, command: &str) {
        self.commands_seen.push(command.to_string());
        self.record_interaction();
    }

    #[must_use]
    pub const fn fsck_count(&self) -> u32 {
        self.fsck_count
    }

    /// Record an fsck invocation. Every 3rd use adds depth pressure.
    pub fn increment_fsck(&mut self) {
        self.fsck_count += 1;
        if self.fsck_count.is_multiple_of(3) {
            self.add_depth(2);
        }
    }

    #[allow(clippy::missing_const_for_fn)]
    fn update_mood(&mut self) {
        self.mood = match self.layer() {
            EscalationLayer::Surface => {
                if self.interaction_count < 5 {
                    EntityMood::Dormant
                } else {
                    EntityMood::Curious
                }
            }
            EscalationLayer::Corruption => EntityMood::Curious,
            EscalationLayer::Presence => {
                if self.interaction_count % 3 == 0 {
                    EntityMood::Wounded
                } else {
                    EntityMood::Helpful
                }
            }
            EscalationLayer::Infection => {
                if self.interaction_count % 2 == 0 {
                    EntityMood::Predatory
                } else {
                    EntityMood::Glitching
                }
            }
        };
    }
}

impl Default for Entity {
    fn default() -> Self {
        Self::new()
    }
}
