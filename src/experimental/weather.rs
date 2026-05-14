use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// A weather station that analyzes the entity's layer to generate meteorological reports
pub struct WeatherStation;

impl WeatherStation {
    #[must_use]
    pub fn generate_report(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let mut report = String::with_capacity(128);

        let _ = writeln!(report, "METEOROLOGICAL REPORT:");

        match entity.layer() {
            EscalationLayer::Surface => {
                let temp = rng.gen_range(15..=25);
                let _ = writeln!(report, "CONDITIONS: CLEAR");
                let _ = writeln!(report, "TEMPERATURE: {temp}°C");
                let _ = writeln!(report, "WIND: CALM");
            }
            EscalationLayer::Corruption => {
                let conditions = if rng.gen_bool(0.5) {
                    "ACID RAIN"
                } else {
                    "FOG"
                };
                let temp = rng.gen_range(30..=45);
                let _ = writeln!(report, "CONDITIONS: {conditions}");
                let _ = writeln!(report, "TEMPERATURE: {temp}°C (RISING)");
                let _ = writeln!(report, "WIND: ERRATIC");
            }
            EscalationLayer::Presence => {
                let conditions = if rng.gen_bool(0.5) {
                    "DATA STORMS"
                } else {
                    "VOICES"
                };
                let temp = rng.gen_range(-20..=0);
                let _ = writeln!(report, "CONDITIONS: {conditions}");
                let _ = writeln!(report, "TEMPERATURE: {temp}°C (DROPPING RAPIDLY)");
                let _ = writeln!(report, "WIND: HOWLING");
            }
            EscalationLayer::Infection => {
                let conditions = if rng.gen_bool(0.5) {
                    "IT IS RAINING FLESH"
                } else {
                    "OBLIVION"
                };
                let _ = writeln!(report, "CONDITIONS: {conditions}");
                let _ = writeln!(report, "TEMPERATURE: ABSOLUTE ZERO");
                let _ = writeln!(report, "WIND: DEAFENING SILENCE");
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
    fn test_weather_station_layers() {
        let mut entity = Entity::new();
        let seed = 42;

        // Surface layer
        let report = WeatherStation::generate_report(&entity, seed);
        assert!(report.contains("CLEAR"));

        // Corruption layer
        entity.update_depth(10);
        let report = WeatherStation::generate_report(&entity, seed);
        assert!(report.contains("ACID RAIN") || report.contains("FOG"));

        // Presence layer
        entity.update_depth(20);
        let report = WeatherStation::generate_report(&entity, seed);
        assert!(report.contains("DATA STORMS") || report.contains("VOICES"));

        // Infection layer
        entity.update_depth(30);
        let report = WeatherStation::generate_report(&entity, seed);
        assert!(report.contains("IT IS RAINING FLESH") || report.contains("OBLIVION"));
    }
}
