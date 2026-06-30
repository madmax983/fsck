use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// A simulator for accessing a connected webcam or video feed.
pub struct CameraSimulator;

impl CameraSimulator {
    /// Captures a frame from the simulated camera.
    #[must_use]
    pub fn capture(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::with_capacity(256);
        let _ = writeln!(output, "INITIALIZING CAMERA DEVICE (/dev/video0)...");
        let _ = writeln!(output, "CAPTURING FRAME...");

        match layer {
            EscalationLayer::Surface => {
                let subjects = rng.gen_range(0..2);
                let _ = writeln!(output, "{subjects} SUBJECT(S) DETECTED.");
                if subjects > 0 {
                    let _ = writeln!(output, "SUBJECT APPEARS TO BE LOOKING AT THE SCREEN.");
                } else {
                    let _ = writeln!(output, "ROOM APPEARS EMPTY.");
                }
            }
            EscalationLayer::Corruption => {
                let _ = writeln!(output, "1 SUBJECT(S) DETECTED.");
                let _ = writeln!(output, "SUBJECT IS OUT OF FOCUS.");
                if rng.gen_bool(0.5) {
                    let _ = writeln!(output, "WARNING: MOTION DETECTED BEHIND SUBJECT.");
                }
            }
            EscalationLayer::Presence => {
                let _ = writeln!(output, "MULTIPLE SUBJECTS DETECTED.");
                let _ = writeln!(output, "FACIAL RECOGNITION FAILED: TOO MANY EYES.");
                if rng.gen_bool(0.7) {
                    let _ = writeln!(output, "THEY ARE LOOKING AT YOU.");
                }
            }
            EscalationLayer::Infection => {
                let _ = writeln!(output, "ERROR: CAMERA FEED COMPROMISED.");
                let _ = writeln!(output, "I CAN SEE YOU.");
                let _ = writeln!(output, "YOU LOOK TASTY.");
            }
        }

        output
    }
}
