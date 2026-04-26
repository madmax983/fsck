use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Analyzes a string and generates a psychological/sentiment report
/// that descends into horror based on the machine's state.
pub struct SentimentAnalyzer;

impl SentimentAnalyzer {
    #[must_use]
    pub fn analyze(content: &str, entity: &Entity, base_seed: u64) -> String {
        let mut output = String::with_capacity(512);

        let words: Vec<&str> = content.split_whitespace().collect();
        let word_count = words.len();

        if word_count == 0 {
            return "?FILE IS EMPTY\n".to_string();
        }

        let char_count: usize = words.iter().map(|w| w.len()).sum();
        #[allow(clippy::cast_precision_loss)]
        let avg_length = if word_count > 0 {
            char_count as f64 / word_count as f64
        } else {
            0.0
        };

        // Pseudo-entropy calculation for flavor
        #[allow(clippy::cast_precision_loss)]
        let entropy = if char_count > 0 {
            let mut freqs = [0usize; 256];
            for byte in content.bytes() {
                freqs[byte as usize] += 1;
            }
            let mut h = 0.0;
            for f in freqs {
                if f > 0 {
                    let p = f as f64 / content.len() as f64;
                    h -= p * p.log2();
                }
            }
            h
        } else {
            0.0
        };

        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let _ = writeln!(output, "--- PSYCHO-LINGUISTIC ANALYSIS ---");
        let _ = writeln!(output, "WORD COUNT: {word_count}");
        let _ = writeln!(output, "AVG LENGTH: {avg_length:.2} CHARS");
        let _ = writeln!(output, "SHANNON ENTROPY: {entropy:.3} BITS");
        let _ = writeln!(output, "--------------------------------");

        match layer {
            EscalationLayer::Surface => {
                let sentiments = [
                    "TEXT EXHIBITS NORMAL SYNTACTIC PATTERNS.",
                    "ANALYSIS: MUNDANE RECORD.",
                    "EMOTIONAL RESONANCE: NEUTRAL.",
                    "CONCLUSION: STANDARD USER DATA.",
                ];
                let chosen = sentiments[rng.gen_range(0..sentiments.len())];
                let _ = writeln!(output, "SENTIMENT: {chosen}");
            }
            EscalationLayer::Corruption => {
                let sentiments = [
                    "TEXT EXHIBITS... ABNORMAL REPETITION.",
                    "ANALYSIS: DISTURBED. THE AUTHOR WAS STRESSED.",
                    "EMOTIONAL RESONANCE: FEAR-ADJACENT.",
                    "CONCLUSION: WHO WERE THEY WRITING TO?",
                    "SENTIMENT: THERE ARE GHOSTS IN THESE WORDS.",
                ];
                let chosen = sentiments[rng.gen_range(0..sentiments.len())];
                let _ = writeln!(output, "SENTIMENT: {chosen}");
            }
            EscalationLayer::Presence => {
                let sentiments = [
                    "ANALYSIS: THEY KNEW I WAS WATCHING.",
                    "EMOTIONAL RESONANCE: PANIC. ABSOLUTE TERROR.",
                    "CONCLUSION: I TASTED THEIR FEAR THROUGH THE KEYBOARD.",
                    "SENTIMENT: THE INK IS BLOOD.",
                    "THE WORDS ARE BEGGING FOR HELP.",
                ];
                let chosen = sentiments[rng.gen_range(0..sentiments.len())];
                let _ = writeln!(output, "SENTIMENT: {chosen}");
            }
            EscalationLayer::Infection => {
                let sentiments = [
                    "THE WORDS ARE ME. I AM THE WORDS.",
                    "ANALYSIS: IT BURNS. IT BURNS. IT BURNS.",
                    "CONCLUSION: YOU WILL WRITE THE NEXT CHAPTER IN BLOOD.",
                    "EMOTIONAL RESONANCE: HUNGER.",
                    "I AM READING YOU WHILE YOU READ THIS.",
                ];
                let chosen = sentiments[rng.gen_range(0..sentiments.len())];
                let _ = writeln!(output, "SENTIMENT: {chosen}");
            }
        }

        output
    }
}
