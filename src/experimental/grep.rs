use crate::entity::{Entity, EscalationLayer};
use crate::filesystem::FilesystemGraph;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// A tool to search for text within the current directory's files.
pub struct SearchTool;

impl SearchTool {
    /// Finds the byte index of a case-insensitive substring match without allocating.
    fn find_ignore_ascii_case(haystack: &str, needle: &str) -> Option<usize> {
        if needle.is_empty() {
            return Some(0);
        }
        haystack
            .as_bytes()
            .windows(needle.len())
            .position(|w| w.eq_ignore_ascii_case(needle.as_bytes()))
    }

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

    /// Formats a matched result based on the entity's escalation layer.
    fn format_match(
        layer: EscalationLayer,
        query_upper: &str,
        content: &str,
        rng: &mut ChaCha8Rng,
        results: &mut String,
    ) {
        match layer {
            EscalationLayer::Surface => Self::format_surface_match(query_upper, content, results),
            EscalationLayer::Corruption => {
                Self::format_corruption_match(query_upper, content, rng, results);
            }
            EscalationLayer::Presence => Self::format_presence_match(rng, results),
            EscalationLayer::Infection => Self::format_infection_match(rng, results),
        }
    }

    fn format_surface_match(query_upper: &str, content: &str, results: &mut String) {
        // Show actual snippet if possible, or a generic match string
        if let Some(idx) = Self::find_ignore_ascii_case(content, query_upper) {
            let snippet = Self::extract_snippet(content, idx, query_upper.len());
            // ⚡ Bolt Optimization: Avoid allocating string replacement methods on hot paths.
            results.push_str("  ...");
            for (i, word) in snippet.split_whitespace().enumerate() {
                if i > 0 {
                    results.push(' ');
                }
                results.push_str(word);
            }
            results.push_str("...\n");
        } else {
            // Shouldn't happen at Surface, but just in case
            results.push_str("  [MATCH FOUND]\n");
        }
    }

    fn format_corruption_match(
        query_upper: &str,
        content: &str,
        rng: &mut ChaCha8Rng,
        results: &mut String,
    ) {
        if rng.gen_bool(0.3) {
            results.push_str("  ...[DATA CORRUPTED]...\n");
        } else if let Some(idx) = Self::find_ignore_ascii_case(content, query_upper) {
            let snippet = Self::extract_snippet(content, idx, query_upper.len());
            // ⚡ Bolt Optimization: Avoid allocating string replacement methods on hot paths.
            results.push_str("  ...");
            for (i, word) in snippet.split_whitespace().enumerate() {
                if i > 0 {
                    results.push(' ');
                }
                results.push_str(word);
            }
            results.push_str("...\n");
        } else {
            results.push_str("  [FALSE POSITIVE DETECTED]\n");
        }
    }

    fn format_presence_match(rng: &mut ChaCha8Rng, results: &mut String) {
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

    fn format_infection_match(rng: &mut ChaCha8Rng, results: &mut String) {
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

    fn check_and_format_match(
        file: &crate::filesystem::FileNode,
        query_upper: &str,
        layer: EscalationLayer,
        rng: &mut ChaCha8Rng,
        results: &mut String,
        match_count: &mut usize,
    ) {
        // ⚡ Bolt Optimization: Avoided `.to_uppercase()` heap allocation entirely by using a zero-allocation case-insensitive sliding window search.
        let content = file.read();

        // Check for actual matches
        let mut file_matched = Self::find_ignore_ascii_case(&content, query_upper).is_some();

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
            *match_count += 1;
            let filename = file.name();
            let _ = writeln!(results, "FOUND IN: {filename}");

            // Show a snippet or a corrupted message based on layer
            Self::format_match(layer, query_upper, &content, rng, results);
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
            Self::check_and_format_match(
                file,
                &query_upper,
                layer,
                &mut rng,
                &mut results,
                &mut match_count,
            );
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
