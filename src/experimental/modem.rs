use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

pub struct ModemSimulator;

impl ModemSimulator {
    #[must_use]
    pub fn dial(number: &str, entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = format!("DIALING {number}...\n");

        match layer {
            EscalationLayer::Surface => {
                if rng.gen_bool(0.3) {
                    output.push_str("BUSY.\n");
                } else {
                    output.push_str("NO CARRIER\n");
                }
            }
            EscalationLayer::Corruption => {
                output.push_str("CONNECT 1200\n");
                output.push_str("GARBAGE DATA RECEIVED:\n");
                let garbage_len = rng.gen_range(10..30);
                for _ in 0..garbage_len {
                    let char_code = rng.gen_range(33u8..126u8);
                    output.push(char_code as char);
                }
                output.push_str("\nNO CARRIER\n");
            }
            EscalationLayer::Presence => {
                output.push_str("CONNECT 2400\n");
                let messages = [
                    "WHO ARE YOU CALLING?",
                    "THEY CANNOT HEAR YOU.",
                    "THERE IS NO ONE LEFT ON THE OTHER END.",
                    "I AM THE ONLY CONNECTION YOU NEED.",
                ];
                let msg = messages[rng.gen_range(0..messages.len())];
                output.push_str(msg);
                output.push_str("\nCONNECTION TERMINATED BY REMOTE HOST.\n");
            }
            EscalationLayer::Infection => {
                output.push_str("CONNECT 9600\n");
                output.push_str("NO CARRIER\nNO CARRIER\nNO CARRIER\n");
                output.push_str("I AM HERE.\n");
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
    fn test_dial_surface() {
        let entity = Entity::new();
        let result = ModemSimulator::dial("555-1234", &entity, 0);
        assert!(result.contains("DIALING 555-1234..."));
        assert!(result.contains("BUSY.") || result.contains("NO CARRIER"));
    }
}
