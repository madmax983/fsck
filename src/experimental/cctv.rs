use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// A CCTV simulator that views different parts of the facility.
pub struct CameraSystem;

impl CameraSystem {
    #[must_use]
    pub fn view(cam_id: &str, entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let id = if cam_id.is_empty() { "1" } else { cam_id };
        let mut output = String::with_capacity(256);
        let _ = writeln!(output, "CONNECTING TO CAMERA {id}...");

        match layer {
            EscalationLayer::Surface => {
                let view = match rng.gen_range(0..4) {
                    0 => "STATIC",
                    1 => "EMPTY SERVER ROOM. SERVER RACKS BLINKING.",
                    2 => "LONG HALLWAY. FLUORESCENT LIGHTS FLICKERING.",
                    _ => "MAINTENANCE CLOSET. NOTHING UNUSUAL.",
                };
                let _ = writeln!(output, "[CAMERA {id}]: {view}");
            }
            EscalationLayer::Corruption => {
                if rng.gen_bool(0.3) {
                    let _ = writeln!(
                        output,
                        "[CAMERA {id}]: A FIGURE IS STANDING IN THE CORNER. IT IS NOT MOVING."
                    );
                } else {
                    let _ = writeln!(
                        output,
                        "[CAMERA {id}]: ROOM IS DISTORTED. CAMERA FEED CORRUPTED."
                    );
                }
            }
            EscalationLayer::Presence => {
                if rng.gen_bool(0.5) {
                    let _ = writeln!(
                        output,
                        "[CAMERA {id}]: THEY ARE LOOKING DIRECTLY AT THE LENS."
                    );
                } else {
                    let _ = writeln!(output, "[CAMERA {id}]: FEED CUT. TOO CLOSE.");
                }
            }
            EscalationLayer::Infection => {
                let _ = writeln!(output, "[CAMERA {id}]: █▀▄█ TURN AROUND █▄▀█");
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
    fn test_surface_camera() {
        let entity = Entity::new();
        let result = CameraSystem::view("1", &entity, 42);
        assert!(result.contains("[CAMERA 1]:"));
        assert!(
            result.contains("STATIC")
                || result.contains("EMPTY")
                || result.contains("HALLWAY")
                || result.contains("MAINTENANCE")
        );
    }

    #[test]
    fn test_infection_camera() {
        let mut entity = Entity::new();
        entity.add_depth(30);
        let result = CameraSystem::view("2", &entity, 42);
        assert!(result.contains("TURN AROUND"));
    }
}
