use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// A weather forecast simulator that devolves into anomalous phenomena
pub struct WeatherService;

impl WeatherService {
    #[must_use]
    pub fn get_forecast(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::with_capacity(256);
        let _ = writeln!(output, "CONNECTING TO REGIONAL METEOROLOGICAL DATABASE...");

        match layer {
            EscalationLayer::Surface => Self::generate_surface(&mut output, &mut rng),
            EscalationLayer::Corruption => Self::generate_corruption(&mut output, &mut rng),
            EscalationLayer::Presence => Self::generate_presence(&mut output, &mut rng),
            EscalationLayer::Infection => Self::generate_infection(&mut output, &mut rng),
        }

        output
    }

    fn generate_surface(output: &mut String, rng: &mut ChaCha8Rng) {
        let conditions = ["CLEAR", "PARTLY CLOUDY", "LIGHT RAIN", "OVERCAST"];
        let temp = rng.gen_range(10..35);
        let condition = conditions[rng.gen_range(0..conditions.len())];
        let _ = writeln!(output, "CURRENT CONDITIONS : {condition}");
        let _ = writeln!(output, "TEMPERATURE        : {temp}C");
        let _ = writeln!(output, "WIND               : {} KM/H", rng.gen_range(5..25));
    }

    fn generate_corruption(output: &mut String, rng: &mut ChaCha8Rng) {
        let conditions = ["HEAVY STATIC", "ASH", "UNSEASONABLE COLD", "YELLOW FOG"];
        let temp = rng.gen_range(-10..55);
        let condition = conditions[rng.gen_range(0..conditions.len())];
        let _ = writeln!(output, "CURRENT CONDITIONS : {condition}");
        let _ = writeln!(output, "TEMPERATURE        : {temp}C (ERR)");
        let _ = writeln!(output, "VISIBILITY         : COMPROMISED");
    }

    fn generate_presence(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(output, "CURRENT CONDITIONS : THEY ARE OUTSIDE");
        let _ = writeln!(output, "TEMPERATURE        : COLD AS THE GRAVE");
        if rng.gen_bool(0.5) {
            let _ = writeln!(output, "FORECAST           : DO NOT LOOK OUT THE WINDOW");
        } else {
            let _ = writeln!(output, "FORECAST           : THE FOG IS KNOCKING");
        }
    }

    fn generate_infection(output: &mut String, rng: &mut ChaCha8Rng) {
        let conditions = [
            "RAINING TEETH",
            "BLOOD",
            "THE SKY IS FALLING",
            "FLESH TORNADO",
        ];
        let condition = conditions[rng.gen_range(0..conditions.len())];
        let _ = writeln!(output, "CURRENT CONDITIONS : {condition}");
        let _ = writeln!(output, "BAROMETRIC PRESSURE: CRUSHING");
        let _ = writeln!(output, "FORECAST           : THERE IS NO OUTSIDE ANYMORE");
    }
}
