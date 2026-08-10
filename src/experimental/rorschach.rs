use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Procedurally generates a text-based Rorschach inkblot test.
pub struct RorschachTest;

impl RorschachTest {
    /// # Panics
    /// Panics if the internal arrays for random character choices are empty.
    #[must_use]
    pub fn generate(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();
        let mut output = String::new();

        let _ = writeln!(output, "GENERATING INKBLOT...");
        let _ = writeln!(output, "WHAT DO YOU SEE?");
        let _ = writeln!(output);

        let rows = rng.gen_range(5..9);
        let cols_half = rng.gen_range(10..15);

        for _ in 0..rows {
            let mut left_half = String::with_capacity(cols_half);
            for _ in 0..cols_half {
                if rng.gen_bool(0.4) {
                    left_half.push(if rng.gen_bool(0.5) { '#' } else { '@' });
                } else if rng.gen_bool(0.2) {
                    left_half.push('.');
                } else {
                    left_half.push(' ');
                }
            }

            let mut right_half: String = left_half.chars().rev().collect();

            // Horror escalation: break symmetry at deep layers
            if matches!(layer, EscalationLayer::Infection) && rng.gen_bool(0.3) {
                let idx = rng.gen_range(0..right_half.len());
                let char_options = ['V', 'O', 'I', 'D', 'E', 'Y', 'E'];
                let new_char = *char_options.choose(&mut rng).unwrap();
                right_half.replace_range(idx..=idx, &new_char.to_string());
            }

            let _ = writeln!(output, "{left_half}{right_half}");
        }

        output
    }
}
