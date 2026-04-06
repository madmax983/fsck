use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

pub struct RadioReceiver;

impl RadioReceiver {
    #[must_use]
    pub fn tune(freq_str: &str, entity: &Entity, global_seed: u64) -> String {
        let Ok(freq) = freq_str.parse::<f32>() else {
            return String::from("?INVALID FREQUENCY. FORMAT: 104.5\n");
        };

        // If the frequency is outside of normal radio bands, clamp or reject? Let's just allow it for creepiness
        if !(0.0..=999.9).contains(&freq) {
            return String::from("?FREQUENCY OUT OF RANGE [0.0 - 999.9]\n");
        }

        // Use frequency multiplied by 10 to get a stable integer for seeding
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let freq_int = (freq * 10.0).round() as u64;

        let seed = global_seed.wrapping_add(freq_int);
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        let layer = entity.layer();

        // Specific easter egg frequency
        #[allow(clippy::float_cmp)]
        if (freq - 66.6).abs() < f32::EPSILON {
            return Self::generate_entity_whisper(layer, &mut rng);
        }

        // Determine station type based on the RNG seeded by this frequency
        // 5% chance of a numbers station
        // 5% chance of a victim echo (only if there are commands)
        // 90% chance of static
        let roll: f64 = rng.gen_range(0.0..1.0);

        if roll < 0.05 {
            Self::generate_numbers_station(&mut rng)
        } else if roll < 0.10 && !entity.commands_seen.is_empty() {
            Self::generate_victim_echo(entity, &mut rng)
        } else {
            Self::generate_static(&mut rng)
        }
    }

    fn generate_static(rng: &mut ChaCha8Rng) -> String {
        let mut output = String::with_capacity(64);
        output.push_str("[... ");

        let static_chars = ['K', 'S', 'H', 'Z', '.', '-', '~', 'X'];
        for _ in 0..15 {
            let ch = static_chars.choose(rng).unwrap_or(&'.');
            output.push(*ch);
        }

        output.push_str(" ...]\n");
        output
    }

    fn generate_numbers_station(rng: &mut ChaCha8Rng) -> String {
        let mut output = String::with_capacity(128);
        output.push_str("[MALE VOICE, DISTORTED]\n");

        // 5 groups of 5 numbers
        for i in 0..5 {
            for _ in 0..5 {
                let _ = write!(output, "{}", rng.gen_range(0..10));
            }
            if i < 4 {
                output.push(' ');
            }
        }
        output.push('\n');
        output
    }

    fn generate_victim_echo(entity: &Entity, rng: &mut ChaCha8Rng) -> String {
        let cmd = entity.commands_seen.choose(rng).expect("Checked not empty");
        let mut output = String::with_capacity(64);
        output.push_str("[FAINT WHISPER]\n");

        for c in cmd.chars() {
            if rng.gen_bool(0.3) {
                output.push('.');
            } else {
                output.push(c);
            }
        }
        output.push('\n');
        output
    }

    fn generate_entity_whisper(layer: EscalationLayer, rng: &mut ChaCha8Rng) -> String {
        let whispers = match layer {
            EscalationLayer::Surface => vec!["nobody is listening", "it's dark in here"],
            EscalationLayer::Corruption => vec![
                "I can hear you typing",
                "the directories are bleeding",
                "there is no escape",
            ],
            EscalationLayer::Presence | EscalationLayer::Infection => vec![
                "I AM RIGHT BEHIND YOU",
                "WE ARE FOREVER",
                "IT HURTS IT HURTS IT HURTS",
            ],
        };

        let chosen = whispers.choose(rng).unwrap_or(&"...");
        format!("[TRANSMISSION OVERRIDE]\n{chosen}\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_freq() {
        let entity = Entity::new();
        let result = RadioReceiver::tune("ABC", &entity, 0);
        assert!(result.contains("INVALID FREQUENCY"));
    }

    #[test]
    fn test_out_of_range_freq() {
        let entity = Entity::new();
        let result = RadioReceiver::tune("1000.5", &entity, 0);
        assert!(result.contains("OUT OF RANGE"));
    }

    #[test]
    fn test_special_freq() {
        let entity = Entity::new();
        let result = RadioReceiver::tune("66.6", &entity, 0);
        assert!(result.contains("TRANSMISSION OVERRIDE"));
    }

    #[test]
    fn test_deterministic_output() {
        let entity = Entity::new();
        let result1 = RadioReceiver::tune("104.5", &entity, 42);
        let result2 = RadioReceiver::tune("104.5", &entity, 42);
        assert_eq!(result1, result2);
    }
}
