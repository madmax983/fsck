use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// A CCTV Security Feed Simulator that degrades from mundane security feeds into unsettling observations.
pub struct CctvSystem;

impl CctvSystem {
    #[must_use]
    pub fn get_feed(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();
        let mut output = String::new();

        let _ = writeln!(output, "ACCESSING SECURITY FEED...\n");
        let _ = writeln!(output, "------------------------");

        match layer {
            EscalationLayer::Surface => Self::generate_surface_feed(&mut output, &mut rng),
            EscalationLayer::Corruption => Self::generate_corruption_feed(&mut output, &mut rng),
            EscalationLayer::Presence => Self::generate_presence_feed(&mut output, &mut rng),
            EscalationLayer::Infection => Self::generate_infection_feed(&mut output, &mut rng),
        }

        let _ = writeln!(output, "------------------------");
        output
    }

    fn generate_surface_feed(output: &mut String, rng: &mut ChaCha8Rng) {
        let statuses = ["ONLINE ", "OFFLINE", "STATIC "];
        for i in 1..=4 {
            let status = statuses[rng.gen_range(0..statuses.len())];
            let _ = writeln!(output, "CAM {i:02} - LOBBY    : [{status}]");
        }
    }

    fn generate_corruption_feed(output: &mut String, _rng: &mut ChaCha8Rng) {
        let _ = writeln!(output, "CAM 01 - LOBBY    : [ONLINE ]");
        let _ = writeln!(output, "CAM 02 - HALLWAY  : [STATIC ] - MOTION DETECTED");
        let _ = writeln!(output, "CAM 03 - BASEMENT : [OFFLINE]");
        let _ = writeln!(output, "CAM 04 - ALLEY    : [ONLINE ] - SHADOWS MOVING");
    }

    fn generate_presence_feed(output: &mut String, rng: &mut ChaCha8Rng) {
        let messages = [
            "SOMEONE IS THERE",
            "BREATHING ON LENS",
            "WATCHING YOU",
            "LOOKING BACK",
            "OBSERVATION: RECIPROCAL",
        ];

        for i in 1..=4 {
            let msg = messages[rng.gen_range(0..messages.len())];
            let _ = writeln!(output, "CAM {i:02} - UNKNOWN  : [{msg}]");
        }
    }

    fn generate_infection_feed(output: &mut String, rng: &mut ChaCha8Rng) {
        let horros = [
            "FLESH ON LENS",
            "TOO MANY EYES",
            "TEETH GRINDING",
            "BLEEDING PIXELS",
            "IT SEES YOU",
        ];

        for i in 1..=4 {
            let msg = horros[rng.gen_range(0..horros.len())];
            let _ = writeln!(output, "CAM {i:02} - INSIDE   : [{msg}]");
        }
    }
}
