use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Dynamic content generators for files that change on read
#[derive(Debug, Clone)]
pub enum DynamicContent {
    /// Counter that increments each read
    Counter { base: String, count: u32 },
    /// Current timestamp (corrupted format at higher depths)
    Timestamp,
    /// Text with random corruption
    Corrupted {
        text: String,
        intensity: f32,
        seed: u64,
    },
    /// Text that cycles through different messages on each read
    RepeatingText {
        messages: Vec<&'static str>,
        current_index: usize,
    },
}

impl DynamicContent {
    /// Creates a counter that increments on each read.
    ///
    /// # Arguments
    /// * `base` - The base text to repeat (repeated count times per read)
    ///
    /// # Returns
    /// A counter starting at 0 that increments with each `generate()` call
    #[must_use]
    pub fn counter(base: &str) -> Self {
        Self::Counter {
            base: base.to_string(),
            count: 0,
        }
    }

    /// Creates a timestamp generator.
    ///
    /// # Returns
    /// A dynamic timestamp that returns the current time on each read.
    /// At higher filesystem depths, the format becomes corrupted.
    #[must_use]
    pub const fn timestamp() -> Self {
        Self::Timestamp
    }

    /// Creates corrupted text with random character replacements.
    ///
    /// # Arguments
    /// * `text` - The original text to corrupt
    /// * `intensity` - Corruption probability per character (0.0-1.0, clamped)
    ///
    /// # Returns
    /// A generator that produces deterministically corrupted text using an
    /// internal seed that increments per call
    #[must_use]
    pub fn corrupted(text: &str, intensity: f32) -> Self {
        Self::Corrupted {
            text: text.to_string(),
            intensity: intensity.clamp(0.0, 1.0),
            seed: 0,
        }
    }

    /// Creates a repeating text generator.
    ///
    /// # Arguments
    /// * `messages` - A list of static strings to cycle through on each read
    ///
    /// # Returns
    /// A generator that returns the next message in the list, wrapping around.
    #[must_use]
    pub fn repeating(messages: &[&'static str]) -> Self {
        Self::RepeatingText {
            messages: messages.to_vec(),
            current_index: 0,
        }
    }

    /// Generates content for this dynamic source.
    /// ⚡ Bolt Optimization: Uses `.with_capacity(text.len() + 1)` in `Corrupted` to prevent
    /// repeated allocations during generation.
    /// ⚡ Bolt Optimization: Switched `RepeatingText.messages` from `Vec<String>` to `Vec<&'static str>` to eliminate heap allocations when initializing and cycling through static predefined repeating messages.
    /// ⚡ Bolt Optimization: Computes total capacity and uses `String::with_capacity()` with `.push_str()` and `writeln!()` for `Counter` to avoid intermediate `.repeat()` and `format!` heap allocations.
    ///
    /// This method mutates internal state (counters, seeds) to produce
    /// different output on each call.
    ///
    /// # Returns
    /// - `Counter`: Base text repeated count times, followed by the count number
    /// - `Timestamp`: Current timestamp (placeholder format in this version)
    /// - `Corrupted`: Text with randomly corrupted characters based on intensity
    /// - `RepeatingText`: The next message in the cycle
    pub fn generate(&mut self) -> String {
        match self {
            Self::Counter { base, count } => {
                *count = count.saturating_add(1);
                let repeat_count = (*count as usize).min(10_000); // Cap repetitions

                // Max length of u32 is 10 digits + 1 for newline
                let capacity = base.len() * repeat_count + 11;
                let mut result = String::with_capacity(capacity);
                for _ in 0..repeat_count {
                    result.push_str(base);
                }
                let _ = writeln!(&mut result, "{count}");
                result
            }
            Self::Timestamp => {
                // In real impl, would use js_sys::Date via web-sys
                // For now, placeholder
                "2024-??-?? ??:??:??\n".to_string()
            }
            Self::Corrupted {
                text,
                intensity,
                seed,
            } => {
                *seed += 1;
                let mut rng = ChaCha8Rng::seed_from_u64(*seed);
                let mut result = String::with_capacity(text.len() + 1);

                for ch in text.chars() {
                    if rng.r#gen::<f32>() < *intensity {
                        // Corrupt this character
                        let corrupt = ['█', '▓', '▒', '░', '?', '#', '@', '$'];
                        result.push(corrupt[rng.gen_range(0..corrupt.len())]);
                    } else {
                        result.push(ch);
                    }
                }

                result.push('\n');
                result
            }
            Self::RepeatingText {
                messages,
                current_index,
            } => {
                if messages.is_empty() {
                    return String::new();
                }
                let msg = messages[*current_index];
                *current_index = (*current_index + 1) % messages.len();
                msg.to_string()
            }
        }
    }
}
