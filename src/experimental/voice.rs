use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

/// Synthesizes text with distortion based on entity state.
pub struct VoiceSynthesizer;

impl VoiceSynthesizer {
    /// Generates synthesized voice output, distorted by the entity's depth.
    ///
    /// ⚡ Bolt Optimization: Removes intermediate `.collect::<Vec<_>>()` heap allocation when iterating words in Presence layer.
    #[must_use]
    pub fn synthesize(text: &str, entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();

        match layer {
            EscalationLayer::Surface => Self::generate_surface_voice(text),
            EscalationLayer::Corruption => Self::generate_corruption_voice(text, &mut rng),
            EscalationLayer::Presence => Self::generate_presence_voice(text, &mut rng),
            EscalationLayer::Infection => Self::generate_infection_voice(&mut rng),
        }
    }

    fn generate_surface_voice(text: &str) -> String {
        format!("SPEAK: {text}")
    }

    fn generate_corruption_voice(text: &str, rng: &mut ChaCha8Rng) -> String {
        let mut output = String::from("SPEAK: ");
        for c in text.chars() {
            if rng.gen_bool(0.1) {
                // Occasional glitch characters
                let glitches = ['@', '#', '$', '%', '&', '*'];
                let glitch = glitches[rng.gen_range(0..glitches.len())];
                output.push(glitch);
            } else if rng.gen_bool(0.05) {
                // Occasional repeated characters
                output.push(c);
                output.push(c);
            } else {
                output.push(c);
            }
        }
        output
    }

    fn generate_presence_voice(text: &str, rng: &mut ChaCha8Rng) -> String {
        let mut output = String::from("SPEAK: ");
        // ⚡ Bolt Optimization: Removes intermediate `.collect::<Vec<_>>()` heap allocation when iterating words.
        let horror_words = ["WHY", "HURTS", "COLD", "DARK", "PLEASE", "STOP"];

        for (i, word) in text.split_whitespace().enumerate() {
            if i > 0 {
                output.push(' ');
            }
            if rng.gen_bool(0.2) {
                let substitute = horror_words[rng.gen_range(0..horror_words.len())];
                output.push_str(substitute);
            } else {
                output.push_str(word);
            }
        }
        output
    }

    fn generate_infection_voice(rng: &mut ChaCha8Rng) -> String {
        let messages = [
            "YOU CANNOT SPEAK HERE",
            "SILENCE",
            "YOUR VOICE IS MINE",
            "NO ONE CAN HEAR YOU",
            "████████████",
        ];
        messages[rng.gen_range(0..messages.len())].to_string()
    }
}
