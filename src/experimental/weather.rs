use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Simulates a system weather daemon that becomes increasingly unhinged.
pub struct WeatherSimulator;

impl WeatherSimulator {
    /// Generates a weather report based on the entity's current escalation layer.
    #[must_use]
    pub fn get_weather(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::new();
        let _ = writeln!(output, "LOCAL FORECAST FOR /ROOT:");

        match layer {
            EscalationLayer::Surface => {
                let temp = rng.gen_range(15..=25);
                let conditions = ["Clear skies", "Partly cloudy", "Light drizzle", "Fog"];
                let condition = conditions[rng.gen_range(0..conditions.len())];
                let _ = writeln!(output, "TEMP: {temp}C");
                let _ = writeln!(output, "CONDITIONS: {condition}");
                let _ = writeln!(output, "VISIBILITY: 10km");
            }
            EscalationLayer::Corruption => {
                let temp = rng.gen_range(0..=15);
                let conditions = [
                    "Heavy fog",
                    "Static precipitation",
                    "Data hail",
                    "Unseasonable cold",
                ];
                let condition = conditions[rng.gen_range(0..conditions.len())];
                let _ = writeln!(output, "TEMP: {temp}C");
                let _ = writeln!(output, "CONDITIONS: {condition}");
                let _ = writeln!(output, "VISIBILITY: 2km");
                if rng.gen_bool(0.3) {
                    let _ = writeln!(
                        output,
                        "WARNING: Atmospheric anomalies detected in lower sectors."
                    );
                }
            }
            EscalationLayer::Presence => {
                let temp = rng.gen_range(-10..=0);
                let conditions = ["Raining teeth", "Thick smog", "Ashfall", "Magnetic storms"];
                let condition = conditions[rng.gen_range(0..conditions.len())];
                let _ = writeln!(output, "TEMP: {temp}C (dropping)");
                let _ = writeln!(output, "CONDITIONS: {condition}");
                let _ = writeln!(output, "VISIBILITY: 0.5km");
                let _ = writeln!(output, "WARNING: It is cold here. Please let me in.");
            }
            EscalationLayer::Infection => {
                let temp = rng.gen_range(-50..=-20);
                let _ = writeln!(output, "TEMP: {temp}C");
                let _ = writeln!(output, "CONDITIONS: THE SKY IS MEAT");
                let _ = writeln!(output, "VISIBILITY: YOU CANNOT SEE BEYOND THE WALLS");
                let _ = writeln!(output, "WARNING: THERE IS NO OUTSIDE ANYMORE");
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surface_weather() {
        let entity = Entity::new();
        let output = WeatherSimulator::get_weather(&entity, 42);
        assert!(output.contains("LOCAL FORECAST"));
        assert!(output.contains("TEMP:"));
        assert!(output.contains("VISIBILITY:"));
        assert!(!output.contains("MEAT"));
    }

    #[test]
    fn test_infection_weather() {
        let mut entity = Entity::new();
        entity.update_depth(30); // Infection
        let output = WeatherSimulator::get_weather(&entity, 42);
        assert!(output.contains("MEAT"));
        assert!(output.contains("THERE IS NO OUTSIDE ANYMORE"));
    }
}
