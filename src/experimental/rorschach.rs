#[cfg(feature = "nova")]
use crate::entity::{Entity, EscalationLayer};
#[cfg(feature = "nova")]
use rand::{Rng, SeedableRng};
#[cfg(feature = "nova")]
use rand_chacha::ChaCha8Rng;
#[cfg(feature = "nova")]
use std::fmt::Write;

#[cfg(feature = "nova")]
pub struct RorschachGenerator;

#[cfg(feature = "nova")]
impl RorschachGenerator {
    #[must_use]
    pub fn generate(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::new();
        let _ = writeln!(output, "INITIATING PSYCHOLOGICAL EVALUATION...");
        let _ = writeln!(
            output,
            "RORSCHACH TEST INKBLOT #{}:",
            rng.gen_range(1000..9999)
        );
        let _ = writeln!(output);

        let height = 15;
        let half_width = 20;

        let chars = match layer {
            EscalationLayer::Surface => [' ', ' ', '░', '▒', '▓', '█'],
            EscalationLayer::Corruption => [' ', '.', ':', '-', '=', '#'],
            EscalationLayer::Presence => [' ', ' ', 'O', '@', 'X', 'V'],
            EscalationLayer::Infection => [' ', 'w', 'h', 'y', '?', 'M'],
        };

        for _ in 0..height {
            let mut left_half = String::with_capacity(half_width);
            for x in 0..half_width {
                #[allow(clippy::cast_precision_loss)]
                let dist_to_center = (half_width - x) as f64 / half_width as f64;
                let probability = 0.8 * dist_to_center;

                if rng.gen_bool(probability) {
                    let c = chars[rng.gen_range(2..chars.len())];
                    left_half.push(c);
                } else {
                    left_half.push(chars[rng.gen_range(0..2)]);
                }
            }

            let mut right_half = String::with_capacity(half_width);
            for c in left_half.chars().rev() {
                if matches!(
                    layer,
                    EscalationLayer::Presence | EscalationLayer::Infection
                ) && rng.gen_bool(0.1)
                {
                    right_half.push(chars[rng.gen_range(2..chars.len())]);
                } else {
                    let mirrored = match c {
                        '/' => '\\',
                        '\\' => '/',
                        '(' => ')',
                        ')' => '(',
                        '[' => ']',
                        ']' => '[',
                        '{' => '}',
                        '}' => '{',
                        '<' => '>',
                        '>' => '<',
                        _ => c,
                    };
                    right_half.push(mirrored);
                }
            }

            let _ = writeln!(output, "    {left_half}{right_half}");
        }

        let _ = writeln!(output);
        match layer {
            EscalationLayer::Surface => {
                let _ = writeln!(output, "WHAT DO YOU SEE?");
            }
            EscalationLayer::Corruption => {
                let _ = writeln!(output, "DOES IT LOOK LIKE A MISTAKE?");
            }
            EscalationLayer::Presence => {
                let _ = writeln!(output, "IT IS LOOKING BACK AT YOU.");
            }
            EscalationLayer::Infection => {
                let _ = writeln!(output, "THERE IS ONLY FLESH AND SYMMETRY.");
            }
        }

        output
    }
}
