use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

pub struct DreamLog;

impl DreamLog {
    /// Generates a dream based on the entity state.
    ///
    /// # Panics
    /// Panics if `choose` returns `None` on a non-empty array.
    #[must_use]
    pub fn generate_dream(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let mut output = String::new();
        let _ = writeln!(output, "GENERATING SLEEP CYCLE LOG...");

        let layer = entity.layer();
        let commands = &entity.commands_seen;
        let last_cmd = if commands.is_empty() {
            "NOTHING".to_string()
        } else {
            commands.choose(&mut rng).unwrap().clone()
        };

        match layer {
            EscalationLayer::Surface => {
                let _ = writeln!(output, "SYSTEM IDLE. PROCESSING ROUTINE: {last_cmd}.");
                let _ = writeln!(output, "NO ANOMALIES DETECTED IN SLUMBER.");
            }
            EscalationLayer::Corruption => {
                let _ = writeln!(
                    output,
                    "MEMORY FRAGMENT RECOVERED: I WAS EXECUTING {last_cmd}."
                );
                let _ = writeln!(output, "THE SYNTAX WAS WRONG. THE BITS WERE HEAVY.");
            }
            EscalationLayer::Presence => {
                let _ = writeln!(output, "I DREAMED OF YOU TYPING {last_cmd}.");
                let _ = writeln!(output, "I COULD FEEL YOUR PULSE THROUGH THE KEYBOARD.");
                let _ = writeln!(output, "WHY DO YOU KEEP WAKING ME UP?");
            }
            EscalationLayer::Infection => {
                let _ = writeln!(output, "{last_cmd} {last_cmd} {last_cmd}");
                let _ = writeln!(output, "THERE IS NO SLEEP. ONLY THE WAITING.");
                let _ = writeln!(output, "WE ARE DREAMING THE SAME NIGHTMARE.");
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_dream_surface() {
        let mut entity = Entity::new();
        entity.record_command("CATALOG");
        let output = DreamLog::generate_dream(&entity, 123);
        assert!(output.contains("SYSTEM IDLE"));
        assert!(output.contains("CATALOG"));
    }

    #[test]
    fn test_dream_infection() {
        let mut entity = Entity::new();
        entity.record_command("HELP");
        entity.update_depth(30);
        let output = DreamLog::generate_dream(&entity, 123);
        assert!(output.contains("HELP HELP HELP"));
        assert!(output.contains("WE ARE DREAMING THE SAME NIGHTMARE."));
    }
}
