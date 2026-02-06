use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

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
}

impl DynamicContent {
    /// Creates a counter that increments on each read.
    ///
    /// # Arguments
    /// * `base` - The base text to repeat (repeated count times per read)
    ///
    /// # Returns
    /// A counter starting at 0 that increments with each generate() call
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
    pub fn timestamp() -> Self {
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
    pub fn corrupted(text: &str, intensity: f32) -> Self {
        Self::Corrupted {
            text: text.to_string(),
            intensity: intensity.clamp(0.0, 1.0),
            seed: 0,
        }
    }

    /// Generates content for this dynamic source.
    ///
    /// This method mutates internal state (counters, seeds) to produce
    /// different output on each call.
    ///
    /// # Returns
    /// - `Counter`: Base text repeated count times, followed by the count number
    /// - `Timestamp`: Current timestamp (placeholder format in this version)
    /// - `Corrupted`: Text with randomly corrupted characters based on intensity
    pub fn generate(&mut self) -> String {
        match self {
            Self::Counter { base, count } => {
                *count = count.saturating_add(1);
                let repeat_count = (*count as usize).min(10_000); // Cap repetitions
                format!("{}{}\n", base.repeat(repeat_count), count)
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
                let mut result = String::new();

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
        }
    }
}
