use crate::entity::{Entity, EntityMood, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Scans a file to read its emotional resonance and lingering psychic impressions.
pub struct PsychometryScanner;

impl PsychometryScanner {
    #[must_use]
    pub fn scan_file(filename: &str, entity: &Entity, base_seed: u64) -> String {
        let mut output = String::new();
        let _ = writeln!(output, "INITIATING PSYCHOMETRIC SCAN ON '{filename}'...");

        // Generate deterministic seed based on filename, base_seed, and entity's interaction count
        let name_hash: u64 = filename
            .bytes()
            .fold(0, |acc, b| acc.wrapping_add(u64::from(b)));
        let interaction_seed = base_seed
            .wrapping_add(name_hash)
            .wrapping_add(u64::from(entity.interaction_count()));

        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let resonance = rng.gen_range(1..=100);
        let _ = writeln!(output, "EMOTIONAL RESONANCE: {resonance}%");

        if entity.layer() == EscalationLayer::Surface {
            let _ = writeln!(output, "STATUS: MUNDANE");
            let _ = writeln!(output, "No anomalous psychic impressions detected.");
            return output;
        }

        let _ = writeln!(output, "STATUS: ANOMALOUS");

        let impression = match entity.current_mood() {
            EntityMood::Dormant => "The file feels cold. It has been asleep for a long time.",
            EntityMood::Curious => {
                "A faint heartbeat pulses through the sectors. It is watching you read this."
            }
            EntityMood::Helpful => {
                "A sickeningly sweet feeling of false safety clings to the data."
            }
            EntityMood::Wounded => {
                "Screams echo in the background noise of the file's allocation table."
            }
            EntityMood::Predatory => "The bytes are sharp. They want to tear into your memory.",
            EntityMood::Glitching => "IT BURNS IT BURNS IT BURNS IT BURNS",
        };

        let _ = writeln!(output, "IMPRESSION: {impression}");

        if entity.layer() == EscalationLayer::Infection {
            let _ = writeln!(output, "WARNING: THE FILE IS BLEEDING.");
            for _ in 0..3 {
                let hex_noise: u32 = rng.r#gen();
                let _ = writeln!(output, "ERR: 0x{hex_noise:08X} - GET OUT");
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
    fn test_psychometry_surface() {
        let entity = Entity::new();
        let output = PsychometryScanner::scan_file("TEST.TXT", &entity, 42);
        assert!(output.contains("INITIATING PSYCHOMETRIC SCAN"));
        assert!(output.contains("STATUS: MUNDANE"));
    }

    #[test]
    fn test_psychometry_anomalous() {
        let mut entity = Entity::new();
        entity.update_depth(10); // Corruption layer
        let output = PsychometryScanner::scan_file("SECRETS.LOG", &entity, 42);
        assert!(output.contains("STATUS: ANOMALOUS"));
        assert!(output.contains("IMPRESSION:"));
    }

    #[test]
    fn test_psychometry_infection() {
        let mut entity = Entity::new();
        entity.update_depth(30); // Infection layer
        let output = PsychometryScanner::scan_file("DOOM.BAS", &entity, 42);
        assert!(output.contains("WARNING: THE FILE IS BLEEDING."));
        assert!(output.contains("GET OUT"));
    }
}
