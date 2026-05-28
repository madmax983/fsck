use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

pub struct RorschachGenerator;

impl RorschachGenerator {
    #[must_use]
    pub fn generate_inkblot(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::new();
        let _ = writeln!(output, "INITIATING RORSCHACH PROTOCOL...");

        let width = 16;
        let height = 10;
        for _ in 0..height {
            let mut row = String::new();
            for _ in 0..width {
                row.push(Self::get_char(layer, &mut rng));
            }
            let rev_row: String = row.chars().rev().collect();
            let _ = writeln!(output, "{row}{rev_row}");
        }
        output
    }

    fn get_char(layer: EscalationLayer, rng: &mut ChaCha8Rng) -> char {
        if rng.gen_bool(0.3) {
            return ' ';
        }
        match layer {
            EscalationLayer::Surface => if rng.gen_bool(0.5) { '#' } else { '.' },
            EscalationLayer::Corruption => if rng.gen_bool(0.1) { '?' } else { '%' },
            EscalationLayer::Presence => if rng.gen_bool(0.2) { '@' } else { 'X' },
            EscalationLayer::Infection => {
                let chars = ['0', 'O', '@', '█', '▄'];
                chars[rng.gen_range(0..chars.len())]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_rorschach_generator_surface() {
        let entity = Entity::new();
        let output = RorschachGenerator::generate_inkblot(&entity, 42);
        assert!(output.contains("INITIATING RORSCHACH PROTOCOL"));
        assert!(output.contains('#') || output.contains('.'));
    }
}
