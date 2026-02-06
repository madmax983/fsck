use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use super::graph::FilesystemGraph;
use super::node::FileNode;
use crate::content::{ContentLibrary, DynamicContent, Era};

/// `Apple IIe` era directory names
const DIR_NAMES: &[&str] = &[
    "GAMES", "DOCS", "SYSTEM", "BASIC", "DATA", "PROGS", "UTIL", "BACKUP", "OLD", "NEW", "TEMP",
    "WORK", "FILES", "STUFF", "MISC", "ARCHIVE", "DONT", "VOID", "EMPTY", "LOST", "FOUND", "ERROR",
    "NULL", "DARK",
];

/// Directory names that should contain themselves (paradoxes)
const PARADOX_NAMES: &[&str] = &["VOID", "LOOP", "STRANGE", "DARK", "ERROR", "NULL"];

pub struct FilesystemGenerator;

impl FilesystemGenerator {
    /// Generates a filesystem with integrated content library.
    ///
    /// Creates a procedurally generated filesystem populated with:
    /// - Static files from the `ContentLibrary`
    /// - Dynamic files (counter, timestamp, corrupted text)
    /// - Victim history files placed at deeper levels
    ///
    /// # Arguments
    /// * `seed` - Random seed for deterministic generation
    /// * `initial_depth` - Maximum depth to generate
    ///
    /// # Returns
    /// A fully populated `FilesystemGraph`
    #[must_use]
    pub fn generate_with_content(seed: u64, initial_depth: u32) -> FilesystemGraph {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut fs = FilesystemGraph::new();
        let library = ContentLibrary::new();

        Self::populate_level_with_content(&mut fs, &mut rng, &library, 0, initial_depth);

        fs
    }

    /// Legacy method for backward compatibility - now uses content library.
    #[must_use]
    pub fn generate(seed: u64, initial_depth: u32) -> FilesystemGraph {
        Self::generate_with_content(seed, initial_depth)
    }

    /// Populates a filesystem level with content from the `ContentLibrary`.
    ///
    /// This method recursively generates directories and files, mixing:
    /// - Static files from the generic file collection
    /// - Dynamic files (counter, timestamp, corrupted text)
    /// - Victim history files (placed deeper in the tree)
    ///
    /// # Arguments
    /// * `fs` - The filesystem graph to populate
    /// * `rng` - Random number generator for deterministic placement
    /// * `library` - Content library with static files and victim histories
    /// * `current_depth` - Current depth in the filesystem tree
    /// * `max_depth` - Maximum depth to generate
    fn populate_level_with_content(
        fs: &mut FilesystemGraph,
        rng: &mut ChaCha8Rng,
        library: &ContentLibrary,
        current_depth: u32,
        max_depth: u32,
    ) {
        if current_depth >= max_depth {
            return;
        }

        // Add 1-4 directories at this level
        let num_dirs = rng.gen_range(1..=4);
        let mut chosen_names: Vec<&str> = Vec::new();

        for _ in 0..num_dirs {
            let name = DIR_NAMES[rng.gen_range(0..DIR_NAMES.len())];
            if !chosen_names.contains(&name) {
                chosen_names.push(name);
                fs.add_child(name);

                // Create paradox for certain directory names
                if PARADOX_NAMES.contains(&name) && rng.gen_bool(0.8) {
                    // 80% chance to make it a paradox
                    if fs.change_dir(name).is_ok() {
                        fs.add_paradox_to_self();
                        let _ = fs.change_dir("..");
                    }
                }
            }
        }

        // Add files - mix of static and dynamic
        let num_files = rng.gen_range(1..=3);
        for _ in 0..num_files {
            let file = if rng.gen_bool(0.3) {
                // 30% chance of dynamic file
                let dynamic = match rng.gen_range(0..3) {
                    0 => DynamicContent::counter("HELLO"),
                    1 => DynamicContent::timestamp(),
                    _ => DynamicContent::corrupted("SYSTEM ERROR", 0.2),
                };
                let random_id = rng.r#gen::<u32>();
                FileNode::with_dynamic(&format!("DYN{random_id}.TXT"), dynamic)
            } else {
                // Static file from library
                let files = library.generic_files();
                let (name, content) = files[rng.gen_range(0..files.len())];
                FileNode::new(name, content)
            };

            fs.current_node_mut().add_file(file);
        }

        // Occasionally place victim history files (deeper = more likely)
        if current_depth >= 3 && rng.gen_bool(0.4) {
            let era = match current_depth {
                3..=8 => Era::Original,
                9..=15 => Era::Technician,
                16..=25 => Era::EstateSale,
                _ => Era::Explorer,
            };

            if let Some(history) = library.history_for_era(era)
                && let Some(entry) = history.entries().first()
            {
                let filename = format!("{}.LOG", history.name());
                let content = format!("{}\n\n{}", entry.date(), entry.content());
                fs.current_node_mut()
                    .add_file(FileNode::new(&filename, &content));
            }
        }

        // Recursively populate children
        let children = fs.list_directories();
        for child_name in children {
            if fs.change_dir(&child_name).is_ok() {
                Self::populate_level_with_content(fs, rng, library, current_depth + 1, max_depth);
                let _ = fs.change_dir("..");
            }
        }
    }
}
