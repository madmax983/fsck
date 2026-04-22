use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// A procedural radio tuner that reveals number stations, static, and EVP (Electronic Voice Phenomena)
/// that devolve into horror depending on the depth and frequency.
pub struct RadioTuner;

impl RadioTuner {
    /// Generates audio transcripts from a given frequency.
    #[must_use]
    pub fn tune(entity: &Entity, frequency: f32, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        // Combine base seed with frequency to create unique channels
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let channel_seed = interaction_seed.wrapping_add((frequency * 10.0) as u64);
        let mut rng = ChaCha8Rng::seed_from_u64(channel_seed);

        let layer = entity.layer();
        let mut output = String::new();

        let _ = writeln!(output, "TUNING TO {frequency:.1} MHz...");

        // Special "known" frequencies that break the procedural generation
        if (frequency - 66.6).abs() < f32::EPSILON {
            Self::generate_infection_station(&mut output, &mut rng);
            return output;
        }

        if (frequency - 101.1).abs() < f32::EPSILON {
            Self::generate_surface_station(&mut output, &mut rng);
            return output;
        }

        match layer {
            EscalationLayer::Surface => Self::generate_surface_station(&mut output, &mut rng),
            EscalationLayer::Corruption => Self::generate_corruption_station(&mut output, &mut rng),
            EscalationLayer::Presence => Self::generate_presence_station(&mut output, &mut rng),
            EscalationLayer::Infection => Self::generate_infection_station(&mut output, &mut rng),
        }

        output
    }

    fn generate_surface_station(output: &mut String, rng: &mut ChaCha8Rng) {
        if rng.gen_bool(0.7) {
            let _ = writeln!(output, "[STATIC... WHITE NOISE...]");
        } else {
            let music = [
                "[FAINT CLASSICAL MUSIC PLAYING]",
                "[A WEATHER BROADCAST... MOSTLY STATIC]",
                "[SOMEONE SPEAKING RUSSIAN...]",
            ];
            let chosen = music[rng.gen_range(0..music.len())];
            let _ = writeln!(output, "{chosen}");
        }
    }

    fn generate_corruption_station(output: &mut String, rng: &mut ChaCha8Rng) {
        let choice = rng.gen_range(0.0..1.0);
        if choice < 0.5 {
            let _ = writeln!(
                output,
                "[NUMBER STATION: {} {} {} {}]",
                rng.gen_range(0..10),
                rng.gen_range(0..10),
                rng.gen_range(0..10),
                rng.gen_range(0..10)
            );
        } else if choice < 0.75 {
            let _ = writeln!(
                output,
                "[STATIC INTERRUPTED BY A METRONOME... TICK... TICK... TICK...]"
            );
        } else {
            let _ = writeln!(
                output,
                "[A DISTORTED VOICE... 'THE WEATHER IS... THE WEATHER IS...']"
            );
        }
    }

    fn generate_presence_station(output: &mut String, rng: &mut ChaCha8Rng) {
        let choice = rng.gen_range(0.0..1.0);
        if choice < 0.4 {
            let _ = writeln!(output, "[HEAVY BREATHING... CLOSE TO THE MICROPHONE...]");
        } else if choice < 0.8 {
            let _ = writeln!(output, "[A REPEATING PHRASE: 'CAN YOU HEAR ME?']");
        } else {
            let words = ["COLD", "DARK", "PLEASE", "HURTS"];
            let chosen = words[rng.gen_range(0..words.len())];
            let _ = writeln!(output, "[A VOICE THROUGH THE STATIC: '{chosen}']");
        }
    }

    fn generate_infection_station(output: &mut String, rng: &mut ChaCha8Rng) {
        let horror = [
            "[SCREAMS BURIED UNDER WHITE NOISE]",
            "[IT SEES YOU]",
            "[SILENCE. PERFECT, HEAVY SILENCE.]",
            "['THE FLESH IS THE DISK...']",
            "[YOUR OWN VOICE, REPEATING EVERYTHING YOU JUST SAID]",
        ];
        let chosen = horror[rng.gen_range(0..horror.len())];
        let _ = writeln!(output, "{chosen}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_surface_tuning() {
        let entity = Entity::new();
        let output = RadioTuner::tune(&entity, 101.1, 42);
        assert!(output.contains("STATIC") || output.contains("RUSSIAN") || output.contains("CLASSICAL"));
    }

    #[test]
    fn test_infection_tuning() {
        let mut entity = Entity::new();
        entity.update_depth(30); // Infection layer
        let output = RadioTuner::tune(&entity, 66.6, 42);
        assert!(
            output.contains("IT SEES YOU")
                || output.contains("SILENCE")
                || output.contains("SCREAMS")
                || output.contains("FLESH")
                || output.contains("YOUR OWN VOICE")
        );
    }
}
