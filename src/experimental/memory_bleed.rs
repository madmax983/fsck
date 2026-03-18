use crate::entity::{Entity, EscalationLayer};
use crate::experimental::EmotionalBleed;
use crate::filesystem::FilesystemGraph;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// A tool to extract and narrativize victim histories scattered in the filesystem.
/// This acts as a timeline generation utility with increasing distortions based on the layer.
pub struct MemoryBleed;

impl MemoryBleed {
    /// Generates a timeline view of the current node's files.
    #[must_use]
    pub fn generate_timeline(fs: &FilesystemGraph, entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();
        let mut output = String::new();

        let _ = writeln!(output, "--- TIMELINE EXTRACTION ---");

        // Find victim entries in the current node
        // We look for files ending in .TXT or .LOG (generic/victim history)
        let mut found_history = false;

        for file in fs.current_node().visible_files() {
            let name = file.name();
            if std::path::Path::new(name)
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("TXT"))
                || std::path::Path::new(name)
                    .extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("LOG"))
            {
                let content = file.content();
                // Filter out empty lines
                let lines: Vec<&str> = content.lines().filter(|l| !l.trim().is_empty()).collect();
                if !lines.is_empty() {
                    found_history = true;

                    // Display file name as an "event" source
                    let _ = writeln!(output, "\n[SOURCE: {name}]");

                    for (i, line) in lines.iter().enumerate() {
                        // We only show a few lines so it's a "summary timeline"
                        if i > 3 {
                            let _ = writeln!(output, "  ... <more>");
                            break;
                        }

                        let distorted_line = match layer {
                            EscalationLayer::Surface => {
                                // Mostly clean
                                (*line).to_string()
                            }
                            EscalationLayer::Corruption => {
                                // Occasional glitching
                                if rng.gen_bool(0.15) {
                                    EmotionalBleed::inject_emotion(
                                        line,
                                        entity.current_mood(),
                                        &mut rng,
                                    )
                                } else {
                                    (*line).to_string()
                                }
                            }
                            EscalationLayer::Presence => {
                                // More frequent emotional bleed
                                if rng.gen_bool(0.4) {
                                    EmotionalBleed::inject_emotion(
                                        line,
                                        entity.current_mood(),
                                        &mut rng,
                                    )
                                } else {
                                    (*line).to_string()
                                }
                            }
                            EscalationLayer::Infection => {
                                // Almost entirely overwritten by emotional bleed
                                EmotionalBleed::inject_emotion(
                                    line,
                                    entity.current_mood(),
                                    &mut rng,
                                )
                            }
                        };

                        let _ = writeln!(output, "  > {distorted_line}");
                    }
                }
            }
        }

        if !found_history {
            match layer {
                EscalationLayer::Surface => {
                    let _ = writeln!(output, "\nNO HISTORY FOUND IN CURRENT CONTEXT.");
                }
                EscalationLayer::Corruption => {
                    let _ = writeln!(output, "\nMEMORY NOT FOUND.");
                }
                EscalationLayer::Presence => {
                    let _ = writeln!(output, "\nI FORGOT THIS PLACE.");
                }
                EscalationLayer::Infection => {
                    let _ = writeln!(output, "\nALL MEMORIES ARE MINE. NONE ARE HERE.");
                }
            }
        }

        output
    }
}
