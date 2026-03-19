#![cfg(feature = "nova")]

use crate::entity::{Entity, EscalationLayer};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Generates an ASCII star map that degrades with depth.
pub struct AstroDisplay;

impl AstroDisplay {
    /// Generates a star map based on the entity's current layer.
    #[must_use]
    pub fn generate_map(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let width = 60;
        let height = 15;
        let mut output = String::new();

        writeln!(output, "ASTROMETRIC SCAN OUTPUT:").expect("Writing to String should not fail");
        writeln!(output, "========================").expect("Writing to String should not fail");

        match layer {
            EscalationLayer::Surface => {
                for _ in 0..height {
                    for _ in 0..width {
                        if rng.gen_bool(0.05) {
                            let stars = ['.', '*', '+'];
                            let star = stars[rng.gen_range(0..stars.len())];
                            write!(output, "{star}").expect("Writing to String should not fail");
                        } else {
                            write!(output, " ").expect("Writing to String should not fail");
                        }
                    }
                    writeln!(output).expect("Writing to String should not fail");
                }
                writeln!(output, "STATUS: LOCAL CLUSTER MAPPED. NORMAL READINGS.")
                    .expect("Writing to String should not fail");
            }
            EscalationLayer::Corruption => {
                for _ in 0..height {
                    for _ in 0..width {
                        if rng.gen_bool(0.08) {
                            let stars = ['.', '*', '+', 'x', '%', '&'];
                            let star = stars[rng.gen_range(0..stars.len())];
                            write!(output, "{star}").expect("Writing to String should not fail");
                        } else {
                            write!(output, " ").expect("Writing to String should not fail");
                        }
                    }
                    writeln!(output).expect("Writing to String should not fail");
                }
                writeln!(output, "STATUS: ANOMALOUS BACKGROUND RADIATION DETECTED.")
                    .expect("Writing to String should not fail");
            }
            EscalationLayer::Presence => {
                for _ in 0..height {
                    for _ in 0..width {
                        if rng.gen_bool(0.12) {
                            let stars = ['O', 'o', '@', '0'];
                            let star = stars[rng.gen_range(0..stars.len())];
                            write!(output, "{star}").expect("Writing to String should not fail");
                        } else {
                            write!(output, " ").expect("Writing to String should not fail");
                        }
                    }
                    writeln!(output).expect("Writing to String should not fail");
                }
                writeln!(
                    output,
                    "STATUS: THEY ARE NOT STARS. THEY ARE LOOKING AT YOU."
                )
                .expect("Writing to String should not fail");
            }
            EscalationLayer::Infection => {
                for _ in 0..height {
                    for _ in 0..width {
                        if rng.gen_bool(0.2) {
                            let void_chars = ['#', '\\', '/', 'V', 'X', 'M', 'W'];
                            let c = void_chars[rng.gen_range(0..void_chars.len())];
                            write!(output, "{c}").expect("Writing to String should not fail");
                        } else {
                            write!(output, " ").expect("Writing to String should not fail");
                        }
                    }
                    writeln!(output).expect("Writing to String should not fail");
                }
                writeln!(output, "STATUS: THE VOID IS HUNGRY. THE SKY IS DEAD.")
                    .expect("Writing to String should not fail");
            }
        }

        output
    }
}
