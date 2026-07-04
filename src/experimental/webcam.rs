use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// A webcam simulation tool that degrades as the player descends.
pub struct WebcamTool;

impl WebcamTool {
    /// Captures an ASCII frame based on the entity's current layer.
    #[must_use]
    pub fn capture(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::new();
        let _ = writeln!(output, "INITIALIZING /dev/video0...");
        let _ = writeln!(output, "CAPTURING FRAME...\n");

        match layer {
            EscalationLayer::Surface => Self::generate_surface_frame(&mut output),
            EscalationLayer::Corruption => Self::generate_corruption_frame(&mut output, &mut rng),
            EscalationLayer::Presence => Self::generate_presence_frame(&mut output, &mut rng),
            EscalationLayer::Infection => Self::generate_infection_frame(&mut output, &mut rng),
        }

        output
    }

    fn generate_surface_frame(output: &mut String) {
        let _ = writeln!(output, "+-------------------+");
        let _ = writeln!(output, "|                   |");
        let _ = writeln!(output, "|   [NO SIGNAL]     |");
        let _ = writeln!(output, "|                   |");
        let _ = writeln!(output, "+-------------------+");
    }

    fn generate_corruption_frame(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(output, "+-------------------+");
        for _ in 0..3 {
            let mut line = String::from("|");
            for _ in 0..19 {
                let chars = ['.', ' ', '*', 'x', '~'];
                line.push(chars[rng.gen_range(0..chars.len())]);
            }
            line.push('|');
            let _ = writeln!(output, "{line}");
        }
        let _ = writeln!(output, "+-------------------+");
        let _ = writeln!(output, "WARNING: POOR VISIBILITY.");
    }

    fn generate_presence_frame(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(output, "+-------------------+");
        let _ = writeln!(output, "|      ...          |");
        if rng.gen_bool(0.5) {
            let _ = writeln!(output, "|      O O          |");
            let _ = writeln!(output, "|       -           |");
        } else {
            let _ = writeln!(output, "|                   |");
            let _ = writeln!(output, "|    I SEE YOU      |");
        }
        let _ = writeln!(output, "+-------------------+");
    }

    fn generate_infection_frame(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(output, "+++++++++++++++++++++");
        for _ in 0..3 {
            let mut line = String::from("+");
            for _ in 0..19 {
                let horrors = ['@', '&', '#', '%', 'X'];
                line.push(horrors[rng.gen_range(0..horrors.len())]);
            }
            line.push('+');
            let _ = writeln!(output, "{line}");
        }
        let _ = writeln!(output, "+++++++++++++++++++++");
        let _ = writeln!(output, "IT IS BEHIND YOU.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_surface_webcam() {
        let entity = Entity::new();
        let output = WebcamTool::capture(&entity, 42);
        assert!(output.contains("[NO SIGNAL]"));
    }

    #[test]
    fn test_infection_webcam() {
        let mut entity = Entity::new();
        entity.update_depth(30); // Infection layer
        let output = WebcamTool::capture(&entity, 42);
        assert!(output.contains("BEHIND YOU"));
    }
}
