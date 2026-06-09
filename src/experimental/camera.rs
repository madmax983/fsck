#[cfg(feature = "nova")]
use crate::entity::{Entity, EscalationLayer};
#[cfg(feature = "nova")]
use rand::{Rng, SeedableRng};
#[cfg(feature = "nova")]
use rand_chacha::ChaCha8Rng;
#[cfg(feature = "nova")]
use std::fmt::Write;

#[cfg(feature = "nova")]
pub struct CameraNetwork;

#[cfg(feature = "nova")]
impl CameraNetwork {
    #[must_use]
    pub fn access_feed(entity: &Entity, base_seed: u64, camera_id: &str) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::new();

        if camera_id.is_empty() {
            let _ = writeln!(output, "ERROR: SPECIFY CAMERA ID (e.g., CAM1, FRONT, ROOM)");
            return output;
        }

        let _ = writeln!(output, "CONNECTING TO CAMERA FEED [{camera_id}]...");

        match layer {
            EscalationLayer::Surface => Self::generate_surface_feed(&mut output, &mut rng, camera_id),
            EscalationLayer::Corruption => Self::generate_corruption_feed(&mut output, &mut rng, camera_id),
            EscalationLayer::Presence => Self::generate_presence_feed(&mut output, &mut rng, camera_id),
            EscalationLayer::Infection => Self::generate_infection_feed(&mut output, &mut rng, camera_id),
        }

        output
    }

    fn generate_surface_feed(output: &mut String, rng: &mut ChaCha8Rng, _id: &str) {
        if rng.gen_bool(0.7) {
            let _ = writeln!(output, "STATUS: OFFLINE");
            let _ = writeln!(output, "NO SIGNAL DETECTED.");
        } else {
            let _ = writeln!(output, "STATUS: ONLINE");
            let _ = writeln!(output, "FEED: [STATIC IMAGE: EMPTY HALLWAY]");
        }
    }

    fn generate_corruption_feed(output: &mut String, rng: &mut ChaCha8Rng, _id: &str) {
        let _ = writeln!(output, "STATUS: INTERMITTENT");
        let glitches = [
            "FRAME DROPPED",
            "MOTION DETECTED: UNKNOWN ENTITY",
            "SIGNAL DEGRADATION: 85%",
            "ARTIFACTING ON LENS",
        ];
        let chosen = glitches[rng.gen_range(0..glitches.len())];
        let _ = writeln!(output, "FEED: {chosen}");
    }

    fn generate_presence_feed(output: &mut String, rng: &mut ChaCha8Rng, _id: &str) {
        let _ = writeln!(output, "STATUS: OVERRIDDEN");
        let visions = [
            "FEED: [IMAGE: YOUR BACK]",
            "FEED: [IMAGE: EMPTY CHAIR]",
            "MOTION DETECTED: RIGHT BEHIND YOU",
            "AUDIO ONLY: HEAVY BREATHING",
        ];
        let chosen = visions[rng.gen_range(0..visions.len())];
        let _ = writeln!(output, "{chosen}");
    }

    fn generate_infection_feed(output: &mut String, rng: &mut ChaCha8Rng, _id: &str) {
        let _ = writeln!(output, "STATUS: WATCHING YOU");
        let horrors = [
            "DON'T TURN AROUND",
            "I CAN SEE YOU BLINK",
            "THE DOOR IS OPENING",
            "WE ARE IN THE ROOM",
        ];
        let chosen = horrors[rng.gen_range(0..horrors.len())];
        let _ = writeln!(output, "FEED: {chosen}");
    }
}

#[cfg(test)]
#[cfg(feature = "nova")]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_camera_surface() {
        let entity = Entity::new();
        let feed = CameraNetwork::access_feed(&entity, 12345, "CAM1");
        assert!(feed.contains("CONNECTING TO CAMERA FEED"));
        assert!(feed.contains("STATUS: "));
    }

    #[test]
    fn test_camera_infection() {
        let mut entity = Entity::new();
        entity.add_depth(30);
        let feed = CameraNetwork::access_feed(&entity, 12345, "CAM1");
        assert!(feed.contains("WATCHING YOU"));
    }

    #[test]
    fn test_camera_empty_id() {
        let entity = Entity::new();
        let feed = CameraNetwork::access_feed(&entity, 12345, "");
        assert!(feed.contains("ERROR: SPECIFY CAMERA ID"));
    }
}
