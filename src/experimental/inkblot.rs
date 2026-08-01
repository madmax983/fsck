use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Generates a procedurally mirrored ASCII inkblot for a psychological test.
pub struct InkblotGenerator;

impl InkblotGenerator {
    #[must_use]
    pub fn generate(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let mut output = String::with_capacity(1024);
        let _ = writeln!(output, "INITIALIZING RORSCHACH PROTOCOL...\n");
        let _ = writeln!(output, "WHAT DO YOU SEE?\n");

        let width = 15;
        let height = 10;

        let layer = entity.layer();
        let density = match layer {
            EscalationLayer::Surface => 0.3,
            EscalationLayer::Corruption => 0.4,
            EscalationLayer::Presence => 0.5,
            EscalationLayer::Infection => 0.7,
        };

        for _ in 0..height {
            let mut left_half = String::with_capacity(width);
            for _ in 0..width {
                if rng.gen_bool(density) {
                    let chars = ['#', '@', '%', '&', '8', 'M', 'W'];
                    left_half.push(chars[rng.gen_range(0..chars.len())]);
                } else {
                    left_half.push(' ');
                }
            }

            // Mirror it
            let mut right_half = left_half.chars().rev().collect::<String>();

            // Deep layers: imperfect mirroring
            if matches!(layer, EscalationLayer::Presence | EscalationLayer::Infection) && rng.gen_bool(0.3) {
                // Glitch the right half
                if let Some(idx) = (0..right_half.len()).choose(&mut rng) {
                    right_half.replace_range(idx..=idx, "?");
                }
            }

            let _ = writeln!(output, "  {left_half}{right_half}");
        }

        if matches!(layer, EscalationLayer::Infection) {
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
    fn test_inkblot_generation() {
        let entity = Entity::new();
        let output = InkblotGenerator::generate(&entity, 42);
        assert!(output.contains("RORSCHACH PROTOCOL"));
        assert!(output.contains("WHAT DO YOU SEE?"));
    }
}
