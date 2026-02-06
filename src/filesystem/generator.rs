use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use super::graph::FilesystemGraph;
use super::node::FileNode;

/// Apple IIe era directory names
const DIR_NAMES: &[&str] = &[
    "GAMES", "DOCS", "SYSTEM", "BASIC", "DATA", "PROGS", "UTIL", "BACKUP",
    "OLD", "NEW", "TEMP", "WORK", "FILES", "STUFF", "MISC", "ARCHIVE",
    "DONT", "VOID", "EMPTY", "LOST", "FOUND", "ERROR", "NULL", "DARK",
];

/// File names and content templates
const FILE_TEMPLATES: &[(&str, &str)] = &[
    ("README.TXT", "WELCOME TO THE SYSTEM\n"),
    ("NOTES.TXT", "REMEMBER TO BACKUP\n"),
    ("LOG.TXT", "SYSTEM STARTED\n"),
    ("HELLO.BAS", "10 PRINT \"HELLO\"\n20 GOTO 10\n"),
];

pub struct FilesystemGenerator;

impl FilesystemGenerator {
    pub fn generate(seed: u64, initial_depth: u32) -> FilesystemGraph {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut fs = FilesystemGraph::new();

        Self::populate_level(&mut fs, &mut rng, 0, initial_depth);

        fs
    }

    fn populate_level(
        fs: &mut FilesystemGraph,
        rng: &mut ChaCha8Rng,
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
            }
        }

        // Add some files to current directory
        let num_files = rng.gen_range(0..=3);
        for _ in 0..num_files {
            let (name, content) = FILE_TEMPLATES[rng.gen_range(0..FILE_TEMPLATES.len())];
            fs.current_node_mut().add_file(FileNode::new(name, content));
        }

        // Recursively populate children
        let children = fs.list_directories();
        for child_name in children {
            if fs.change_dir(&child_name).is_ok() {
                Self::populate_level(fs, rng, current_depth + 1, max_depth);
                let _ = fs.change_dir("..");
            }
        }
    }
}
