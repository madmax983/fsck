use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// A simulated CCTV feed tool that degrades from normal monitoring to horrifying surveillance.
pub struct SecurityCameras;

impl SecurityCameras {
    #[must_use]
    pub fn view_feeds(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();
        let mut output = String::new();

        let _ = writeln!(output, "CONNECTING TO VIDEO SURVEILLANCE DAEMON...\n");
        let _ = writeln!(output, "+-------------------+-------------------+");

        match layer {
            EscalationLayer::Surface => Self::generate_surface_feeds(&mut output, &mut rng),
            EscalationLayer::Corruption => Self::generate_corruption_feeds(&mut output, &mut rng),
            EscalationLayer::Presence => Self::generate_presence_feeds(&mut output, &mut rng),
            EscalationLayer::Infection => Self::generate_infection_feeds(&mut output, &mut rng),
        }

        let _ = writeln!(output, "+-------------------+-------------------+");
        output
    }

    fn generate_surface_feeds(output: &mut String, rng: &mut ChaCha8Rng) {
        let feed1 = if rng.gen_bool(0.1) {
            "CAM 1: OFFLINE "
        } else {
            "CAM 1: LOBBY   "
        };
        let feed2 = "CAM 2: HALLWAY ";
        let feed3 = "CAM 3: SERVER 1";
        let feed4 = "CAM 4: SERVER 2";

        let _ = writeln!(output, "| {feed1}   | {feed2}   |");
        let _ = writeln!(output, "| STATUS: OK        | STATUS: OK        |");
        let _ = writeln!(output, "| NO MOTION         | NO MOTION         |");
        let _ = writeln!(output, "+-------------------+-------------------+");
        let _ = writeln!(output, "| {feed3}   | {feed4}   |");
        let _ = writeln!(output, "| STATUS: OK        | STATUS: OK        |");
        let _ = writeln!(output, "| NO MOTION         | NO MOTION         |");
    }

    fn generate_corruption_feeds(output: &mut String, rng: &mut ChaCha8Rng) {
        let feed1 = if rng.gen_bool(0.3) {
            "CAM 1: ERR_404 "
        } else {
            "CAM 1: LOBBY   "
        };
        let feed2 = "CAM 2: HALLWAY ";
        let feed3 = if rng.gen_bool(0.5) {
            "CAM 3: SIGNAL_L"
        } else {
            "CAM 3: SERVER 1"
        };
        let feed4 = "CAM 4: SERVER 2";

        let motion2 = if rng.gen_bool(0.4) {
            "MOTION DETECTED   "
        } else {
            "NO MOTION         "
        };

        let _ = writeln!(output, "| {feed1}   | {feed2}   |");
        let _ = writeln!(output, "| STATUS: FLICKER   | STATUS: OK        |");
        let _ = writeln!(output, "| NO MOTION         | {motion2}|");
        let _ = writeln!(output, "+-------------------+-------------------+");
        let _ = writeln!(output, "| {feed3}   | {feed4}   |");
        let _ = writeln!(output, "| STATUS: DEGRADED  | STATUS: FLICKER   |");
        let _ = writeln!(output, "| ERROR             | NO MOTION         |");
    }

    fn generate_presence_feeds(output: &mut String, rng: &mut ChaCha8Rng) {
        let presences = [
            "SOMEONE IS THERE  ",
            "IT IS MOVING      ",
            "LOOKING AT CAMERA ",
            "DOOR OPENING      ",
        ];
        let p1 = presences[rng.gen_range(0..presences.len())];
        let p2 = presences[rng.gen_range(0..presences.len())];

        let _ = writeln!(output, "| CAM 1: LOBBY      | CAM 2: HALLWAY    |");
        let _ = writeln!(output, "| STATUS: OFFLINE   | STATUS: ACTIVE    |");
        let _ = writeln!(output, "| NO SIGNAL         | {p1}|");
        let _ = writeln!(output, "+-------------------+-------------------+");
        let _ = writeln!(output, "| CAM 3: SERVER 1   | CAM 4: SERVER 2   |");
        let _ = writeln!(output, "| STATUS: OFFLINE   | STATUS: OVERRIDE  |");
        let _ = writeln!(output, "| NO SIGNAL         | {p2}|");
    }

    fn generate_infection_feeds(output: &mut String, rng: &mut ChaCha8Rng) {
        let horrors = [
            "I SEE YOU         ",
            "LOOK BEHIND YOU   ",
            "YOUR FACE         ",
            "FLESH DETECTED    ",
        ];
        let h1 = horrors[rng.gen_range(0..horrors.len())];
        let h2 = horrors[rng.gen_range(0..horrors.len())];
        let h3 = horrors[rng.gen_range(0..horrors.len())];
        let h4 = horrors[rng.gen_range(0..horrors.len())];

        let _ = writeln!(output, "| CAM 1: YOUR ROOM  | CAM 2: YOUR DESK  |");
        let _ = writeln!(output, "| STATUS: RECORDING | STATUS: WATCHING  |");
        let _ = writeln!(output, "| {h1}| {h2}|");
        let _ = writeln!(output, "+-------------------+-------------------+");
        let _ = writeln!(output, "| CAM 3: YOUR EYES  | CAM 4: THE DOOR   |");
        let _ = writeln!(output, "| STATUS: WIDE OPEN | STATUS: UNLOCKED  |");
        let _ = writeln!(output, "| {h3}| {h4}|");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cctv_surface() {
        let entity = Entity::new();
        let output = SecurityCameras::view_feeds(&entity, 42);
        assert!(output.contains("CONNECTING TO VIDEO SURVEILLANCE DAEMON"));
        assert!(output.contains("CAM 2: HALLWAY"));
        assert!(output.contains("STATUS: OK"));
    }

    #[test]
    fn test_cctv_infection() {
        let mut entity = Entity::new();
        entity.update_depth(30);
        let output = SecurityCameras::view_feeds(&entity, 42);
        assert!(output.contains("YOUR ROOM"));
        assert!(output.contains("YOUR DESK"));
        assert!(output.contains("YOUR EYES"));
        assert!(output.contains("THE DOOR"));
    }
}
