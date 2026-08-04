use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Procedurally generates an ASCII inkblot for psychological evaluation.
pub struct RorschachTest;

impl RorschachTest {
    #[must_use]
    pub fn generate_inkblot(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::new();
        let _ = writeln!(output, "INITIATING PSYCHOLOGICAL EVALUATION...");
        let _ = writeln!(
            output,
            "PLEASE RECORD WHAT YOU SEE IN THE FOLLOWING IMAGE:\n"
        );

        let height = 15;
        let width = 20; // Half width, we mirror it
        let chars = ['#', '@', '%', '&', '*', '.', ' ', ' ', ' '];

        for _ in 0..height {
            let mut left_half = String::with_capacity(width);
            for _ in 0..width {
                // Higher depth = denser, more erratic patterns
                let char_idx = match layer {
                    EscalationLayer::Surface => rng.gen_range(4..chars.len()), // mostly empty/light
                    EscalationLayer::Corruption => rng.gen_range(2..chars.len()),
                    EscalationLayer::Presence => rng.gen_range(0..chars.len()),
                    EscalationLayer::Infection => {
                        if rng.gen_bool(0.1) {
                            rng.gen_range(0..chars.len())
                        } else {
                            0 // very dense
                        }
                    }
                };
                left_half.push(chars[char_idx]);
            }

            let mut right_half: String = left_half.chars().rev().collect();

            // Infection layer introduces asymmetry (it breaks the test)
            if matches!(layer, EscalationLayer::Infection) && rng.gen_bool(0.3) {
                let r_chars: Vec<char> = right_half.chars().collect();
                let mut mutated = String::new();
                for c in r_chars {
                    if rng.gen_bool(0.5) {
                        mutated.push('?');
                    } else {
                        mutated.push(c);
                    }
                }
                right_half = mutated;
            }

            let _ = writeln!(output, "{left_half}{right_half}");
        }

        if matches!(
            layer,
            EscalationLayer::Presence | EscalationLayer::Infection
        ) {
            let _ = writeln!(output, "\nIT SEES YOU TOO.");
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_surface_rorschach() {
        let entity = Entity::new();
        let output = RorschachTest::generate_inkblot(&entity, 42);
        assert!(output.contains("INITIATING PSYCHOLOGICAL EVALUATION"));
        assert!(!output.contains("IT SEES YOU TOO"));
    }

    #[test]
    fn test_infection_rorschach() {
        let mut entity = Entity::new();
        entity.add_depth(30);
        let output = RorschachTest::generate_inkblot(&entity, 42);
        assert!(output.contains("IT SEES YOU TOO"));
    }
}
