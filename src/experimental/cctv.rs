use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Connects to archaic camera feeds that degrade and reveal unsettling sights.
pub struct CctvViewer;

impl CctvViewer {
    #[must_use]
    pub fn view_feed(entity: &Entity, base_seed: u64, cam_id: &str) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();
        let mut output = String::new();

        let target_cam = if cam_id.is_empty() {
            rng.gen_range(1..10)
        } else {
            cam_id.parse::<u32>().unwrap_or(0)
        };

        if target_cam == 0 {
            return "ERROR: INVALID CAMERA ID\n".to_string();
        }

        let _ = writeln!(output, "CONNECTING TO CAMERA FEED {target_cam}...");

        match layer {
            EscalationLayer::Surface => Self::generate_surface_feed(&mut output, &mut rng),
            EscalationLayer::Corruption => Self::generate_corruption_feed(&mut output, &mut rng),
            EscalationLayer::Presence => Self::generate_presence_feed(&mut output, &mut rng),
            EscalationLayer::Infection => Self::generate_infection_feed(&mut output, &mut rng),
        }

        output
    }

    fn generate_surface_feed(output: &mut String, rng: &mut ChaCha8Rng) {
        let feeds = [
            "FEED ACTIVE: Empty server room. Blinking lights.",
            "FEED ACTIVE: Hallway 3B. No movement detected.",
            "FEED ACTIVE: Loading dock. Doors secured.",
            "FEED ACTIVE: Break room. Fluorescent lights flickering.",
            "FEED ACTIVE: Exterior gate. Weather: Overcast.",
        ];
        let chosen = feeds[rng.gen_range(0..feeds.len())];
        let _ = writeln!(output, "STATUS: ONLINE\n[ {chosen} ]");
    }

    fn generate_corruption_feed(output: &mut String, rng: &mut ChaCha8Rng) {
        let feeds = [
            "FEED ACTIVE: Empty server r-- [SIGNAL DEGRADED]",
            "FEED ACTIVE: Hallway 3B. Shadow shifting in the corner.",
            "FEED ACTIVE: Loading dock. Doors unsecur##@$!",
            "FEED ACTIVE: Break room. Who left the monitor on?",
            "FEED ACTIVE: Exterior gate. Something is standing outside.",
        ];
        let chosen = feeds[rng.gen_range(0..feeds.len())];
        let _ = writeln!(output, "STATUS: INTERMITTENT\n[ {chosen} ]");
    }

    fn generate_presence_feed(output: &mut String, rng: &mut ChaCha8Rng) {
        let feeds = [
            "FEED ACTIVE: It is looking directly at the camera.",
            "FEED ACTIVE: The room is breathing.",
            "FEED ACTIVE: The shadow resembles a person.",
            "FEED ACTIVE: Why is the camera panning by itself?",
            "FEED ACTIVE: [ NO SIGNAL ] - Wait, I can hear it.",
        ];
        let chosen = feeds[rng.gen_range(0..feeds.len())];
        let _ = writeln!(output, "STATUS: COMPROMISED\n[ {chosen} ]");
    }

    fn generate_infection_feed(output: &mut String, rng: &mut ChaCha8Rng) {
        let feeds = [
            "FEED ACTIVE: YOU ARE LOOKING AT YOURSELF FROM BEHIND.",
            "FEED ACTIVE: THE EYES. SO MANY EYES.",
            "FEED ACTIVE: THE CAMERA LENS IS BLEEDING.",
            "FEED ACTIVE: IT IS INSIDE THE MONITOR.",
            "FEED ACTIVE: TURN AROUND NOW.",
        ];
        let chosen = feeds[rng.gen_range(0..feeds.len())];
        let _ = writeln!(output, "STATUS: CRITICAL\n[ {chosen} ]");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_surface_cctv() {
        let entity = Entity::new();
        let output = CctvViewer::view_feed(&entity, 42, "1");
        assert!(output.contains("CONNECTING TO CAMERA FEED 1..."));
        assert!(output.contains("STATUS: ONLINE"));
    }

    #[test]
    fn test_infection_cctv() {
        let mut entity = Entity::new();
        entity.update_depth(30);
        let output = CctvViewer::view_feed(&entity, 42, "9");
        assert!(output.contains("STATUS: CRITICAL"));
    }
}
