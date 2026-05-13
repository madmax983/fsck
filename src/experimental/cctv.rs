use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// A simulated CCTV interface to tap into the facility's cameras.
pub struct CctvViewer;

impl CctvViewer {
    #[must_use]
    pub fn view_camera(entity: &Entity, base_seed: u64, cam_id: &str) -> String {
        let mut output = String::with_capacity(512);

        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let display_id = if cam_id.is_empty() { "CAM-00" } else { cam_id };

        let _ = writeln!(output, "CONNECTING TO {display_id}...");

        match layer {
            EscalationLayer::Surface => {
                let scenes = [
                    "EMPTY HALLWAY",
                    "SERVER ROOM. BLINKING LIGHTS.",
                    "LOBBY. NO MOVEMENT.",
                    "PARKING LOT. ONE CAR.",
                ];
                let chosen = scenes[rng.gen_range(0..scenes.len())];
                let _ = writeln!(output, "[ FEED: {chosen} ]");
            }
            EscalationLayer::Corruption => {
                let scenes = [
                    "EMPTY HALLWAY. FLICKERING LIGHTS.",
                    "SERVER ROOM. SHADOW IN THE CORNER.",
                    "LOBBY. THE DESK IS GONE.",
                    "PARKING LOT. SOMETHING IS CRAWLING.",
                    "STAIRWELL. A TALL FIGURE.",
                ];
                let chosen = scenes[rng.gen_range(0..scenes.len())];
                let _ = writeln!(output, "[ FEED: {chosen} ]");
                if rng.gen_bool(0.3) {
                    let _ = writeln!(output, "WARNING: MOTION DETECTED.");
                }
            }
            EscalationLayer::Presence => {
                let _ = writeln!(output, "[ FEED: THE CAMERA IS COVERED IN SOMETHING DARK. ]");
                if rng.gen_bool(0.5) {
                    let _ = writeln!(output, "AUDIO DETECTED: 'I see you looking.'");
                }
            }
            EscalationLayer::Infection => {
                let _ = writeln!(output, "THIS IS A MIRROR.");
                let _ = writeln!(output, "[      YOU     ]");
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
    fn test_cctv_surface() {
        let entity = Entity::new();
        let output = CctvViewer::view_camera(&entity, 42, "CAM-01");
        assert!(output.contains("CONNECTING TO CAM-01..."));
        assert!(
            output.contains("EMPTY HALLWAY")
                || output.contains("SERVER ROOM")
                || output.contains("LOBBY")
                || output.contains("PARKING LOT")
        );
    }

    #[test]
    fn test_cctv_infection() {
        let mut entity = Entity::new();
        entity.add_depth(30); // Infection layer
        let output = CctvViewer::view_camera(&entity, 42, "CAM-02");
        assert!(output.contains("THIS IS A MIRROR."));
        assert!(output.contains("[      YOU     ]"));
    }
}
