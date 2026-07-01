use crate::entity::{Entity, EntityMood, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Scans the system kernel for biometric signs.
pub struct VitalsMonitor;

impl VitalsMonitor {
    #[must_use]
    pub fn scan_vitals(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();
        let mood = entity.current_mood();
        let mut output = String::new();

        let _ = writeln!(output, "INITIATING KERNEL BIOMETRIC SCAN...");

        let (bpm_min, bpm_max, temp_min, temp_max, resp_options, status) = match layer {
            EscalationLayer::Surface => (0, 0, 30.0, 60.0, &["NONE", "N/A"][..], "INANIMATE"),
            EscalationLayer::Corruption => {
                (10, 30, 70.0, 90.0, &["FAINT", "SHALLOW"][..], "WARMING")
            }
            EscalationLayer::Presence => (
                40,
                120,
                95.0,
                101.0,
                &["ERRATIC", "LABORED", "HEAVY"][..],
                "FEVERISH",
            ),
            EscalationLayer::Infection => (
                150,
                250,
                105.0,
                120.0,
                &["RAPID", "CHOKING", "GASPING"][..],
                "CRITICAL",
            ),
        };

        let bpm = if bpm_max > bpm_min {
            rng.gen_range(bpm_min..=bpm_max)
        } else {
            0
        };
        let temp = rng.gen_range(temp_min..temp_max);
        let resp = resp_options[rng.gen_range(0..resp_options.len())];

        if layer == EscalationLayer::Surface {
            let _ = writeln!(output, "TARGET: SYSTEM KERNEL");
            let _ = writeln!(output, "BPM: 0");
        } else {
            let _ = writeln!(output, "TARGET: UNKNOWN BIOLOGICAL ENTITY");
            let _ = writeln!(output, "BPM: {bpm}");
        }
        let _ = writeln!(output, "TEMP: {temp:.1}C");
        let _ = writeln!(output, "RESPIRATION: {resp}");
        let _ = writeln!(output, "STATUS: {status}");

        match mood {
            EntityMood::Dormant => {}
            EntityMood::Curious => {
                let _ = writeln!(output, "NOTE: PUPILS DILATED");
            }
            EntityMood::Helpful => {
                let _ = writeln!(output, "NOTE: SEROTONIN LEVELS ELEVATED");
            }
            EntityMood::Wounded => {
                let _ = writeln!(output, "WARNING: SEVERE INTERNAL HEMORRHAGING");
            }
            EntityMood::Predatory => {
                let _ = writeln!(output, "WARNING: ADRENALINE SPIKE DETECTED");
            }
            EntityMood::Glitching => {
                let _ = writeln!(output, "CRITICAL: MULTIPLE ORGAN FAILURE");
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_vitals_surface() {
        let entity = Entity::new();
        let scan = VitalsMonitor::scan_vitals(&entity, 42);
        assert!(scan.contains("INITIATING KERNEL BIOMETRIC SCAN"));
        assert!(scan.contains("INANIMATE"));
    }

    #[test]
    fn test_vitals_infection() {
        let mut entity = Entity::new();
        entity.update_depth(30);
        let scan = VitalsMonitor::scan_vitals(&entity, 42);
        assert!(scan.contains("UNKNOWN BIOLOGICAL ENTITY"));
        assert!(scan.contains("CRITICAL"));
    }
}
