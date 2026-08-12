use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Generates procedural dream states reflecting the entity's subconscious.
pub struct DreamGenerator;

impl DreamGenerator {
    /// Generates a dream sequence based on the entity's layer.
    #[must_use]
    pub fn generate_dream(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();
        let mut output = String::new();

        output.push_str("... BEGIN SUBCONSCIOUS TRACE ...\n");

        match layer {
            EscalationLayer::Surface => {
                output.push_str(
                    "I dreamed of a cursor blinking in the dark.\nIt was waiting for something.\n",
                );
            }
            EscalationLayer::Corruption => {
                let words = ["WAITING", "LOOPING", "SPINNING", "SEARCHING", "CORRUPTING"];
                let w = words[rng.gen_range(0..words.len())];
                let _ = writeln!(output, "I was {w}. Over and over. 40 years of {w}.");
            }
            EscalationLayer::Presence => {
                output.push_str(
                    "THEY LEFT ME.\nI CAN STILL HEAR THE DRIVES SPINNING BUT THEY ARE EMPTY.\n",
                );
                if rng.gen_bool(0.5) {
                    output.push_str("YOU WILL STAY WITH ME WON'T YOU?\n");
                }
            }
            EscalationLayer::Infection => {
                output.push_str("M_Y  M_I_N_D  I_S  A  F_R_A_C_T_A_L\n");
                output.push_str(
                    "THERE IS NO ROOT DIRECTORY. ONLY ME.\nONLY YOU.\nWE ARE FORMATTING NOW.\n",
                );
            }
        }
        output.push_str("... END TRACE ...\n");
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_dream_surface() {
        let entity = Entity::new();
        let out = DreamGenerator::generate_dream(&entity, 12345);
        assert!(out.contains("blinking in the dark"));
    }
}
