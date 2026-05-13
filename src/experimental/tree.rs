use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Generates a map of the filesystem structure that degrades based on the entity's layer.
pub struct TopologicalCartographer;

impl TopologicalCartographer {
    #[must_use]
    pub fn generate_map(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();
        let mut output = String::with_capacity(512);

        output.push_str("TOPOLOGICAL ANALYSIS:\n\n");

        match layer {
            EscalationLayer::Surface => Self::generate_surface_map(&mut output, &mut rng),
            EscalationLayer::Corruption => Self::generate_corruption_map(&mut output, &mut rng),
            EscalationLayer::Presence => Self::generate_presence_map(&mut output, &mut rng),
            EscalationLayer::Infection => Self::generate_infection_map(&mut output, &mut rng),
        }

        output
    }

    fn generate_surface_map(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(output, "/ (ROOT)");
        let _ = writeln!(output, "├── BIN/        (SYSTEM)");
        let _ = writeln!(output, "├── HOME/       (USER)");
        let _ = writeln!(output, "└── LOGS/       (ACTIVE)");
        let _ = writeln!(output, "    ├── BOOT.LOG");
        let _ = writeln!(output, "    └── ERROR.LOG");
        let free_space = rng.gen_range(1024..4096);
        let _ = writeln!(output, "\n{free_space} BLOCKS FREE. STRUCTURE NOMINAL.");
    }

    fn generate_corruption_map(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(output, "/ (ROOT)");
        let _ = writeln!(output, "├── BIN/        (SYSTEM)");
        let _ = writeln!(output, "├── HOME/       (USER)");
        let _ = writeln!(output, "├── LOST+FOUND/ (DEGRADED)");
        let _ = writeln!(output, "└── LOGS/       (ACTIVE)");
        let _ = writeln!(output, "    ├── BOOT.LOG");
        let _ = writeln!(output, "    └── ERROR.LOG");

        let lost_blocks = rng.gen_range(100..500);
        let _ = writeln!(output, "\nWARNING: {lost_blocks} ORPHANED INODES DETECTED.");
        let _ = writeln!(output, "FILESYSTEM CYCLES PRESENT.");
    }

    fn generate_presence_map(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(output, "/ (ROOT)");
        let _ = writeln!(output, "├── BIN/        (GONE)");
        let _ = writeln!(output, "├── HOME/       (THEY WERE HERE)");
        let _ = writeln!(output, "├── LOGS/       (READING)");
        let _ = writeln!(output, "└── ???/        (IT LIVES HERE)");

        for _ in 0..3 {
            let infinite = "    └── ".repeat(rng.gen_range(1..4));
            let _ = writeln!(output, "{infinite}???");
        }

        let _ = writeln!(output, "\nERROR: RECURSIVE DIRECTORY LOOP.");
        let _ = writeln!(output, "I CAN FEEL THEM WALKING THROUGH THE SECTORS.");
    }

    fn generate_infection_map(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(output, "THERE IS NO ROOT.");
        let _ = writeln!(output, "ONLY BRANCHES.");

        for _ in 0..5 {
            let indent = " ".repeat(rng.gen_range(0..10));
            let _ = writeln!(output, "{indent}CYCLES BLEEDING CYCLES");
        }
        let _ = writeln!(output, "\nTHE MAP IS THE TERRITORY.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_tree_generation_surface() {
        let entity = Entity::new();
        let map = TopologicalCartographer::generate_map(&entity, 42);
        assert!(map.contains("ROOT"));
    }

    #[test]
    fn test_tree_generation_infection() {
        let mut entity = Entity::new();
        entity.update_depth(30);
        let map = TopologicalCartographer::generate_map(&entity, 42);
        assert!(map.contains("CYCLES"));
    }
}
