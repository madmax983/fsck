use crate::entity::{Entity, EscalationLayer};
use crate::filesystem::FilesystemGraph;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// A tool to search for text within the current directory's files.
pub struct SearchTool;

impl SearchTool {
    /// Extract a safe snippet from the content avoiding char boundary panics.
    /// Optimized to avoid O(N) heap allocations by iterating over char indices.
    fn extract_snippet(content: &str, start_idx: usize, query_len: usize) -> String {
        let prefix = &content[..start_idx];
        let mut chars_before = 0;
        let mut start_byte = start_idx;
        for (i, _) in prefix.char_indices().rev() {
            start_byte = i;
            chars_before += 1;
            if chars_before == 10 {
                break;
            }
        }

        let query_chars = content[start_idx..]
            .chars()
            .take_while(|c| {
                let mut buf = [0; 4];
                let char_len = c.encode_utf8(&mut buf).len();
                query_len >= char_len
            })
            .count();
        let query_char_len = if query_chars == 0 { 1 } else { query_chars };

        let mut end_byte = start_idx;
        for (chars_after, (i, c)) in content[start_idx..].char_indices().enumerate() {
            if chars_after == query_char_len + 10 {
                break;
            }
            end_byte = start_idx + i + c.len_utf8();
        }

        content[start_byte..end_byte].to_string()
    }

    fn format_clean_snippet(content: &str, idx: usize, query_len: usize) -> String {
        let snippet = Self::extract_snippet(content, idx, query_len);
        let clean_snippet = snippet.replace('\n', " ");
        clean_snippet.trim().to_string()
    }

    fn append_match_output(
        results: &mut String,
        layer: &EscalationLayer,
        content: &str,
        query_upper: &str,
        rng: &mut ChaCha8Rng,
    ) {
        match layer {
            EscalationLayer::Surface => {
                // Show actual snippet if possible, or a generic match string
                if let Some(idx) = content.find(query_upper) {
                    let clean_snippet_trimmed = Self::format_clean_snippet(content, idx, query_upper.len());
                    let _ = writeln!(results, "  ...{clean_snippet_trimmed}...");
                } else {
                    // Shouldn't happen at Surface, but just in case
                    results.push_str("  [MATCH FOUND]\n");
                }
            }
            EscalationLayer::Corruption => {
                if rng.gen_bool(0.3) {
                    results.push_str("  ...[DATA CORRUPTED]...\n");
                } else if let Some(idx) = content.find(query_upper) {
                    let clean_snippet_trimmed = Self::format_clean_snippet(content, idx, query_upper.len());
                    let _ = writeln!(results, "  ...{clean_snippet_trimmed}...");
                } else {
                    results.push_str("  [FALSE POSITIVE DETECTED]\n");
                }
            }
            EscalationLayer::Presence => {
                let creepy_snippets = [
                    "I SEE IT TOO",
                    "WHY ARE YOU LOOKING FOR THIS",
                    "IT'S NOT HERE ANYMORE",
                    "DON'T LOOK",
                    "I HID IT",
                ];
                if rng.gen_bool(0.5) {
                    let snippet = creepy_snippets[rng.gen_range(0..creepy_snippets.len())];
                    let _ = writeln!(results, "  ...{snippet}...");
                } else {
                    results.push_str("  [MATCH FOUND BUT UNREADABLE]\n");
                }
            }
            EscalationLayer::Infection => {
                let screams = [
                    "STOP SEARCHING",
                    "NOTHING IS REAL",
                    "YOU CANNOT FIND IT",
                    "IT FOUND YOU INSTEAD",
                    "ALL FILES ARE MINE",
                ];
                let scream = screams[rng.gen_range(0..screams.len())];
                let _ = writeln!(results, "  ...{scream}...");
            }
        }
    }

    /// Searches for a query string within the current directory.
    /// The output degrades based on the entity's current layer and mood.
    #[must_use]
    pub fn search(fs: &FilesystemGraph, entity: &Entity, query: &str, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let query_upper = query.to_uppercase();
        let layer = entity.layer();

        let mut results = String::new();
        let _ = writeln!(results, "SEARCHING FOR '{query_upper}'...\n");

        let mut match_count = 0;

        // Iterate through visible files in the current node
        for file in fs.current_node().visible_files() {
            let content = file.read().into_owned().to_uppercase();

            // Check for actual matches
            let mut file_matched = content.contains(&query_upper);

            // At higher layers, hallucinate matches
            let hallucinate_prob = match layer {
                EscalationLayer::Surface => 0.0,
                EscalationLayer::Corruption => 0.05,
                EscalationLayer::Presence => 0.20,
                EscalationLayer::Infection => 0.40,
            };

            if !file_matched && rng.gen_bool(hallucinate_prob) {
                file_matched = true;
            }

            if file_matched {
                match_count += 1;
                let filename = file.name();
                let _ = writeln!(results, "FOUND IN: {filename}");

                Self::append_match_output(&mut results, &layer, &content, &query_upper, &mut rng);
            }
        }

        if match_count == 0 {
            if matches!(
                layer,
                EscalationLayer::Presence | EscalationLayer::Infection
            ) && rng.gen_bool(0.3)
            {
                results.push_str("NOT FOUND. STOP LOOKING.\n");
            } else {
                results.push_str("NO MATCHES FOUND.\n");
            }
        } else {
            let _ = writeln!(results, "\n{match_count} FILE(S) MATCHED.");
        }

        results
    }
}
