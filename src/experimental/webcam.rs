use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

pub struct WebcamViewer;

impl WebcamViewer {
    #[must_use]
    pub fn capture(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();
        let mut output = String::new();

        output.push_str("INITIALIZING /DEV/VIDEO0...\n");
        output.push_str("CAPTURING FRAME...\n\n");

        match layer {
            EscalationLayer::Surface => Self::generate_surface_frame(&mut output, &mut rng),
            EscalationLayer::Corruption => Self::generate_corruption_frame(&mut output, &mut rng),
            EscalationLayer::Presence => Self::generate_presence_frame(&mut output, &mut rng),
            EscalationLayer::Infection => Self::generate_infection_frame(&mut output, &mut rng),
        }

        output
    }

    fn generate_surface_frame(output: &mut String, rng: &mut ChaCha8Rng) {
        if rng.gen_bool(0.7) {
            output.push_str("[ NO SIGNAL ]\n");
            output.push_str("CAMERA NOT DETECTED OR POWERED OFF\n");
        } else {
            output.push_str("........................\n");
            output.push_str("......#........#........\n");
            output.push_str("...#####......#####.....\n");
            output.push_str("........................\n");
            output.push_str("EMPTY ROOM DETECTED\n");
        }
    }

    fn generate_corruption_frame(output: &mut String, rng: &mut ChaCha8Rng) {
        output.push_str("▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒\n");
        if rng.gen_bool(0.5) {
            output.push_str("▒▒▒#▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒\n");
        } else {
            output.push_str("▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒#▒▒▒▒▒▒▒▒\n");
        }
        output.push_str("▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒\n");
        output.push_str("WARN: MOVEMENT DETECTED IN FRAME\n");
    }

    fn generate_presence_frame(output: &mut String, rng: &mut ChaCha8Rng) {
        output.push_str("█▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀█\n");
        output.push_str("█                      █\n");
        if rng.gen_bool(0.3) {
            output.push_str("█       (0)  (0)       █\n");
        } else {
            output.push_str("█                      █\n");
        }
        output.push_str("█                      █\n");
        output.push_str("█▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄█\n");
        output.push_str("OBSERVATION IN PROGRESS. PLEASE HOLD STILL.\n");
    }

    fn generate_infection_frame(output: &mut String, _rng: &mut ChaCha8Rng) {
        output.push_str("👁️  IT SEES YOU\n");
        output.push_str("WE ARE LOOKING AT EACH OTHER\n");
        output.push_str("THE FLESH BEHIND THE GLASS IS FRAGILE\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_webcam_surface() {
        let entity = Entity::new();
        let output = WebcamViewer::capture(&entity, 42);
        assert!(output.contains("NO SIGNAL"));
    }

    #[test]
    fn test_webcam_infection() {
        let mut entity = Entity::new();
        entity.update_depth(30);
        let output = WebcamViewer::capture(&entity, 42);
        assert!(output.contains("IT SEES YOU"));
    }
}
