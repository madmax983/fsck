use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Simulates a cryptographic analysis tool that decrypts or encrypts strings,
/// but breaks down and hallucinates as the system's paranoia (EscalationLayer) increases.
pub struct CryptTool;

impl CryptTool {
    #[must_use]
    pub fn analyze(text: &str, entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::with_capacity(512);
        let _ = writeln!(output, "ANALYZING CRYPTOGRAPHIC SIGNATURE...");

        if text.is_empty() {
            let _ = writeln!(output, "NO INPUT PROVIDED.");
            return output;
        }

        // Basic "encryption" - just rot13 for flavor
        let rot13: String = text
            .chars()
            .map(|c| {
                if c.is_ascii_lowercase() {
                    (((c as u8 - b'a') + 13) % 26 + b'a') as char
                } else if c.is_ascii_uppercase() {
                    (((c as u8 - b'A') + 13) % 26 + b'A') as char
                } else {
                    c
                }
            })
            .collect();

        match layer {
            EscalationLayer::Surface => {
                let _ = writeln!(output, "CIPHER DETECTED: ROT-13");
                let _ = writeln!(output, "RESULT: {}", rot13);
            }
            EscalationLayer::Corruption => {
                let _ = writeln!(output, "CIPHER DETECTED: ROT-13");
                let _ = writeln!(output, "RESULT: {}", rot13);
                if rng.gen_bool(0.3) {
                    let _ = writeln!(output, "WARNING: ENCRYPTION KEYS CORRUPTED");
                }
            }
            EscalationLayer::Presence => {
                let _ = writeln!(output, "CIPHER DETECTED: UNKNOWN NON-STANDARD PRIME");
                let _ = writeln!(output, "RESULT: {}", rot13);
                let _ = writeln!(output, "ANALYSIS: THE MESSAGE IS BLEEDING THROUGH THE CIPHER.");
            }
            EscalationLayer::Infection => {
                let _ = writeln!(output, "CIPHER DETECTED: FLESH");
                let _ = writeln!(output, "RESULT: THERE ARE NO SECRETS HERE. {}", rot13);
                for _ in 0..3 {
                    let noise: u32 = rng.r#gen();
                    let _ = writeln!(output, "ERR_DECRYPT: 0x{:08X}", noise);
                }
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_crypt_surface() {
        let entity = Entity::new();
        let result = CryptTool::analyze("HELLO", &entity, 42);
        assert!(result.contains("URYYB")); // ROT13 of HELLO
        assert!(result.contains("CIPHER DETECTED: ROT-13"));
    }

    #[test]
    fn test_crypt_infection() {
        let mut entity = Entity::new();
        entity.update_depth(30); // Infection
        let result = CryptTool::analyze("HELLO", &entity, 42);
        assert!(result.contains("URYYB"));
        assert!(result.contains("CIPHER DETECTED: FLESH"));
    }
}
