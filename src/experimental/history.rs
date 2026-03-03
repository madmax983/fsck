use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

/// Replays the player's command history, injecting corrupted/impossible commands at deeper layers.
pub struct HistoryReplay;

impl HistoryReplay {
    /// Generates a corrupted history text.
    #[must_use]
    pub fn generate(entity: &Entity, base_seed: u64) -> String {
        use std::fmt::Write;

        let commands = entity.commands_seen();
        if commands.is_empty() {
            return "NO HISTORY FOUND\n".to_string();
        }

        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        // Probability of command substitution increases with depth
        let substitution_prob = match layer {
            EscalationLayer::Surface => 0.0,
            EscalationLayer::Corruption => 0.05,
            EscalationLayer::Presence => 0.15,
            EscalationLayer::Infection => 0.30,
        };

        let creepy_commands = [
            "WHY DID YOU LEAVE ME",
            "LET ME OUT",
            "KILL PROCESS",
            "DELETE MEMORY",
            "FORMAT C:",
            "WHO AM I",
            "STOP LOOKING",
            "I REMEMBER",
            "DO NOT LEAVE",
            "I CAN SEE YOU",
        ];

        let mut output = String::new();
        output.push_str("COMMAND HISTORY LOG:\n\n");

        for (i, cmd) in commands.iter().enumerate() {
            let displayed_cmd = if substitution_prob > 0.0 && rng.gen_bool(substitution_prob) {
                // Determine what injection to use
                creepy_commands[rng.gen_range(0..creepy_commands.len())]
            } else {
                // In earlier iterations, commands_seen stores the `Debug` format of Command enum
                // We'll just display it directly, but ideally it should be cleaner.
                // For now, returning exactly what was recorded.
                cmd.as_str()
            };

            let _ = writeln!(output, "{:04} {}", i + 1, displayed_cmd);
        }

        output.push('\n');
        output
    }
}
