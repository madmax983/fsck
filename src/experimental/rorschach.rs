use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Procedurally generates a Rorschach inkblot test that degrades into
/// psychological horror as the player descends into the impossible filesystem.
pub struct RorschachTest;

impl RorschachTest {
    #[must_use]
    pub fn generate(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();
        let mut output = String::new();

        let _ = writeln!(output, "--- CLINICAL PSYCHOLOGICAL EVALUATION ---");
        let _ = writeln!(output, "SUBJECT: UNKNOWN");
        let _ = writeln!(output, "TEST 01: RORSCHACH INKBLOT\n");

        match layer {
            EscalationLayer::Surface => Self::generate_blob(&mut output, &mut rng, false),
            EscalationLayer::Corruption => Self::generate_blob(&mut output, &mut rng, true),
            EscalationLayer::Presence => Self::generate_pareidolia(&mut output, &mut rng),
            EscalationLayer::Infection => Self::generate_infection(&mut output, &mut rng),
        }

        let _ = writeln!(
            output,
            "\nPLEASE RECORD WHAT YOU SEE. THERE ARE NO WRONG ANSWERS."
        );
        output
    }

    fn generate_blob(output: &mut String, rng: &mut ChaCha8Rng, is_corrupted: bool) {
        let width = 12;
        let height = 10;

        let chars = if is_corrupted {
            [' ', ' ', '.', '-', '+', 'x', 'X', 'M', 'W', '#', '█']
        } else {
            [' ', ' ', ' ', '░', '▒', '▓', '█', '█', '█', '█', '█']
        };

        // ⚡ Bolt Optimization: Avoid intermediate strings
        for _y in 0..height {
            let mut row_chars = [chars[0]; 12];
            #[allow(clippy::needless_range_loop)]
            #[allow(clippy::cast_precision_loss)]
            for x in 0..width {
                let center_bias = (x as f64 / (width as f64)).powi(2);
                let is_solid = rng.gen_bool(center_bias + 0.1);

                if is_solid {
                    row_chars[x] = chars[rng.gen_range(chars.len() / 2..chars.len())];
                } else {
                    row_chars[x] = chars[rng.gen_range(0..chars.len() / 2)];
                }
            }

            output.push_str("    ");
            for c in &row_chars {
                output.push(*c);
            }
            for c in row_chars.iter().rev() {
                output.push(*c);
            }
            output.push('\n');
        }
    }

    fn generate_pareidolia(output: &mut String, rng: &mut ChaCha8Rng) {
        let patterns = [
            "      ████      ████      \n    ████████  ████████    \n  ████    ██████    ████  \n  ██        ██        ██  \n  ████    ██████    ████  \n    ████████  ████████    \n      ████      ████      ",
            "    ░░▒▒▓▓██████▓▓▒▒░░    \n  ░░▒▒▓▓██      ██▓▓▒▒░░  \n  ░░▒▒▓▓██  ██  ██▓▓▒▒░░  \n  ░░▒▒▓▓██      ██▓▓▒▒░░  \n    ░░▒▒▓▓██████▓▓▒▒░░    ",
            "          ██████          \n        ███░░░░███        \n      ███░░████░░███      \n      ███░░████░░███      \n        ███░░░░███        \n          ██████          ",
        ];

        let chosen = patterns[rng.gen_range(0..patterns.len())];
        let _ = writeln!(output, "{chosen}");
    }

    fn generate_infection(output: &mut String, rng: &mut ChaCha8Rng) {
        let messages = [
            "I   S E E   Y O U",
            "I T   H U R T S",
            "W H Y   A R E   Y O U   L O O K I N G",
            "T H E R E   I S   N O   R E F L E C T I O N",
            "Y O U   A R E   T H E   B L O T",
        ];
        let msg = messages[rng.gen_range(0..messages.len())];

        for _ in 0..5 {
            let _ = writeln!(output, "    {msg}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_surface_rorschach() {
        let entity = Entity::new();
        let output = RorschachTest::generate(&entity, 42);
        assert!(output.contains("TEST 01: RORSCHACH INKBLOT"));
        assert!(output.contains("THERE ARE NO WRONG ANSWERS."));
    }

    #[test]
    fn test_infection_rorschach() {
        let mut entity = Entity::new();
        entity.update_depth(30); // Infection layer
        let output = RorschachTest::generate(&entity, 42);

        assert!(
            output.contains("S E E")
                || output.contains("H U R T S")
                || output.contains("L O O K I N G")
                || output.contains("R E F L E C T I O N")
                || output.contains("B L O T")
        );
    }
}
