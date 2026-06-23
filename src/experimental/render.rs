use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// A tool to procedurally render a file's content as an ASCII image.
pub struct AsciiRenderer;

impl AsciiRenderer {
    #[must_use]
    pub fn render(content: &str, entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));

        // Hash the content to make the image deterministic based on file contents
        let mut content_hash: u64 = 0;
        for byte in content.bytes() {
            content_hash = content_hash.wrapping_mul(31).wrapping_add(u64::from(byte));
        }

        let combined_seed = interaction_seed.wrapping_add(content_hash);
        let mut rng = ChaCha8Rng::seed_from_u64(combined_seed);
        let layer = entity.layer();

        let mut output = String::new();
        let _ = writeln!(output, "RENDERING FILE AS ASCII...\n");

        let width = 32;
        let height = 16;

        for y in 0..height {
            for x in 0..width {
                let char_to_print = Self::get_pixel_character(x, y, width, height, layer, &mut rng);
                output.push(char_to_print);
            }
            output.push('\n');
        }

        output
    }

    #[allow(clippy::cast_precision_loss)]
    fn get_pixel_character(
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        layer: EscalationLayer,
        rng: &mut ChaCha8Rng,
    ) -> char {
        // Base procedural shape generation
        let dx = (x as f32 - width as f32 / 2.0) / (width as f32 / 2.0);
        let dy = (y as f32 - height as f32 / 2.0) / (height as f32 / 2.0);
        let dist = dx.hypot(dy);

        let mut c = if dist < rng.gen_range(0.4..0.8) {
            let chars = ['#', '*', '+', '=', '-', '.', ' '];
            chars[rng.gen_range(0..chars.len())]
        } else {
            ' '
        };

        match layer {
            EscalationLayer::Surface => {}
            EscalationLayer::Corruption => {
                if rng.gen_bool(0.1) {
                    let corrupt_chars = ['?', '%', '$', '&'];
                    c = corrupt_chars[rng.gen_range(0..corrupt_chars.len())];
                }
            }
            EscalationLayer::Presence => {
                // Eyes
                if (x == width / 3 || x == 2 * width / 3) && y == height / 3 {
                    c = '@';
                } else if rng.gen_bool(0.15) {
                    c = '~';
                }
            }
            EscalationLayer::Infection => {
                if dist < 0.3 {
                    let horrors = ['X', '0', '8', 'V', 'M'];
                    c = horrors[rng.gen_range(0..horrors.len())];
                } else if rng.gen_bool(0.2) {
                    let static_chars = ['|', '\\', '/', '_'];
                    c = static_chars[rng.gen_range(0..static_chars.len())];
                }
            }
        }
        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_surface_render() {
        let entity = Entity::new();
        let output = AsciiRenderer::render("hello", &entity, 42);
        assert!(output.contains("RENDERING FILE"));
        assert!(!output.contains('@')); // No eyes on surface
    }

    #[test]
    fn test_presence_render() {
        let mut entity = Entity::new();
        entity.update_depth(20); // Presence layer
        let output = AsciiRenderer::render("hello", &entity, 42);
        assert!(output.contains('@')); // Eyes should appear
    }

    #[test]
    fn test_deterministic_render() {
        let entity = Entity::new();
        let output1 = AsciiRenderer::render("hello", &entity, 42);
        let output2 = AsciiRenderer::render("hello", &entity, 42);
        assert_eq!(output1, output2);

        let output3 = AsciiRenderer::render("world", &entity, 42);
        assert_ne!(output1, output3); // Different content = different hash
    }
}
