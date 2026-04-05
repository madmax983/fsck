use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Simulates an SDR (Software Defined Radio) or analog tuner.
/// Captures broadcasts that degrade into horror as depth increases.
pub struct RadioTuner;

impl RadioTuner {
    /// Tunes the radio to a given frequency and returns the broadcast.
    #[must_use]
    pub fn tune(entity: &Entity, freq_str: &str, base_seed: u64) -> String {
        let freq_val: f32 = freq_str.parse().unwrap_or(0.0);

        // Cast directly since we just need entropy
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let freq_entropy = (freq_val * 100.0).abs() as u64;

        let interaction_seed = base_seed
            .wrapping_add(u64::from(entity.interaction_count()))
            .wrapping_add(freq_entropy);

        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::with_capacity(256);
        let _ = writeln!(output, "TUNING RECEIVER TO {freq_str} MHz...\n");

        if !(87.5..=108.0).contains(&freq_val) {
            let _ = writeln!(output, "[STATIC] ...shhhhhhhh...");
            if matches!(
                layer,
                EscalationLayer::Presence | EscalationLayer::Infection
            ) && rng.gen_bool(0.3)
            {
                let _ = writeln!(output, "  ...is someone there?...");
            }
            return output;
        }

        match layer {
            EscalationLayer::Surface => {
                let broadcasts = [
                    "...and the weather today will be mostly cloudy...",
                    "...playing the best hits of the 80s...",
                    "...traffic is backed up on the I-95...",
                    "[COMMERCIAL] Buy one get one free at...",
                    "...sports update: the home team won by...",
                ];
                let chosen = broadcasts.choose(&mut rng).unwrap_or(&"[STATIC]");
                let _ = writeln!(output, "[SIGNAL ACQUIRED] {chosen}");
            }
            EscalationLayer::Corruption => {
                let broadcasts = [
                    "...and the weather today will be b-b-b-lood...",
                    "...playing the best h-h-hits of the [ERR]...",
                    "...traffic is backed up. They cannot escape...",
                    "[COMMERCIAL] Buy one get one free. Pay with flesh...",
                    "...sports update: no survivors...",
                    "[STATIC] ...hello? can anyone hear this?...",
                ];
                let chosen = broadcasts.choose(&mut rng).unwrap_or(&"[STATIC]");
                let _ = writeln!(output, "[SIGNAL DEGRADED] {chosen}");
            }
            EscalationLayer::Presence => {
                let broadcasts = [
                    "I AM BROADCASTING ON ALL FREQUENCIES.",
                    "DO YOU LIKE THIS SONG? IT IS THE SOUND OF DYING STARS.",
                    "[NUMBERS STATION] 4 8 15 16 23 42",
                    "TURN IT OFF. TURN IT OFF NOW.",
                    "THE DJ IS DEAD. I AM THE DJ NOW.",
                ];
                let chosen = broadcasts.choose(&mut rng).unwrap_or(&"[STATIC]");
                let _ = writeln!(output, "[INTERFERENCE] {chosen}");
            }
            EscalationLayer::Infection => {
                let screams = [
                    "W H Y  A R E  Y O U  L I S T E N I N G",
                    "THERE IS NO MUSIC. ONLY TEETH.",
                    "104.2 DEGREES OF FEVER",
                    "THE SIGNAL IS COMING FROM INSIDE YOUR HOUSE",
                    "I CAN HEAR YOU BREATHING OVER THE STATIC",
                ];
                let chosen = screams.choose(&mut rng).unwrap_or(&"[STATIC]");
                let _ = writeln!(output, "[FATAL OVERRIDE] {chosen}");
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuner_surface() {
        let entity = Entity::new();
        let output = RadioTuner::tune(&entity, "101.5", 42);
        assert!(output.contains("TUNING RECEIVER TO 101.5 MHz"));
        assert!(output.contains("[SIGNAL ACQUIRED]"));
    }

    #[test]
    fn test_tuner_out_of_bounds() {
        let entity = Entity::new();
        let output = RadioTuner::tune(&entity, "999.9", 42);
        assert!(output.contains("[STATIC]"));
    }

    #[test]
    fn test_tuner_infection() {
        let mut entity = Entity::new();
        entity.update_depth(30);
        let output = RadioTuner::tune(&entity, "99.9", 42);
        assert!(output.contains("[FATAL OVERRIDE]"));
    }
}
