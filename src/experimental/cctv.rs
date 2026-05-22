use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// A security camera viewer that shows a feed that degrades into horror.
pub struct CctvViewer;

impl CctvViewer {
    #[must_use]
    pub fn view(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::with_capacity(512);

        match layer {
            EscalationLayer::Surface => {
                let _ = writeln!(output, "CAMERA 01 - SECURE");
                let _ = writeln!(output, "==================");
                let _ = writeln!(output, "      [ ]         ");
                let _ = writeln!(output, "==================");
                let _ = writeln!(output, "STATUS: ALL CLEAR");
            }
            EscalationLayer::Corruption => {
                let _ = writeln!(output, "CAMERA 01 - SIGNAL DEGRADED");
                let _ = writeln!(output, "==================");
                let static_char = if rng.gen_bool(0.5) { '#' } else { '%' };
                let _ = writeln!(output, "      [{static_char}]         ");
                let _ = writeln!(output, "==================");
                let _ = writeln!(output, "STATUS: UNKNOWN INTERFERENCE");
            }
            EscalationLayer::Presence => {
                let _ = writeln!(output, "CAMERA 01 - IT IS WATCHING");
                let _ = writeln!(output, "==================");
                let _ = writeln!(output, "      [@]         ");
                let _ = writeln!(output, "==================");
                let _ = writeln!(output, "STATUS: PRESENCE DETECTED");
            }
            EscalationLayer::Infection => {
                let _ = writeln!(output, "CAMERA 01 - TURN IT OFF");
                let _ = writeln!(output, "==================");
                let _ = writeln!(output, "      [X]         ");
                let _ = writeln!(output, "==================");
                let _ = writeln!(output, "STATUS: TURN IT OFF TURN IT OFF");
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_cctv_surface() {
        let entity = Entity::new();
        let output = CctvViewer::view(&entity, 12345);
        assert!(output.contains("CAMERA 01 - SECURE"));
        assert!(output.contains("[ ]"));
    }

    #[test]
    fn test_cctv_corruption() {
        let mut entity = Entity::new();
        entity.update_depth(15);
        let output = CctvViewer::view(&entity, 12345);
        assert!(output.contains("SIGNAL DEGRADED"));
        // output contains [#] or [%]
        assert!(output.contains("[#]") || output.contains("[%]"));
    }

    #[test]
    fn test_cctv_presence() {
        let mut entity = Entity::new();
        entity.update_depth(20);
        let output = CctvViewer::view(&entity, 12345);
        assert!(output.contains("IT IS WATCHING"));
        assert!(output.contains("[@]"));
    }

    #[test]
    fn test_cctv_infection() {
        let mut entity = Entity::new();
        entity.update_depth(30);
        let output = CctvViewer::view(&entity, 12345);
        assert!(output.contains("TURN IT OFF"));
        assert!(output.contains("[X]"));
    }
}
