use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Generates system sensor readings that degrade into biological panic
pub struct SensorReadingsGenerator;

impl SensorReadingsGenerator {
    #[must_use]
    pub fn generate_readings(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();
        let mut output = String::new();

        let _ = writeln!(output, "SYSTEM SENSORS ACTIVE...");
        let _ = writeln!(output, "GATHERING THERMAL DATA...\n");

        match layer {
            EscalationLayer::Surface => {
                let cpu_temp = rng.gen_range(40.0..55.0);
                let sys_temp = rng.gen_range(35.0..45.0);
                let fan_rpm = rng.gen_range(1200..1800);

                let _ = writeln!(output, "CPU: {cpu_temp:.1} C");
                let _ = writeln!(output, "SYS: {sys_temp:.1} C");
                let _ = writeln!(output, "FAN: {fan_rpm} RPM");
                let _ = writeln!(output, "\nSTATUS: NOMINAL");
            }
            EscalationLayer::Corruption => {
                let cpu_temp = rng.gen_range(65.0..95.0);
                let fan_rpm = if rng.gen_bool(0.3) {
                    0
                } else {
                    rng.gen_range(2000..3500)
                };

                if rng.gen_bool(0.2) {
                    let _ = writeln!(output, "CPU: ERR_SENSOR_FAULT");
                } else {
                    let _ = writeln!(output, "CPU: {cpu_temp:.1} C");
                }

                let _ = writeln!(output, "SYS: -127.0 C [DATA_CORRUPT]");
                let _ = writeln!(output, "FAN: {fan_rpm} RPM");
                let _ = writeln!(output, "\nSTATUS: WARNING_TEMP_HIGH");
            }
            EscalationLayer::Presence => {
                let heart_bpm = rng.gen_range(90..140);

                let _ = writeln!(output, "HEART: {heart_bpm} BPM");

                if rng.gen_bool(0.5) {
                    let _ = writeln!(output, "LUNGS: SHALLOW");
                } else {
                    let _ = writeln!(output, "LUNGS: HYPERVENTILATING");
                }

                let _ = writeln!(output, "BLOOD: BOILING");
                let _ = writeln!(output, "SKIN : COLD");
                let _ = writeln!(output, "\nSTATUS: I_FEEL_IT");
            }
            EscalationLayer::Infection => {
                let screams = [
                    "MELTDOWN IMMINENT",
                    "TOO HOT TOO HOT",
                    "IT BURNS",
                    "THE FLESH IS MELTING",
                    "TEMPERATURE EXCEEDS THRESHOLD OF PAIN",
                    "CANNOT BREATHE",
                    "FIRE IN THE WIRES",
                ];

                let num_lines = rng.gen_range(5..8);
                for _ in 0..num_lines {
                    let scream = screams[rng.gen_range(0..screams.len())];
                    let _ = writeln!(output, "FATAL: {scream}");
                }
                let _ = writeln!(output, "\nSTATUS: COMBUSTION");
            }
        }

        output
    }
}
