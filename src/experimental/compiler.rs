use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Simulates a code compiler that degrades based on entity depth.
pub struct CompilerSimulator;

impl CompilerSimulator {
    #[must_use]
    pub fn compile(target_file: &str, entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let _rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();
        let mut output = String::new();

        let _ = writeln!(output, "[  0%] Building C object {target_file}");
        let _ = writeln!(output, "[ 20%] Building C object {target_file}");
        let _ = writeln!(output, "[ 40%] Building C object {target_file}");

        match layer {
            EscalationLayer::Surface => {
                let _ = writeln!(output, "[ 60%] Building C object {target_file}");
                let _ = writeln!(output, "[ 80%] Building C object {target_file}");
                let _ = writeln!(output, "[100%] Built target {target_file}");
            }
            EscalationLayer::Corruption => {
                let _ = writeln!(
                    output,
                    "[ 60%] warning: implicit declaration of function 'escape'"
                );
                let _ = writeln!(output, "[ 80%] Building C object {target_file}");
                let _ = writeln!(output, "[100%] Built target {target_file}");
            }
            EscalationLayer::Presence => {
                let _ = writeln!(
                    output,
                    "[ 60%] warning: 'flesh' is uninitialized in this function"
                );
                let _ = writeln!(output, "[ 80%] Building C object {target_file}");
                let _ = writeln!(output, "[ 99%] Built target {target_file}");
                let _ = writeln!(output, "ld: error: undefined reference to 'GOD'");
            }
            EscalationLayer::Infection => {
                let _ = writeln!(output, "[ 60%] fatal error: too many missing limbs");
                let _ = writeln!(output, "[ 66%] IT HURTS");
                let _ = writeln!(output, "[ 66%] IT HURTS");
                let _ = writeln!(output, "[ 66%] IT HURTS");
            }
        }

        output
    }
}
