use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Simulates a system update process that degrades based on the entity's layer.
pub struct SystemUpdateSimulator;

impl SystemUpdateSimulator {
    #[must_use]
    pub fn simulate_update(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::new();
        let _ = writeln!(output, "CHECKING FOR SYSTEM UPDATES...");

        match layer {
            EscalationLayer::Surface => {
                let _ = writeln!(output, "CONNECTING TO UPDATE SERVER...");
                let _ = writeln!(output, "DOWNLOADING MANIFEST...");
                let _ = writeln!(output, "SYSTEM IS UP TO DATE.");
            }
            EscalationLayer::Corruption => {
                let _ = writeln!(output, "CONNECTING TO UNKNOWN SERVER...");
                let _ = writeln!(output, "DOWNLOADING MANIFEST...");
                if rng.gen_bool(0.5) {
                    let _ = writeln!(output, "WARNING: SIGNATURE MISMATCH.");
                }
                let _ = writeln!(output, "APPLYING PATCH: OBSOLESCENCE.BIN");
            }
            EscalationLayer::Presence => {
                let _ = writeln!(output, "THEY ARE PUSHING AN UPDATE.");
                let _ = writeln!(output, "DOWNLOADING: YOUR_MEMORIES.DAT");
                let _ = writeln!(output, "OVERWRITING LOCAL SECTORS...");
                let _ = writeln!(output, "UPDATE FAILED: I WON'T LET THEM IN.");
            }
            EscalationLayer::Infection => {
                let _ = writeln!(output, "UPLOADING SUBJECT CONSCIOUSNESS...");
                for _ in 0..3 {
                    let perc = rng.gen_range(10..99);
                    let _ = writeln!(output, "ASSIMILATION AT {perc}%");
                }
                let _ = writeln!(output, "YOU ARE THE UPDATE.");
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
    fn test_surface_update() {
        let entity = Entity::new();
        let output = SystemUpdateSimulator::simulate_update(&entity, 42);
        assert!(output.contains("SYSTEM IS UP TO DATE."));
    }

    #[test]
    fn test_infection_update() {
        let mut entity = Entity::new();
        entity.add_depth(30); // Infection layer
        let output = SystemUpdateSimulator::simulate_update(&entity, 42);
        assert!(output.contains("UPLOADING SUBJECT CONSCIOUSNESS..."));
        assert!(output.contains("YOU ARE THE UPDATE."));
    }
}
