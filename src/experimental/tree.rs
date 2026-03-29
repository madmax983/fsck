use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

use crate::entity::{Entity, EscalationLayer};
use crate::filesystem::FilesystemGraph;

pub struct TreeMapper;

impl TreeMapper {
    /// Generates a visual tree representation of the current directory structure.
    /// Injects corruption and entity whispers based on the `EscalationLayer`.
    #[must_use]
    pub fn generate_tree(fs: &FilesystemGraph, entity: &Entity, base_seed: u64) -> String {
        let mut output = String::new();
        let current_path = fs.current_path();

        let _ = writeln!(output, "DIRECTORY TREE: {current_path}");

        let layer = entity.layer();
        let mut rng = ChaCha8Rng::seed_from_u64(
            base_seed.wrapping_add(u64::from(entity.interaction_count())),
        );

        // Start traversal, but because we can't look deep into petgraph without traversing,
        // we'll hallucinate depth from the current directory.
        Self::traverse_directory(
            fs,
            &mut output,
            "",
            0,
            layer,
            &mut rng,
        );

        output
    }

    fn traverse_directory(
        fs: &FilesystemGraph,
        output: &mut String,
        prefix: &str,
        depth: usize,
        layer: EscalationLayer,
        rng: &mut ChaCha8Rng,
    ) {
        let max_depth = match layer {
            EscalationLayer::Surface => 1,
            EscalationLayer::Corruption => 2,
            EscalationLayer::Presence => 3,
            EscalationLayer::Infection => 5,
        };

        if depth >= max_depth {
            return;
        }

        let node = fs.current_node();
        let dirs = fs.list_directories();
        let files: Vec<_> = node.visible_files().collect();
        let total_items = dirs.len() + files.len();

        // Entity whispers in deep layers
        if depth > 0
            && matches!(
                layer,
                EscalationLayer::Presence | EscalationLayer::Infection
            )
            && rng.gen_bool(0.15)
        {
            let fake_prefix = format!("{prefix}├── ");
            let _ = writeln!(output, "{fake_prefix}{}", Self::whisper(rng));
        }

        let mut current_item = 0;

        for dir_name in &dirs {
            current_item += 1;
            let is_last = current_item == total_items;
            let (node_prefix, child_prefix) = if is_last {
                ("└── ", "    ")
            } else {
                ("├── ", "│   ")
            };

            let display_name = Self::corrupt_if_needed(dir_name, layer, rng);
            let _ = writeln!(output, "{prefix}{node_prefix}{display_name}");

            // Hallucinate deeper structures based on layer
            if depth < max_depth - 1 && matches!(layer, EscalationLayer::Corruption | EscalationLayer::Presence | EscalationLayer::Infection) && rng.gen_bool(0.3) {
                let fake_sub_prefix = format!("{prefix}{child_prefix}");
                Self::hallucinate_branch(output, &fake_sub_prefix, depth + 1, max_depth, layer, rng);
            }
        }

        for file in files {
            current_item += 1;
            let is_last = current_item == total_items;
            let node_prefix = if is_last { "└── " } else { "├── " };

            let display_name = Self::corrupt_if_needed(file.name(), layer, rng);
            let _ = writeln!(output, "{prefix}{node_prefix}{display_name}");
        }
    }

    fn hallucinate_branch(
        output: &mut String,
        prefix: &str,
        depth: usize,
        max_depth: usize,
        layer: EscalationLayer,
        rng: &mut ChaCha8Rng,
    ) {
        if depth >= max_depth {
            return;
        }

        let fake_dirs = ["LOST", "FOUND", "NULL", "VOID", "SECTOR", "MEMORY", "DARK"];
        let fake_files = ["ERROR.TXT", "CORRUPT.TXT", "WHY.TXT", "WHO.TXT", "WATCHING.TXT"];

        let num_items = rng.gen_range(1..=3);
        for i in 1..=num_items {
            let is_last = i == num_items;
            let (node_prefix, child_prefix) = if is_last {
                ("└── ", "    ")
            } else {
                ("├── ", "│   ")
            };

            if rng.gen_bool(0.6) {
                // Directory
                let name = fake_dirs[rng.gen_range(0..fake_dirs.len())];
                let display_name = Self::corrupt_if_needed(name, layer, rng);
                let _ = writeln!(output, "{prefix}{node_prefix}{display_name}");

                if depth < max_depth - 1 && rng.gen_bool(0.4) {
                    Self::hallucinate_branch(
                        output,
                        &format!("{prefix}{child_prefix}"),
                        depth + 1,
                        max_depth,
                        layer,
                        rng,
                    );
                }
            } else {
                // File
                let name = fake_files[rng.gen_range(0..fake_files.len())];
                let display_name = Self::corrupt_if_needed(name, layer, rng);
                let _ = writeln!(output, "{prefix}{node_prefix}{display_name}");
            }
        }
    }

    fn corrupt_if_needed(original: &str, layer: EscalationLayer, rng: &mut ChaCha8Rng) -> String {
        if matches!(
            layer,
            EscalationLayer::Presence | EscalationLayer::Infection
        ) && rng.gen_bool(0.1)
        {
            let corrupt = ['?', '#', '@', '_', '-', '~', '█', '▓', '▒', '░'];
            let mut result = String::with_capacity(original.len());
            for ch in original.chars() {
                if rng.gen_bool(0.3) {
                    result.push(corrupt[rng.gen_range(0..corrupt.len())]);
                } else {
                    result.push(ch);
                }
            }
            result
        } else {
            original.to_string()
        }
    }

    fn whisper(rng: &mut ChaCha8Rng) -> &'static str {
        let whispers = [
            "???",
            "IT SEES YOU",
            "DONT LOOK DOWN",
            "BEHIND YOU",
            "SECTOR 7",
            "YOU ARE LOST",
            "NOTHING HERE",
            "WHY DID YOU COME BACK",
            "CORRUPTED_SECTOR",
            "████████",
        ];
        whispers[rng.gen_range(0..whispers.len())]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filesystem::FilesystemGenerator;

    #[test]
    fn test_tree_generation_surface() {
        let fs = FilesystemGenerator::generate_with_content(42, 2, None);
        let entity = Entity::new();
        let output = TreeMapper::generate_tree(&fs, &entity, 42);

        assert!(output.contains("DIRECTORY TREE:"));
        // At surface level, no whispers should be present
        assert!(!output.contains("???"));
        assert!(!output.contains("IT SEES YOU"));
    }

    #[test]
    fn test_tree_generation_infection() {
        let fs = FilesystemGenerator::generate_with_content(42, 2, None);
        let mut entity = Entity::new();
        entity.update_depth(30); // Reach Infection layer
        let output = TreeMapper::generate_tree(&fs, &entity, 42);

        assert!(output.contains("DIRECTORY TREE:"));
        // Deep layer should contain some corruption or structure
        assert!(output.contains("├── ") || output.contains("└── "));
    }
}
