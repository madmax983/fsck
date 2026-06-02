use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

pub struct WebcamCapture;

impl WebcamCapture {
    #[must_use]
    pub fn capture(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();

        match layer {
            EscalationLayer::Surface => "[WEBCAM: ACTIVE] Feed: Static Noise\n".to_string(),
            EscalationLayer::Corruption => {
                let glitches = [
                    "Multiple shadows detected",
                    "Face not recognized",
                    "Artifacting...",
                ];
                let glitch = glitches[rng.gen_range(0..glitches.len())];
                format!("[WEBCAM: GLITCH] {glitch}\n")
            }
            EscalationLayer::Presence => "[WEBCAM: ERROR] Someone is behind you\n".to_string(),
            EscalationLayer::Infection => {
                "[WEBCAM: RECIPROCAL OBSERVATION] I CAN SEE YOU\n".to_string()
            }
        }
    }
}
