use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Simulates a local weather system that becomes increasingly horrific as depth increases.
pub struct WeatherSimulator;

impl WeatherSimulator {
    #[must_use]
    pub fn generate_forecast(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::with_capacity(256);
        let _ = writeln!(output, "LOCAL WEATHER FORECAST:");

        match layer {
            EscalationLayer::Surface => Self::generate_surface_weather(&mut output, &mut rng),
            EscalationLayer::Corruption => Self::generate_corruption_weather(&mut output, &mut rng),
            EscalationLayer::Presence => Self::generate_presence_weather(&mut output, &mut rng),
            EscalationLayer::Infection => Self::generate_infection_weather(&mut output, &mut rng),
        }

        output
    }

    fn generate_surface_weather(output: &mut String, rng: &mut ChaCha8Rng) {
        let conditions = ["CLEAR SKIES", "PARTLY CLOUDY", "LIGHT RAIN"];
        let temp = rng.gen_range(65..80);
        let chosen = conditions[rng.gen_range(0..conditions.len())];
        let _ = writeln!(output, "TEMP: {temp}F, {chosen}");
    }

    fn generate_corruption_weather(output: &mut String, rng: &mut ChaCha8Rng) {
        let conditions = ["HEAVY FOG", "UNSEASONABLE COLD", "STATIC IN THE AIR"];
        let temp = rng.gen_range(40..60);
        let chosen = conditions[rng.gen_range(0..conditions.len())];
        let _ = writeln!(output, "TEMP: {temp}F, {chosen}");
    }

    fn generate_presence_weather(output: &mut String, rng: &mut ChaCha8Rng) {
        let conditions = ["THE WIND SOUNDS LIKE VOICES", "THUNDER BUT NO CLOUDS", "IT IS DARK NOW"];
        let temp = rng.gen_range(20..40);
        let chosen = conditions[rng.gen_range(0..conditions.len())];
        let _ = writeln!(output, "TEMP: {temp}F, {chosen}");
    }

    fn generate_infection_weather(output: &mut String, rng: &mut ChaCha8Rng) {
        let conditions = ["BLOOD RAIN", "THE WINDOWS ARE BREATHING", "ACID FOG", "TEETH FALLING LIKE HAIL"];
        let temp = rng.gen_range(100..120);
        let chosen = conditions[rng.gen_range(0..conditions.len())];
        let _ = writeln!(output, "TEMP: {temp}F, {chosen}");
    }
}
