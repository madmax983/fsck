use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Generates system sensor readouts (temperature, fan speed, voltage)
/// that degrade and become unsettling as the player descends.
pub struct HardwareSensors;

impl HardwareSensors {
    /// Reads hardware sensors based on the entity's current layer.
    #[must_use]
    pub fn read_sensors(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();
        let mut report = String::new();

        report.push_str("HARDWARE SENSOR READOUT\n=======================\n");

        match layer {
            EscalationLayer::Surface => {
                let temp = rng.gen_range(35.0..45.0);
                let fan = rng.gen_range(1200..1800);
                let volt = rng.gen_range(4.9..5.1);

                let _ = writeln!(report, "CPU TEMP:  {temp:.1} C");
                let _ = writeln!(report, "FAN SPEED: {fan} RPM");
                let _ = writeln!(report, "VOLTAGE:   {volt:.2} V");
            }
            EscalationLayer::Corruption => {
                let temp = rng.gen_range(50.0..85.0);
                let fan = rng.gen_range(2500..4000);
                let volt = rng.gen_range(4.5..5.5);

                let _ = writeln!(report, "CPU TEMP:  {temp:.1} C (WARN)");
                let _ = writeln!(report, "FAN SPEED: {fan} RPM (HIGH)");
                let _ = writeln!(report, "VOLTAGE:   {volt:.2} V (FLUX)");
            }
            EscalationLayer::Presence => {
                let temp = rng.gen_range(90.0..120.0);
                let volt = rng.gen_range(3.0..7.0);

                let _ = writeln!(report, "CPU TEMP:  {temp:.1} C (TOO HOT)");
                let _ = writeln!(report, "FAN SPEED: ERR_NO_RESPONSE");
                let _ = writeln!(report, "VOLTAGE:   {volt:.2} V (CRITICAL)");
                report.push_str("\nWARNING: THERMAL THROTTLING DISABLED.\n");
            }
            EscalationLayer::Infection => {
                let corruptions = ["BURNING", "MELTING", "TOO HOT", "FLESH", "SCREAMING"];
                let msg1 = corruptions[rng.gen_range(0..corruptions.len())];
                let msg2 = corruptions[rng.gen_range(0..corruptions.len())];
                let msg3 = corruptions[rng.gen_range(0..corruptions.len())];

                let _ = writeln!(report, "CPU TEMP:  {msg1}");
                let _ = writeln!(report, "FAN SPEED: {msg2}");
                let _ = writeln!(report, "VOLTAGE:   {msg3}");

                report.push_str("\nSYS_HALT: THE SILICON IS BOILING.\n");
            }
        }

        report
    }
}
