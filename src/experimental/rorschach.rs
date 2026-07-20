use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

pub struct RorschachGenerator;

impl RorschachGenerator {
    pub fn generate(entity: &Entity, base_seed: u64) -> String {
        let seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        let width = 12;
        let height = 10;

        let mut left_half = vec![vec![' '; width]; height];

        let density = match entity.layer() {
            EscalationLayer::Surface => 0.2,
            EscalationLayer::Corruption => 0.4,
            EscalationLayer::Presence => 0.6,
            EscalationLayer::Infection => 0.8,
        };

        let chars = match entity.layer() {
            EscalationLayer::Surface => &['#', '*', '.', '-', '|'],
            EscalationLayer::Corruption => &['%', '@', '&', '~', '?'],
            EscalationLayer::Presence => &['X', 'x', '+', '=', '/'],
            EscalationLayer::Infection => &['█', '▓', '▒', '░', '▄'],
        };

        for y in 0..height {
            for x in 0..width {
                if rng.gen_bool(density) {
                    left_half[y][x] = *chars.choose(&mut rng).unwrap_or(&'#');
                }
            }
        }

        // Ensure some central connection so it looks like one piece
        for y in 2..height-2 {
            if rng.gen_bool(0.7) {
                left_half[y][width-1] = *chars.choose(&mut rng).unwrap_or(&'#');
            }
        }

        let mut output = String::with_capacity((width * 2 + 1) * height + 100);
        output.push('\n');
        for row in &left_half {
            for &c in row {
                output.push(c);
            }
            for &c in row.iter().rev() {
                output.push(c);
            }
            output.push('\n');
        }

        let prompt = match entity.layer() {
            EscalationLayer::Surface => "\nWHAT DO YOU SEE?\n",
            EscalationLayer::Corruption => "\nDOES IT LOOK LIKE YOU?\n",
            EscalationLayer::Presence => "\nIT SEES YOU TOO.\n",
            EscalationLayer::Infection => "\nTHERE IS NOTHING ELSE.\n",
        };
        output.push_str(prompt);
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rorschach_symmetry() {
        let entity = Entity::new();
        let output = RorschachGenerator::generate(&entity, 12345);
        let lines: Vec<&str> = output.trim().lines().collect();

        // Check the first 10 lines for symmetry
        for line in lines.iter().take(10) {
            let len = line.chars().count();
            let left: String = line.chars().take(len / 2).collect();
            let right: String = line.chars().skip(len / 2).take(len / 2).rev().collect();
            assert_eq!(left, right, "Line is not symmetric");
        }
    }
}
