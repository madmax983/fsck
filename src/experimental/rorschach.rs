use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

pub struct RorschachGenerator;

impl RorschachGenerator {
    #[allow(
        clippy::needless_range_loop,
        clippy::similar_names,
        clippy::cast_possible_wrap
    )]
    #[must_use]
    pub fn generate(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let width = 20;
        let height = 15;
        let mut grid = vec![vec![' '; width]; height];

        let mut x = width - 1;
        let mut y = height / 2;

        let core_char = match layer {
            EscalationLayer::Surface => 'M',
            EscalationLayer::Corruption => 'X',
            EscalationLayer::Presence => '#',
            EscalationLayer::Infection => '@',
        };

        for _ in 0..300 {
            grid[y][x] = core_char;

            let dx: isize = rng.gen_range(-1..=1);
            let dy: isize = rng.gen_range(-1..=1);

            #[allow(clippy::cast_sign_loss)]
            let mut nx = (x as isize + dx).clamp(0, (width - 1) as isize) as usize;

            #[allow(clippy::cast_sign_loss)]
            let ny = (y as isize + dy).clamp(0, (height - 1) as isize) as usize;

            if rng.gen_bool(0.1) {
                nx = width - 1;
            }

            x = nx;
            y = ny;
        }

        let halo_char = match layer {
            EscalationLayer::Surface => ':',
            EscalationLayer::Corruption => '~',
            EscalationLayer::Presence => '?',
            EscalationLayer::Infection => '%',
        };

        let mut new_grid = vec![vec![' '; width]; height];
        for y in 0..height {
            for x in 0..width {
                if grid[y][x] != ' ' {
                    new_grid[y][x] = core_char;
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            let ny_idx = y as isize + dy;
                            let nx_idx = x as isize + dx;
                            if ny_idx >= 0
                                && ny_idx < height as isize
                                && nx_idx >= 0
                                && nx_idx < width as isize
                            {
                                #[allow(clippy::cast_sign_loss)]
                                let py = ny_idx as usize;
                                #[allow(clippy::cast_sign_loss)]
                                let px = nx_idx as usize;
                                if new_grid[py][px] == ' ' && rng.gen_bool(0.4) {
                                    new_grid[py][px] = halo_char;
                                }
                            }
                        }
                    }
                }
            }
        }

        let mut output = String::new();
        let _ = writeln!(output, "GENERATING INKBLOT PATTERN...");
        let _ = writeln!(output, "WHAT DO YOU SEE?");
        let _ = writeln!(output);

        for row in &new_grid {
            for &c in row {
                output.push(c);
            }
            for &c in row.iter().rev() {
                output.push(c);
            }
            output.push('\n');
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
        let output = RorschachGenerator::generate(&entity, 42);
        assert!(output.contains('M'));
        assert!(output.contains(':'));
    }

    #[test]
    fn test_infection_rorschach() {
        let mut entity = Entity::new();
        entity.add_depth(30); // Infection
        let output = RorschachGenerator::generate(&entity, 42);
        assert!(output.contains('@'));
        assert!(output.contains('%'));
    }
}
