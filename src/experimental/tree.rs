use crate::entity::{Entity, EscalationLayer};
use crate::filesystem::FilesystemGraph;
use petgraph::graph::NodeIndex;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

pub struct TreeVisualizer;

impl TreeVisualizer {
    #[must_use]
    pub fn generate_tree(fs: &FilesystemGraph, entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::with_capacity(512);
        let start_idx = fs.current_index();
        let current_dir_name = fs.current_dir_name();

        let _ = writeln!(output, "{current_dir_name}");

        let max_depth = match layer {
            EscalationLayer::Surface => 3,
            EscalationLayer::Corruption => 4,
            EscalationLayer::Presence => 5,
            EscalationLayer::Infection => 6,
        };

        Self::print_tree_recursive(
            fs,
            start_idx,
            &mut output,
            "",
            0,
            max_depth,
            layer,
            &mut rng,
        );

        output
    }

    #[allow(clippy::too_many_arguments)]
    fn print_tree_recursive(
        fs: &FilesystemGraph,
        node_idx: NodeIndex,
        output: &mut String,
        prefix: &str,
        depth: u32,
        max_depth: u32,
        layer: EscalationLayer,
        rng: &mut ChaCha8Rng,
    ) {
        if depth >= max_depth {
            return;
        }

        let dir_node = fs.node(node_idx);
        let mut file_names: Vec<String> = dir_node
            .visible_files()
            .map(|f| f.name().to_string())
            .collect();

        if matches!(
            layer,
            EscalationLayer::Presence | EscalationLayer::Infection
        ) && rng.gen_bool(0.3)
        {
            let hallucinations = [
                "I_AM_HERE.txt",
                "DONT_LOOK.sys",
                "bleeding.log",
                "you_are_lost.cfg",
                "MEMORIES.dmp",
            ];
            file_names.push(hallucinations[rng.gen_range(0..hallucinations.len())].to_string());
        }

        let walker = fs.list_directory_indices_from(node_idx);
        let mut subdirs = Vec::new();
        for idx in walker {
            subdirs.push(idx);
        }

        let files_len = file_names.len();
        let is_loop = matches!(layer, EscalationLayer::Infection) && rng.gen_bool(0.1);

        for (i, file_name) in file_names.iter_mut().enumerate() {
            let is_last = i == files_len - 1 && subdirs.is_empty() && !is_loop;
            let marker = if is_last { "└── " } else { "├── " };

            if matches!(
                layer,
                EscalationLayer::Corruption
                    | EscalationLayer::Presence
                    | EscalationLayer::Infection
            ) && rng.gen_bool(0.1)
                && !file_name.is_empty()
            {
                let chars: Vec<char> = file_name.chars().collect();
                let flip_idx = rng.gen_range(0..chars.len());
                let mut new_chars = chars.clone();
                new_chars[flip_idx] = '?';
                *file_name = new_chars.into_iter().collect();
            }

            let _ = writeln!(output, "{prefix}{marker}{file_name}");
        }

        for (i, &subdir_idx) in subdirs.iter().enumerate() {
            let is_last = i == subdirs.len() - 1 && !is_loop;
            let marker = if is_last { "└── " } else { "├── " };
            let new_prefix = format!("{prefix}{}", if is_last { "    " } else { "│   " });

            let dir_name = if matches!(layer, EscalationLayer::Infection) && rng.gen_bool(0.2) {
                "ERROR_NO_DIR".to_string()
            } else {
                fs.node(subdir_idx).name().to_string()
            };

            let _ = writeln!(output, "{prefix}{marker}{dir_name}");
            Self::print_tree_recursive(
                fs,
                subdir_idx,
                output,
                &new_prefix,
                depth + 1,
                max_depth,
                layer,
                rng,
            );
        }

        if is_loop {
            let marker = "└── ";
            let _ = writeln!(output, "{prefix}{marker}... [RECURSION DETECTED] ...");
        }
    }
}
