use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Simulates a webcam capture that degrades into horror as the player descends.
pub struct WebcamSimulator;

impl WebcamSimulator {
    #[must_use]
    pub fn capture(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::new();
        let _ = writeln!(output, "INITIALIZING WEBCAM...");

        match layer {
            EscalationLayer::Surface => Self::generate_surface_capture(&mut output, &mut rng),
            EscalationLayer::Corruption => Self::generate_corruption_capture(&mut output, &mut rng),
            EscalationLayer::Presence => Self::generate_presence_capture(&mut output, &mut rng),
            EscalationLayer::Infection => Self::generate_infection_capture(&mut output, &mut rng),
        }

        output
    }

    fn generate_surface_capture(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(output, "CAPTURING IMAGE...");
        if rng.gen_bool(0.5) {
            let _ = writeln!(output, "[ASCII_ART: A BLURRY ROOM]");
        } else {
            let _ = writeln!(output, "[ASCII_ART: STATIC]");
        }
    }

    fn generate_corruption_capture(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(output, "C@PTUR1NG IM@G#...");
        if rng.gen_bool(0.5) {
            let _ = writeln!(output, "[ASCII_ART: A DARK CORNER. SOMETHING MOVED.]");
        } else {
            let _ = writeln!(output, "[ASCII_ART: GLITCHED DATA STREAM]");
        }
    }

    fn generate_presence_capture(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(output, "I SEE YOU");
        if rng.gen_bool(0.5) {
            let _ = writeln!(output, "[ASCII_ART: A PAIR OF EYES STARING BACK]");
        } else {
            let _ = writeln!(output, "[ASCII_ART: YOUR OWN FACE, DISTORTED]");
        }
    }

    fn generate_infection_capture(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(output, "I SEE YOU");
        let _ = writeln!(output, "WE ARE WATCHING");
        if rng.gen_bool(0.5) {
            let _ = writeln!(output, "[ASCII_ART: FLESH AND TEETH]");
        } else {
            let _ = writeln!(output, "[ASCII_ART: THE VOID]");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_webcam_surface() {
        let entity = Entity::new();
        // Run multiple times to catch random branches without flaking
        let mut success = false;
        for i in 0..10 {
            let output = WebcamSimulator::capture(&entity, 42 + i);
            if output.contains("INITIALIZING WEBCAM")
                && output.contains("ASCII_ART")
                && !output.contains("EYE")
            {
                success = true;
                break;
            }
        }
        assert!(success);
    }

    #[test]
    fn test_webcam_infection() {
        let mut entity = Entity::new();
        entity.update_depth(30); // Infection layer
        let mut success = false;
        for i in 0..10 {
            let output = WebcamSimulator::capture(&entity, 42 + i);
            if output.contains("I SEE YOU") {
                success = true;
                break;
            }
        }
        assert!(success);
    }
}
