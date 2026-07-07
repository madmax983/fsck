use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

pub struct AsciiRenderer;

impl AsciiRenderer {
    #[must_use]
    pub fn render_view(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::new();
        let _ = writeln!(output, "=== ENVIRONMENTAL SENSORS ===");

        let width = 20;
        let height = 10;

        if layer == EscalationLayer::Infection {
            let _ = writeln!(output, "ERROR: SENSORS COMPROMISED. THEY ARE HERE.");
        } else if layer == EscalationLayer::Presence {
            let _ = writeln!(output, "WARNING: UNKNOWN ENTITY DETECTED.");
        }

        for y in 0..height {
            for x in 0..width {
                if y == 0 || y == height - 1 || x == 0 || x == width - 1 {
                    output.push('#');
                } else {
                    let chance = match layer {
                        EscalationLayer::Surface => 0.01,
                        EscalationLayer::Corruption => 0.05,
                        EscalationLayer::Presence => 0.1,
                        EscalationLayer::Infection => 0.3,
                    };
                    if rng.gen_bool(chance) {
                        let chars = ['%', '&', '?', '!', '@'];
                        output.push(chars[rng.gen_range(0..chars.len())]);
                    } else {
                        output.push(' ');
                    }
                }
            }
            output.push('\n');
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_vision_surface() {
        let entity = Entity::new();
        let output = AsciiRenderer::render_view(&entity, 42);
        assert!(output.contains("==="));
    }

    #[test]
    fn test_vision_infection() {
        let mut entity = Entity::new();
        entity.update_depth(30); // Infection layer
        let output = AsciiRenderer::render_view(&entity, 42);
        assert!(output.contains("ERROR"));
    }
}
