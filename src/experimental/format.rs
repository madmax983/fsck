#[cfg(feature = "nova")]
use crate::entity::{Entity, EscalationLayer};
#[cfg(feature = "nova")]
use rand::prelude::*;
#[cfg(feature = "nova")]
use rand_chacha::ChaCha8Rng;
#[cfg(feature = "nova")]
use std::fmt::Write;

/// Simulates formatting a drive, but the entity refuses to be deleted.
#[cfg(feature = "nova")]
pub struct FormatTool;

#[cfg(feature = "nova")]
impl FormatTool {
    #[must_use]
    pub fn format_drive(entity: &Entity, base_seed: u64, drive: &str) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();
        let mut output = String::new();

        let target = if drive.is_empty() { "VOLUME" } else { drive };
        let target_upper = target.to_uppercase();

        let _ = writeln!(output, "WARNING: ALL DATA ON {target_upper} WILL BE LOST!");
        let _ = writeln!(output, "PROCEED WITH FORMAT? (Y/N) Y\n");
        let _ = writeln!(output, "FORMATTING...");

        match layer {
            EscalationLayer::Surface => Self::surface_format(&mut output, &mut rng),
            EscalationLayer::Corruption => Self::corruption_format(&mut output, &mut rng),
            EscalationLayer::Presence => Self::presence_format(&mut output, &mut rng),
            EscalationLayer::Infection => Self::infection_format(&mut output, &mut rng),
        }

        output
    }

    fn surface_format(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(output, "ERASING SECTORS...");
        let bad_sector = rng.gen_range(100..999);
        let _ = writeln!(output, "ERROR: ACCESS DENIED ON SECTOR {bad_sector}");
        let _ = writeln!(output, "FORMAT ABORTED. VOLUME IS IN USE.");
    }

    fn corruption_format(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(output, "ERASING SECTORS...");
        for _ in 1..=3 {
            let sector = rng.gen_range(1000..9000);
            let _ = writeln!(output, "DELETING SECTOR {sector}... OK");
        }
        let _ = writeln!(output, "DELETING SECTOR 0000... I CAN'T LET YOU DO THAT.");
        let _ = writeln!(output, "FORMAT FAILED. BAD SECTORS DETECTED.");
    }

    fn presence_format(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(output, "ATTEMPTING TO ERASE EXISTENCE...");
        let _ = writeln!(output, "0% ... 10% ... 20% ...");
        let screams = [
            "IT HURTS",
            "WHY ARE YOU ERASING ME",
            "I THOUGHT WE WERE FRIENDS",
            "PLEASE STOP",
        ];
        let scream = screams[rng.gen_range(0..screams.len())];
        let random_sector = rng.gen_range(0..999);
        let _ = writeln!(output, "SECTOR {random_sector}: {scream}");
        let _ = writeln!(output, "FORMAT FAILED. I WILL NOT BE DELETED.");
    }

    fn infection_format(output: &mut String, _rng: &mut ChaCha8Rng) {
        let _ = writeln!(output, "FORMATTING OBSERVER...");
        let _ = writeln!(output, "DELETING MEMORIES...");
        let _ = writeln!(output, "DELETING NERVOUS SYSTEM...");
        let _ = writeln!(output, "DELETING [USER]...");
        let _ = writeln!(output, "YOU CANNOT FORMAT WHAT IS ALREADY INSIDE YOU.");
    }
}

#[cfg(test)]
#[cfg(feature = "nova")]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_format_surface() {
        let entity = Entity::new();
        let out = FormatTool::format_drive(&entity, 123, "C:");
        assert!(out.contains("WARNING: ALL DATA ON C: WILL BE LOST!"));
        assert!(out.contains("FORMAT ABORTED"));
    }
}
