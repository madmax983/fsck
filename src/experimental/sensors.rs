use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// A sensor tool that degrades from hardware diagnostics into horrifying biological metrics
pub struct HardwareSensors;

impl HardwareSensors {
    #[must_use]
    pub fn get_readings(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();
        let mut output = String::new();

        let _ = writeln!(output, "QUERYING SENSORS...\n");

        match layer {
            EscalationLayer::Surface => {
                let temp = rng.gen_range(30.0..45.0);
                let fan = rng.gen_range(1200..1800);
                let _ = writeln!(output, "CPU_TEMP     : {temp:.1}C");
                let _ = writeln!(output, "SYS_FAN      : {fan} RPM");
                let _ = writeln!(output, "VOLTAGE      : 5.02V");
                let _ = writeln!(output, "DISK_SEEK    : 12ms");
            }
            EscalationLayer::Corruption => {
                let temp = rng.gen_range(45.0..85.0);
                let fan = rng.gen_range(2000..4500);
                let err_volts = rng.gen_range(4.1..5.9);
                let _ = writeln!(output, "CPU_TEMP     : {temp:.1}C");
                let _ = writeln!(output, "SYS_FAN      : {fan} RPM");
                let _ = writeln!(output, "VOLTAGE      : {err_volts:.2}V (WARN)");
                let _ = writeln!(output, "VIBRATION    : DETECTED");
            }
            EscalationLayer::Presence => {
                let room_temp = rng.gen_range(15.0..22.0);
                let _ = writeln!(output, "ROOM_TEMP    : {room_temp:.1}C");
                let _ = writeln!(output, "PROXIMITY    : 0.5m");
                if rng.gen_bool(0.5) {
                    let _ = writeln!(output, "BREATHING    : TRUE");
                } else {
                    let _ = writeln!(output, "PULSE        : STEADY");
                }
                let _ = writeln!(output, "OBSERVATION  : RECIPROCAL");
            }
            EscalationLayer::Infection => {
                let hr = rng.gen_range(80..140);
                let _ = writeln!(output, "HEARTBEAT    : {hr} BPM");
                let _ = writeln!(output, "EYE_CONTACT_SEC: {}", rng.gen_range(10..300));
                let _ = writeln!(output, "FEAR_LEVEL   : RISING");
                let _ = writeln!(output, "DOORS_LOCKED : TRUE");
            }
        }

        output
    }
}
