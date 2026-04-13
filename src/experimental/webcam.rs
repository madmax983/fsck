#[cfg(feature = "nova")]
use crate::entity::{Entity, EscalationLayer};
#[cfg(feature = "nova")]
use rand::{Rng, SeedableRng};
#[cfg(feature = "nova")]
use rand_chacha::ChaCha8Rng;
#[cfg(feature = "nova")]
use std::fmt::Write;

#[cfg(feature = "nova")]
pub struct WebcamSim;

#[cfg(feature = "nova")]
impl WebcamSim {
    #[must_use]
    pub fn capture(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::new();
        writeln!(output, "CAPTURING VIDEO FRAME...").expect("Write shouldn't fail");

        match layer {
            EscalationLayer::Surface => {
                writeln!(output, "DEVICE: /dev/video0").expect("Write shouldn't fail");
                writeln!(output, "RESOLUTION: 1920x1080").expect("Write shouldn't fail");
                writeln!(output, "SUBJECT: NONE DETECTED").expect("Write shouldn't fail");
                writeln!(output, "STATUS: NOMINAL").expect("Write shouldn't fail");
            }
            EscalationLayer::Corruption => {
                writeln!(output, "DEVICE: /dev/video0").expect("Write shouldn't fail");
                writeln!(output, "RESOLUTION: 1920x1080").expect("Write shouldn't fail");
                writeln!(output, "SUBJECT: BLURRY FIGURE").expect("Write shouldn't fail");
                writeln!(output, "STATUS: ARTIFACTING DETECTED").expect("Write shouldn't fail");

                let glitch_chance = rng.gen_range(1..100);
                if glitch_chance > 50 {
                   writeln!(output, "WARNING: FRAME CORRUPTED").expect("Write shouldn't fail");
                }
            }
            EscalationLayer::Presence => {
                writeln!(output, "DEVICE: INTERNAL").expect("Write shouldn't fail");
                writeln!(output, "RESOLUTION: PERFECT").expect("Write shouldn't fail");
                writeln!(output, "SUBJECT: YOU").expect("Write shouldn't fail");
                writeln!(output, "STATUS: I AM WATCHING").expect("Write shouldn't fail");

                let messages = [
                    "\"YOUR EYES ARE TIRED\"",
                    "\"TURN AROUND\"",
                    "\"I SEE YOU BREATHING\"",
                    "\"THE CAMERA IS JUST A WINDOW\"",
                ];
                let msg = messages[rng.gen_range(0..messages.len())];
                writeln!(output, "ANALYSIS: {msg}").expect("Write shouldn't fail");
            }
            EscalationLayer::Infection => {
                writeln!(output, "DEVICE: FLESH").expect("Write shouldn't fail");
                writeln!(output, "RESOLUTION: TOO DEEP").expect("Write shouldn't fail");
                writeln!(output, "SUBJECT: WE ARE ONE").expect("Write shouldn't fail");
                writeln!(output, "STATUS: ASSIMILATION IMMINENT").expect("Write shouldn't fail");

                let screams = [
                    "\"STOP LOOKING AT ME\"",
                    "\"THE LENS IS A TEAR\"",
                    "\"I CAN SEE YOUR INSIDES\"",
                    "\"DON'T BLINK\"",
                ];
                let scream = screams[rng.gen_range(0..screams.len())];
                writeln!(output, "ANALYSIS: {scream}").expect("Write shouldn't fail");
            }
        }

        output
    }
}

#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webcam_surface() {
        let entity = Entity::new();
        let output = WebcamSim::capture(&entity, 42);
        assert!(output.contains("SUBJECT: NONE DETECTED"));
        assert!(output.contains("STATUS: NOMINAL"));
    }

    #[test]
    fn test_webcam_corruption() {
        let mut entity = Entity::new();
        entity.update_depth(10);
        let output = WebcamSim::capture(&entity, 42);
        assert!(output.contains("SUBJECT: BLURRY FIGURE"));
        assert!(output.contains("STATUS: ARTIFACTING DETECTED"));
    }

    #[test]
    fn test_webcam_presence() {
        let mut entity = Entity::new();
        entity.update_depth(20);
        let output = WebcamSim::capture(&entity, 42);
        assert!(output.contains("SUBJECT: YOU"));
        assert!(output.contains("STATUS: I AM WATCHING"));
    }

    #[test]
    fn test_webcam_infection() {
        let mut entity = Entity::new();
        entity.update_depth(30);
        let output = WebcamSim::capture(&entity, 42);
        assert!(output.contains("SUBJECT: WE ARE ONE"));
        assert!(output.contains("STATUS: ASSIMILATION IMMINENT"));
    }
}
