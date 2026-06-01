use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// A CCTV viewer that degrades from mundane security feeds into horrifying observations.
pub struct CctvViewer;

impl CctvViewer {
    #[must_use]
    pub fn view_cameras(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();
        let mut output = String::new();

        let _ = writeln!(output, "CONNECTING TO SECURITY FEED...\n");

        match layer {
            EscalationLayer::Surface => Self::generate_surface(&mut output, &mut rng),
            EscalationLayer::Corruption => Self::generate_corruption(&mut output),
            EscalationLayer::Presence => Self::generate_presence(&mut output),
            EscalationLayer::Infection => Self::generate_infection(&mut output),
        }

        output
    }

    fn generate_surface(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(output, "CAM 1 [FRONT DESK] : NO SIGNAL");
        let _ = writeln!(output, "CAM 2 [SERVER RM]  : ONLINE - NO MOTION");
        let _ = writeln!(output, "CAM 3 [LOADING]    : OFFLINE");
        let _ = writeln!(output, "CAM 4 [BREAK RM]   : ONLINE - {} DETECTED", if rng.gen_bool(0.2) { "MOTION" } else { "NO MOTION" });
    }

    fn generate_corruption(output: &mut String) {
        let _ = writeln!(output, "CAM 1 [FRONT DESK] : SIGNAL CORRUPTED");
        let _ = writeln!(output, "CAM 2 [SERVER RM]  : ONLINE - FIGURE STANDING IN CORNER");
        let _ = writeln!(output, "CAM 3 [LOADING]    : OFFLINE");
        let _ = writeln!(output, "CAM 4 [BREAK RM]   : ONLINE - SHADOWS MOVING");
    }

    fn generate_presence(output: &mut String) {
        let _ = writeln!(output, "CAM 1 [YOUR DOOR]  : RECORDING");
        let _ = writeln!(output, "CAM 2 [HALLWAY]    : APPROACHING");
        let _ = writeln!(output, "CAM 3 [YOUR ROOM]  : YOU ARE LOOKING AT THE SCREEN");
        let _ = writeln!(output, "CAM 4 [BACKUP]     : SIGNAL LOST IN STATIC");
    }

    fn generate_infection(output: &mut String) {
        let _ = writeln!(output, "CAM 1 [EYES]       : WIDE OPEN");
        let _ = writeln!(output, "CAM 2 [INSIDE]     : IT IS UNDER THE SKIN");
        let _ = writeln!(output, "CAM 3 [BEHIND YOU] : DO NOT TURN AROUND");
        let _ = writeln!(output, "CAM 4 [THE TRUTH]  : THERE IS NO CAMERA");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_cctv_surface() {
        let entity = Entity::new();
        let output = CctvViewer::view_cameras(&entity, 42);
        assert!(output.contains("CAM 1 [FRONT DESK]"));
        assert!(output.contains("SERVER RM"));
    }

    #[test]
    fn test_cctv_infection() {
        let mut entity = Entity::new();
        entity.update_depth(30); // Infection layer
        let output = CctvViewer::view_cameras(&entity, 42);
        assert!(output.contains("CAM 3 [BEHIND YOU]"));
    }
}
