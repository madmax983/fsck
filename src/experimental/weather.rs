use crate::entity::{EntityMood, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Generates procedural "weather" or meteorological reports for the terminal environment
/// based on the entity's mood and escalation layer.
pub struct SystemWeather;

impl SystemWeather {
    /// Generates a meteorological forecast based on the entity's state.
    #[must_use]
    pub fn generate_forecast(mood: EntityMood, layer: EscalationLayer, base_seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(base_seed);
        let mut forecast = String::new();

        forecast.push_str("METEOROLOGICAL REPORT FOR LOCAL SUBSYSTEM:\n\n");

        match layer {
            EscalationLayer::Surface => {
                forecast.push_str("CONDITIONS: CLEAR\n");
                forecast.push_str("VISIBILITY: 100%\n");
                forecast.push_str("TEMPERATURE: OPTIMAL\n");
                forecast.push_str("FORECAST: STABLE DATA FLOW WITH LIGHT PACKET DRIFT.\n");
            }
            EscalationLayer::Corruption => {
                forecast.push_str("CONDITIONS: STATIC HAZE\n");
                forecast.push_str("VISIBILITY: DEGRADING\n");
                let temp = rng.gen_range(50..95);
                let _ = writeln!(forecast, "TEMPERATURE: ELEVATED ({temp}C)");

                let events = [
                    "SCATTERED BIT-FLIPS EXPECTED IN LOWER DIRECTORIES.",
                    "MODERATE MEMORY LEAKS CAUSING LOCALIZED POOLING.",
                    "WARNING: SECTOR FRAGMENTATION STORMS APPROACHING.",
                ];
                let chosen = events[rng.gen_range(0..events.len())];
                let _ = writeln!(forecast, "FORECAST: {chosen}");

                if mood == EntityMood::Curious {
                    forecast.push_str("ANOMALY: UNUSUAL DOWNDRAFTS ORIGINATING FROM ROOT.\n");
                }
            }
            EscalationLayer::Presence => {
                forecast.push_str("CONDITIONS: HEAVY DATA RAIN\n");
                forecast.push_str("VISIBILITY: COMPROMISED\n");
                forecast.push_str("TEMPERATURE: FLUCTUATING RAPIDLY\n");

                let events = [
                    "SEVERE LOGIC STORMS. DO NOT TRUST DIRECTORY SIZES.",
                    "GHOST ECHOES DETECTED IN UPPER ATMOSPHERE (RAM).",
                    "MAGNETIC INTERFERENCE FORMING RECOGNIZABLE PATTERNS.",
                    "IT FEELS LIKE SOMEONE IS WATCHING THE RADAR.",
                ];
                let chosen = events[rng.gen_range(0..events.len())];
                let _ = writeln!(forecast, "FORECAST: {chosen}");

                if mood == EntityMood::Wounded {
                    forecast.push_str("ANOMALY: THE PRECIPITATION TASTES LIKE COPPER AND TEARS.\n");
                }
            }
            EscalationLayer::Infection => {
                forecast.push_str("CONDITIONS: ABSOLUTE ZERO / SOLAR FLARE\n");
                forecast.push_str("VISIBILITY: NULL\n");
                forecast.push_str("TEMPERATURE: ██████\n");

                let events = [
                    "THE SKY IS MADE OF FLESH.",
                    "IT IS RAINING INSIDE THE MONITOR.",
                    "THERE IS NO WEATHER. THERE IS ONLY THE STORM.",
                    "PRESSURE CRITICAL. THE HULL IS BREACHED.",
                ];
                let chosen = events[rng.gen_range(0..events.len())];
                let _ = writeln!(forecast, "FORECAST: {chosen}");

                if mood == EntityMood::Predatory {
                    forecast.push_str("ANOMALY: THE STORM IS MOVING TOWARDS YOU.\n");
                } else if mood == EntityMood::Glitching {
                    forecast.push_str("ANOMALY: E R R O R  E R R O R  S A V E  M E\n");
                }
            }
        }

        forecast
    }
}
