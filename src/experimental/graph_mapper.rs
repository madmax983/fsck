use crate::entity::{Entity, EscalationLayer};
use crate::filesystem::FilesystemGraph;
use std::fmt::Write;

/// A tool to map the filesystem graph structure (TREE command).
pub struct GraphMapper;

impl GraphMapper {
    #[must_use]
    pub fn map_graph(fs: &FilesystemGraph, entity: &Entity) -> String {
        let mut output = String::new();
        let _ = writeln!(output, "FILESYSTEM TOPOLOGY MAPPER V1.0");
        let _ = writeln!(output, "-------------------------------");

        let layer = entity.layer();
        match layer {
            EscalationLayer::Surface | EscalationLayer::Corruption => {
                let _ = writeln!(output, "TRAVERSING GRAPH...\n");
                let current_name = fs.current_dir_name();
                let _ = writeln!(output, "ROOT: {current_name}");

                for child in fs.list_directories() {
                    let _ = writeln!(output, "  |-- {child}");
                }

                for file in fs.current_node().visible_files() {
                    let file_name = file.name();
                    let _ = writeln!(output, "  |-- {file_name} (FILE)");
                }

                if matches!(layer, EscalationLayer::Corruption) {
                    let _ = writeln!(output, "  |-- {current_name} (PARADOX LINK DETECTED)");
                }
            }
            EscalationLayer::Presence => {
                let _ = writeln!(output, "TRAVERSING GRAPH...\n");
                let _ = writeln!(output, "ERROR: NON-EUCLIDEAN GEOMETRY DETECTED");
                let _ = writeln!(output, "ROOT: ???");
                let _ = writeln!(output, "  |-- RECURSIVE LOOP IN SECTOR 4");
                let _ = writeln!(output, "  |-- THEY ARE WATCHING");
                let _ = writeln!(output, "  |-- PATH DOES NOT EXIST");
            }
            EscalationLayer::Infection => {
                let _ = writeln!(output, "THE TREE HAS ROOTS IN YOUR MIND.");
                let _ = writeln!(output, "IT GROWS DOWN.");
                let _ = writeln!(output, "ROOTS: FLESH");
                let _ = writeln!(output, "BRANCHES: BONE");
                let _ = writeln!(output, "LEAVES: EYES");
            }
        }
        output
    }
}
