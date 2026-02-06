use serde::{Deserialize, Serialize};

/// The four layers of horror escalation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EscalationLayer {
    /// Depth 0-10: Almost normal
    Surface,
    /// Depth 11-30: Things stop making sense
    Corruption,
    /// Depth 31-60: It speaks directly
    Presence,
    /// Depth 61+: Your terminal changes
    Infection,
}

impl EscalationLayer {
    pub fn from_depth(depth: u32) -> Self {
        match depth {
            0..=10 => Self::Surface,
            11..=30 => Self::Corruption,
            31..=60 => Self::Presence,
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
    interaction_count: u32,
    commands_seen: Vec<String>,
    mood: EntityMood,
}

impl Entity {
    pub fn new() -> Self {
        Self {
            current_depth: 0,
            max_depth_reached: 0,
            interaction_count: 0,
            commands_seen: Vec::new(),
            mood: EntityMood::Dormant,
        }
    }

    pub fn layer(&self) -> EscalationLayer {
        EscalationLayer::from_depth(self.max_depth_reached)
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
