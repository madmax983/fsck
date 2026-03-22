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
            EscalationLayer::Surface => format!("SPEAK: {text}"),
            EscalationLayer::Corruption => {
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
            EscalationLayer::Presence => {
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
            EscalationLayer::Infection => {
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
    }
}
