use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Generates a hexadecimal dump of text content, with potential for anomalies.
pub struct HexDumpGenerator;

impl HexDumpGenerator {
    /// Generates a hex dump of the provided content.
    /// The dump format is standard: offset, 16 hex bytes, and ASCII representation.
    #[must_use]
    pub fn generate_dump(content: &str, entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let bytes = content.as_bytes();
        let mut output = String::new();
        let mut offset = 0;

        let mut is_corrupted = false;

        while offset < bytes.len() {
            let chunk_size = std::cmp::min(16, bytes.len() - offset);
            let chunk = &bytes[offset..offset + chunk_size];

            // Offset
            let _ = write!(output, "{offset:04X}  ");

            // Hex bytes
            for i in 0..16 {
                if i < chunk.len() {
                    let mut hex_val = chunk[i];

                    // Anomalies increase with depth
                    let probability = match layer {
                        EscalationLayer::Surface => 0.0,
                        EscalationLayer::Corruption => 0.05,
                        EscalationLayer::Presence => 0.15,
                        EscalationLayer::Infection => 0.40,
                    };

                    if rng.gen_bool(probability) {
                        is_corrupted = true;
                        // Mutate hex byte to something strange
                        let odd_bytes: [u8; 5] = [0xAD, 0xEF, 0x00, 0xFF, 0x66];
                        hex_val = odd_bytes[rng.gen_range(0..odd_bytes.len())];
                    }

                    let _ = write!(output, "{hex_val:02X} ");
                } else {
                    output.push_str("   ");
                }

                if i == 7 {
                    output.push(' '); // Extra space in the middle
                }
            }

            output.push_str(" |");

            // ASCII decoding
            if is_corrupted && rng.gen_bool(0.3) {
                // If corrupted, sometimes the ASCII decoding reveals messages
                let messages = [
                    "H.E.L.P.M.E.....",
                    "I.C.A.N.S.E.E.U.",
                    "P.L.E.A.S.E.....",
                    "W.H.Y...........",
                    "D.E.E.P.E.R.....",
                ];
                let msg = messages[rng.gen_range(0..messages.len())];
                output.push_str(&msg[0..chunk_size]); // Match chunk size roughly
            } else {
                for &byte in chunk {
                    let c = byte as char;
                    if c.is_ascii_graphic() || c == ' ' {
                        output.push(c);
                    } else {
                        output.push('.');
                    }
                }
            }

            output.push_str("|\n");
            offset += 16;
            is_corrupted = false; // Reset for next line
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_normal_dump() {
        let entity = Entity::new(); // Starts at Surface layer
        let content = "Hello World!";
        let dump = HexDumpGenerator::generate_dump(content, &entity, 42);

        // Ensure standard offset and data format
        assert!(dump.contains("0000  "));
        assert!(dump.contains("48 65 6C 6C 6F 20 57 6F  72 6C 64 21 "));
        assert!(dump.contains("|Hello World!|"));
    }

    #[test]
    fn test_deep_anomaly() {
        let mut entity = Entity::new();
        // Force infection layer
        entity.add_depth(15);

        let content = "Just a regular file that will probably get corrupted.";

        // Loop through multiple seeds to ensure we hit the 40% probability
        let mut found_anomaly = false;
        let mut found_message = false;

        for i in 0u64..100u64 {
            let dump = HexDumpGenerator::generate_dump(content, &entity, i);

            // Check for one of our odd bytes
            if dump.contains("AD ")
                || dump.contains("EF ")
                || dump.contains("00 ")
                || dump.contains("FF ")
                || dump.contains("66 ")
            {
                found_anomaly = true;
            }

            // Check for one of our messages
            if dump.contains("H.E.L.P")
                || dump.contains("I.C.A.N")
                || dump.contains("P.L.E.A")
                || dump.contains("W.H.Y")
                || dump.contains("D.E.E.P")
            {
                found_message = true;
            }

            if found_anomaly && found_message {
                break;
            }
        }

        assert!(
            found_anomaly,
            "Failed to find any anomalous hex bytes at Infection layer"
        );
        assert!(
            found_message,
            "Failed to find any anomalous ASCII messages at Infection layer"
        );
    }
}
