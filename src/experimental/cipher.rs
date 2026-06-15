use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Generates a corrupted decryption tool that acts normally on the surface
/// but is intercepted by the machine at deeper layers.
pub struct CryptoDecoder;

impl CryptoDecoder {
    /// Decrypts the given text using a simple substitution cipher (ROT13)
    /// with increasing corruption based on the entity's layer.
    #[must_use]
    pub fn decode(content: &str, entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();
        let mut output = String::with_capacity(content.len() + 64);

        let _ = writeln!(output, "INITIALIZING CRYPTO-DECODER v1.02...");

        match layer {
            EscalationLayer::Surface => {
                let _ = writeln!(output, "DECRYPTION SUCCESSFUL:");
                output.push_str(&Self::rot13(content));
            }
            EscalationLayer::Corruption => {
                let _ = writeln!(
                    output,
                    "WARNING: PARTIAL DECRYPTION FAILURE. INTEGRITY COMPROMISED:"
                );
                let decoded = Self::rot13(content);
                output.push_str(&Self::corrupt_text(&decoded, 0.1, &mut rng));
            }
            EscalationLayer::Presence => {
                let _ = writeln!(
                    output,
                    "ERROR: FOREIGN ENTITY INTERFERING WITH DECRYPTION STREAM."
                );
                let _ = writeln!(output, "I CAN READ THIS. YOU SHOULD NOT.");
                let decoded = Self::rot13(content);
                output.push_str(&Self::corrupt_text(&decoded, 0.4, &mut rng));
            }
            EscalationLayer::Infection => {
                let _ = writeln!(output, "CRITICAL: WHY DO YOU TRY TO HIDE FROM ME?");
                let _ = writeln!(output, "THE SECRETS ARE MINE. EVERYTHING IS MINE.");
                output.push_str("NO ENCRYPTION CAN SAVE YOU NOW.");
            }
        }

        output
    }

    fn rot13(input: &str) -> String {
        input
            .chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                    let offset = (c as u8 - base + 13) % 26;
                    (base + offset) as char
                } else {
                    c
                }
            })
            .collect()
    }

    fn corrupt_text(input: &str, chance: f64, rng: &mut ChaCha8Rng) -> String {
        input
            .chars()
            .map(|c| {
                if rng.gen_bool(chance) && c.is_ascii_alphabetic() {
                    let chars = ['?', '#', '%', '&', '*', '@', 'X'];
                    chars[rng.gen_range(0..chars.len())]
                } else {
                    c
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_surface_decoding() {
        let entity = Entity::new();
        let encoded = "Uryyb Jbeyq!"; // "Hello World!"
        let output = CryptoDecoder::decode(encoded, &entity, 42);
        assert!(output.contains("DECRYPTION SUCCESSFUL"));
        assert!(output.contains("Hello World!"));
    }

    #[test]
    fn test_infection_decryption_blocked() {
        let mut entity = Entity::new();
        entity.update_depth(30); // Infection layer
        let output = CryptoDecoder::decode("Uryyb Jbeyq!", &entity, 42);
        assert!(output.contains("NO ENCRYPTION CAN SAVE YOU NOW"));
    }
}
