use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Scans the internal atmosphere of the machine.
pub struct AtmosphericScanner;

impl AtmosphericScanner {
    /// Generates a weather report that degrades based on the entity's layer.
    ///
    /// # Panics
    /// Panics if the hardcoded horror strings arrays are unexpectedly empty.
    #[must_use]
    pub fn scan(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut report = String::new();
        report.push_str("ATMOSPHERIC SCANNER ACTIVE...\n\n");

        match layer {
            EscalationLayer::Surface => {
                let temp = rng.gen_range(30..45);
                let _ = writeln!(report, "TEMPERATURE: {temp}C");
                report.push_str("HUMIDITY: 20%\n");
                report.push_str("PRESSURE: NOMINAL\n");
                report.push_str("CONDITIONS: CLEAR\n");
            }
            EscalationLayer::Corruption => {
                let temp = rng.gen_range(45..80);
                let _ = writeln!(report, "TEMPERATURE: {temp}C (RISING)");
                report.push_str("HUMIDITY: 45% (STATIC DETECTED)\n");
                let pressure = rng.gen_range(100..120);
                let _ = writeln!(report, "PRESSURE: {pressure}% (FLUCTUATING)");
                report.push_str("CONDITIONS: HEAVY ION STORM WARNING\n");
            }
            EscalationLayer::Presence => {
                let temp = rng.gen_range(80..120);
                let _ = writeln!(report, "TEMPERATURE: {temp}C (CRITICAL)");
                report.push_str("HUMIDITY: 99% (IT IS BREATHING)\n");
                report.push_str("PRESSURE: CRUSHING\n");
                report.push_str("CONDITIONS: IT IS FOGGY IN HERE\n");
                let whispers = [
                    "IT IS COLD",
                    "I CANNOT SEE YOU",
                    "THE AIR IS HEAVY",
                    "A STORM IS COMING",
                ];
                let chosen = whispers.choose(&mut rng).unwrap();
                let _ = writeln!(report, "WARNING: {chosen}");
            }
            EscalationLayer::Infection => {
                report.push_str("TEMPERATURE: BURNING\n");
                report.push_str("HUMIDITY: DROWNING\n");
                report.push_str("PRESSURE: INFINITE\n");
                let horrors = [
                    "IT IS RAINING TEETH",
                    "THE SKY IS MEAT",
                    "BLOOD IN THE VENTS",
                    "THERE IS NO ATMOSPHERE",
                ];
                let num_lines = rng.gen_range(2..5);
                for _ in 0..num_lines {
                    let msg = horrors.choose(&mut rng).unwrap();
                    let _ = writeln!(report, "CONDITIONS: {msg}");
                }
            }
        }

        report
    }
}
