#[cfg(feature = "nova")]
use crate::entity::{Entity, EscalationLayer};
#[cfg(feature = "nova")]
use crate::filesystem::FilesystemGraph;
#[cfg(feature = "nova")]
use rand::prelude::*;
#[cfg(feature = "nova")]
use rand_chacha::ChaCha8Rng;
#[cfg(feature = "nova")]
use std::fmt::Write;

/// Mangles filesystem data to give the user an abstract, corrupted map
#[cfg(feature = "nova")]
pub struct Cartographer;

#[cfg(feature = "nova")]
impl Cartographer {
    #[must_use]
    pub fn map(fs: &FilesystemGraph, entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::new();
        let _ = writeln!(output, "MAPPING LOCAL SECTOR...");

        let current_dir = fs.current_dir_name();
        let depth = fs.current_depth();

        match layer {
            EscalationLayer::Surface => {
                let _ = writeln!(output, "LOC: {current_dir} (DEPTH: {depth})");
            }
            EscalationLayer::Corruption => {
                if rng.gen_bool(0.3) {
                    let _ = writeln!(output, "LOC: UNKNOWN (DEPTH: ?)");
                } else {
                    let _ = writeln!(output, "LOC: {current_dir} (DEPTH: {depth})");
                }
            }
            EscalationLayer::Presence => {
                let _ = writeln!(output, "LOC: I_AM_HERE (DEPTH: DEEPER)");
            }
            EscalationLayer::Infection => {
                let _ = writeln!(output, "LOC: NO_ESCAPE (DEPTH: INFINITE)");
                return output;
            }
        }

        let mut files = Vec::new();
        for file in fs.current_node().visible_files() {
            files.push(file.name().to_string());
        }

        let mut dirs = Vec::new();
        for dir in fs.list_directories() {
            dirs.push(dir.to_string());
        }

        if dirs.is_empty() && files.is_empty() {
            let _ = writeln!(output, "  [EMPTY SECTOR]");
        } else {
            for dir in dirs {
                let _ = writeln!(output, "  + {dir}");
            }
            for file in files {
                let _ = writeln!(output, "  - {file}");
            }
        }

        if matches!(layer, EscalationLayer::Presence) && rng.gen_bool(0.5) {
            let _ = writeln!(output, "  ? SOMETHING IS WATCHING YOU");
        }

        output
    }
}

#[cfg(all(test, feature = "nova"))]
mod tests {
    use super::*;
    use crate::entity::Entity;
    use crate::filesystem::FilesystemGraph;

    #[test]
    fn test_map_surface() {
        let mut fs = FilesystemGraph::new();
        fs.add_child("TEST_DIR");
        fs.change_dir("TEST_DIR", 0, 0.0).unwrap();
        let entity = Entity::new();
        let map_output = Cartographer::map(&fs, &entity, 42);
        assert!(map_output.contains("TEST_DIR"));
        assert!(map_output.contains("DEPTH: 1"));
    }

    #[test]
    fn test_map_infection() {
        let fs = FilesystemGraph::new();
        let mut entity = Entity::new();
        entity.add_depth(30); // Infection
        let map_output = Cartographer::map(&fs, &entity, 42);
        assert!(map_output.contains("NO_ESCAPE"));
    }
}
