use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Simulates atmospheric interference and digital weather patterns
/// within the terminal environment.
pub struct AtmosphericInterference;

impl AtmosphericInterference {
    /// Generates a localized weather report that degrades as the entity descends.
    #[must_use]
    pub fn generate_weather(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::with_capacity(512);
        let _ = writeln!(output, "LOCALIZED ATMOSPHERIC SCAN...");

        let temp = rng.gen_range(50.0..95.0);
        let humidity = rng.gen_range(20.0..80.0);

        match layer {
            EscalationLayer::Surface => {
                let _ = writeln!(output, "TEMP: {temp:.1} F");
                let _ = writeln!(output, "HUMIDITY: {humidity:.1}%");
                let _ = writeln!(output, "CONDITIONS: CLEAR. MINIMAL STATIC.");
            }
            EscalationLayer::Corruption => {
                let _ = writeln!(output, "TEMP: {temp:.1} F [SENSOR DRIFT DETECTED]");
                let _ = writeln!(output, "HUMIDITY: {humidity:.1}%");
                let _ = writeln!(output, "CONDITIONS: HEAVY STATIC. DIGITAL DRIZZLE.");
                Self::generate_static(&mut output, &mut rng, 3, 40);
            }
            EscalationLayer::Presence => {
                let temp_drop = temp - 30.0;
                let _ = writeln!(output, "TEMP: {temp_drop:.1} F [RAPID COOLING]");
                let _ = writeln!(output, "HUMIDITY: 99.9% [DAMP WALLS]");
                let _ = writeln!(output, "CONDITIONS: THEY ARE IN THE RAIN.");
                Self::generate_static(&mut output, &mut rng, 5, 50);
            }
            EscalationLayer::Infection => {
                let temp_err = rng.gen_range(-999.0..999.0);
                let _ = writeln!(output, "TEMP: {temp_err:.1} F");
                let _ = writeln!(output, "HUMIDITY: 100% [LIQUID DATA]");
                let _ = writeln!(output, "CONDITIONS: █▀▄█ THE STORM IS INSIDE █▄▀█");
                Self::generate_static(&mut output, &mut rng, 8, 60);
            }
        }

        output
    }

    fn generate_static(output: &mut String, rng: &mut ChaCha8Rng, lines: usize, width: usize) {
        let chars = ['.', ',', '\'', '`', '~', '*', '\\', '/', '|', '?', 'X'];

        for _ in 0..lines {
            for _ in 0..width {
                if rng.gen_bool(0.3) {
                    let c = chars[rng.gen_range(0..chars.len())];
                    output.push(c);
                } else {
                    output.push(' ');
                }
            }
            output.push('\n');
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_surface_weather() {
        let entity = Entity::new();
        let output = AtmosphericInterference::generate_weather(&entity, 42);
        assert!(output.contains("CLEAR. MINIMAL STATIC."));
        assert!(output.contains("TEMP:"));
    }

    #[test]
    fn test_infection_weather() {
        let mut entity = Entity::new();
        entity.add_depth(30); // Reach infection layer
        let output = AtmosphericInterference::generate_weather(&entity, 42);
        assert!(output.contains("THE STORM IS INSIDE"));
        assert!(output.contains("█▀▄█"));
    }
}
