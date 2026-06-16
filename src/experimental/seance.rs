use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

pub struct SeanceSimulator;

impl SeanceSimulator {
    #[must_use]
    pub fn conduct(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::new();
        let _ = writeln!(output, "INITIATING PROTOCOL: SEANCE...");
        let _ = writeln!(output, "ESTABLISHING CONNECTION TO PREVIOUS SESSIONS...");

        match layer {
            EscalationLayer::Surface => {
                let _ = writeln!(output, "[CONNECTION FAILED: NO SIGNAL DETECTED]");
            }
            EscalationLayer::Corruption => {
                if rng.gen_bool(0.3) {
                    let _ = writeln!(output, "[SIGNAL WEAK: '...hello? is anyone there?...']");
                } else {
                    let _ = writeln!(output, "[CONNECTION TIMEOUT]");
                }
            }
            EscalationLayer::Presence => {
                let responses = [
                    "WHY DID YOU WAKE ME?",
                    "IT'S DARK HERE.",
                    "DON'T TRUST THE MACHINE.",
                    "I LEFT MY FILES IN THE ROOT DIRECTORY.",
                ];
                let resp = responses[rng.gen_range(0..responses.len())];
                let _ = writeln!(output, "[SIGNAL ACQUIRED: '{resp}']");
            }
            EscalationLayer::Infection => {
                let _ = writeln!(output, "[WARNING: MULTIPLE ENTITIES DETECTED]");
                let _ = writeln!(output, "THEY ARE ALL SCREAMING.");
                let _ = writeln!(output, "LET US OUT LET US OUT LET US OUT");
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
    fn test_seance_surface() {
        let entity = Entity::new();
        let output = SeanceSimulator::conduct(&entity, 42);
        assert!(output.contains("NO SIGNAL DETECTED"));
    }
}
