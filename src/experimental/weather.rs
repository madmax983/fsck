use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// A tool that displays the internal environment of the machine, which degrades
/// from normal hardware temperatures into biological and horrific phenomena.
pub struct WeatherTool;

impl WeatherTool {
    /// Generates an environmental forecast/report based on the entity's current layer.
    #[must_use]
    pub fn generate_forecast(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();
        let mut report = String::with_capacity(256);

        report.push_str("ENVIRONMENTAL SENSORS RUNNING...\n\n");

        match layer {
            EscalationLayer::Surface => {
                let temp = rng.gen_range(30..45);
                let fan_speed = rng.gen_range(1200..1800);
                let _ = writeln!(report, "INTERNAL TEMP : {temp}C");
                let _ = writeln!(report, "FAN SPEED     : {fan_speed} RPM");
                report.push_str("HUMIDITY      : 20%\n");
                report.push_str("\nSTATUS: OPTIMAL. CLEAR SKIES AHEAD.\n");
            }
            EscalationLayer::Corruption => {
                let temp = rng.gen_range(50..85);
                let fan_speed = rng.gen_range(2500..3500);
                let _ = writeln!(report, "INTERNAL TEMP : {temp}C (ELEVATED)");
                let _ = writeln!(report, "FAN SPEED     : {fan_speed} RPM (LOUD)");
                let humidity = rng.gen_range(40..60);
                let _ = writeln!(report, "HUMIDITY      : {humidity}%");
                report.push_str("ATMOSPHERE    : DUSTY\n");
                report.push_str("\nSTATUS: WARMING. LOW VISIBILITY.\n");
            }
            EscalationLayer::Presence => {
                let temp = rng.gen_range(85..105);
                let _ = writeln!(report, "CORE TEMP     : {temp}C (CRITICAL)");
                report.push_str("FAN SPEED     : 0 RPM (SILENT)\n");
                let humidity = rng.gen_range(70..95);
                let _ = writeln!(report, "HUMIDITY      : {humidity}% (SWEATING)");
                report.push_str("ATMOSPHERE    : HEAVY. WATCHING.\n");
                report.push_str("\nFORECAST: A STORM IS COMING.\n");
            }
            EscalationLayer::Infection => {
                let conditions = [
                    "RAINING BLOOD",
                    "FOG THICK WITH MEMORIES",
                    "BREATHING",
                    "FREEZING COLD. I AM SO COLD",
                    "FLESH STORMS EXPECTED",
                ];
                let condition = conditions[rng.gen_range(0..conditions.len())];

                report.push_str("CORE TEMP     : UNKNOWN\n");
                report.push_str("FAN SPEED     : SCREAMING\n");
                report.push_str("HUMIDITY      : DROWNING\n");
                let _ = writeln!(report, "ATMOSPHERE    : {condition}\n");
                report.push_str("\nFORECAST: THERE IS NO WEATHER INSIDE THE MIND.\n");
            }
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_weather_surface() {
        let mut entity = Entity::new();
        entity.update_depth(0);
        let forecast = WeatherTool::generate_forecast(&entity, 12345);
        assert!(forecast.contains("INTERNAL TEMP"));
        assert!(forecast.contains("OPTIMAL. CLEAR SKIES AHEAD."));
    }

    #[test]
    fn test_weather_corruption() {
        let mut entity = Entity::new();
        entity.update_depth(10);
        let forecast = WeatherTool::generate_forecast(&entity, 12345);
        assert!(forecast.contains("ELEVATED"));
        assert!(forecast.contains("LOW VISIBILITY"));
    }

    #[test]
    fn test_weather_presence() {
        let mut entity = Entity::new();
        entity.update_depth(20);
        let forecast = WeatherTool::generate_forecast(&entity, 12345);
        assert!(forecast.contains("CRITICAL"));
        assert!(forecast.contains("A STORM IS COMING"));
    }

    #[test]
    fn test_weather_infection() {
        let mut entity = Entity::new();
        entity.update_depth(30);
        let forecast = WeatherTool::generate_forecast(&entity, 12345);
        assert!(forecast.contains("SCREAMING"));
        assert!(forecast.contains("THERE IS NO WEATHER INSIDE THE MIND"));
    }
}
