use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// A CCTV network viewer that escalates from normal security feeds to terrifying, voyeuristic angles
pub struct CctvNetwork;

impl CctvNetwork {
    #[must_use]
    pub fn view_cameras(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();
        let mut output = String::new();

        let _ = writeln!(output, "ESTABLISHING VIDEO LINK...\n");

        match layer {
            EscalationLayer::Surface => Self::generate_surface_feeds(&mut output, &mut rng),
            EscalationLayer::Corruption => {
                Self::generate_corruption_feeds(&mut output, &mut rng);
            }
            EscalationLayer::Presence => Self::generate_presence_feeds(&mut output, &mut rng),
            EscalationLayer::Infection => Self::generate_infection_feeds(&mut output, &mut rng),
        }

        output
    }

    fn generate_surface_feeds(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(output, "CAM 01 [FRONT DESK] : ONLINE (EMPTY)");
        let _ = writeln!(output, "CAM 02 [HALLWAY B]  : ONLINE (STATIC)");
        let _ = writeln!(output, "CAM 03 [SERVER RM]  : ONLINE (BLINKING LIGHTS)");
        if rng.gen_bool(0.3) {
            let _ = writeln!(output, "CAM 04 [PARKING]    : MOTION DETECTED (CAT)");
        } else {
            let _ = writeln!(output, "CAM 04 [PARKING]    : OFFLINE");
        }
    }

    fn generate_corruption_feeds(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(output, "CAM 01 [FRONT DESK] : SIGNAL DEGRADED");
        let _ = writeln!(output, "CAM 02 [HALLWAY B]  : SHADOW MOVEMENT DETECTED");
        let _ = writeln!(output, "CAM 03 [SERVER RM]  : FEED LOOPING");
        if rng.gen_bool(0.5) {
            let _ = writeln!(output, "CAM 04 [?????????]  : SOMEONE IS STANDING THERE");
        } else {
            let _ = writeln!(output, "CAM 04 [UNKNOWN]    : PITCH BLACK");
        }
    }

    fn generate_presence_feeds(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(output, "CAM 01 [YOUR ROOM]  : ONLINE");
        let _ = writeln!(output, "CAM 02 [YOUR BACK]  : MOTION DETECTED");
        if rng.gen_bool(0.7) {
            let _ = writeln!(output, "CAM 03 [YOUR SCREEN]: I CAN SEE MYSELF");
        } else {
            let _ = writeln!(output, "CAM 03 [YOUR EYES]  : REFLECTION NOT MATCHING");
        }
        let _ = writeln!(output, "CAM 04 [CLOSET]     : DOOR SLOWLY OPENING");
    }

    fn generate_infection_feeds(output: &mut String, rng: &mut ChaCha8Rng) {
        let eyes = rng.gen_range(2..100);
        let _ = writeln!(output, "CAM 01 [INSIDE]     : {eyes} EYES OPEN");
        let _ = writeln!(output, "CAM 02 [MIRROR]     : HE IS SMILING");
        let _ = writeln!(output, "CAM 03 [FLESH]      : PULSATING");
        let _ = writeln!(output, "CAM 04 [MIND]       : TRANSMITTING FEAR");
    }
}
