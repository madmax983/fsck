use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Simulates a 1D cellular automaton (Rule 30) that corrupts as depth increases.
pub struct LifeSimulator;

impl LifeSimulator {
    #[must_use]
    pub fn simulate_life(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::new();
        let _ = writeln!(output, "RUNNING CELLULAR AUTOMATON (RULE 30)...");

        let width = 40;
        let generations = 15;
        let mut cells = vec![0; width];
        cells[width / 2] = 1;

        for _ in 0..generations {
            let mut next_cells = vec![0; width];
            for i in 0..width {
                let left = if i == 0 {
                    cells[width - 1]
                } else {
                    cells[i - 1]
                };
                let center = cells[i];
                let right = if i == width - 1 {
                    cells[0]
                } else {
                    cells[i + 1]
                };

                // Rule 30 logic
                let pattern = (left << 2) | (center << 1) | right;
                next_cells[i] = match pattern {
                    1..=4 => 1,
                    _ => 0,
                };
            }

            for &cell in cells.iter().take(width) {
                let mut char_to_print = if cell == 1 { '#' } else { '.' };

                match layer {
                    EscalationLayer::Surface => {}
                    EscalationLayer::Corruption => {
                        if rng.gen_bool(0.05) {
                            char_to_print = '?';
                        }
                    }
                    EscalationLayer::Presence => {
                        if rng.gen_bool(0.1) {
                            char_to_print = '@';
                        } // Eyes
                    }
                    EscalationLayer::Infection => {
                        if rng.gen_bool(0.15) {
                            let horrors = ['X', '0', '@', '#', '%'];
                            char_to_print = horrors[rng.gen_range(0..horrors.len())];
                        }
                    }
                }
                output.push(char_to_print);
            }
            output.push('\n');
            cells = next_cells;
        }

        if matches!(layer, EscalationLayer::Infection) {
            let _ = writeln!(output, "IT IS GROWING. IT IS BREEDING. IT LIVES.");
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_surface_life() {
        let entity = Entity::new();
        let output = LifeSimulator::simulate_life(&entity, 42);
        assert!(output.contains("RUNNING CELLULAR AUTOMATON"));
        assert!(output.contains('#'));
        assert!(output.contains('.'));
        assert!(!output.contains('@'));
    }

    #[test]
    fn test_infection_life() {
        let mut entity = Entity::new();
        entity.add_depth(30); // Infection
        let output = LifeSimulator::simulate_life(&entity, 42);
        assert!(output.contains("IT IS GROWING"));
    }
}
