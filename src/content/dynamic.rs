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
    pub fn counter(base: &str) -> Self {
        Self::Counter {
            base: base.to_string(),
            count: 0,
        }
    }

    pub fn timestamp() -> Self {
        Self::Timestamp
    }

    pub fn corrupted(text: &str, intensity: f32) -> Self {
        Self::Corrupted {
            text: text.to_string(),
            intensity: intensity.clamp(0.0, 1.0),
            seed: 0,
        }
    }

    pub fn generate(&mut self) -> String {
        match self {
            Self::Counter { base, count } => {
                *count += 1;
                format!("{}{}\n", base.repeat(*count as usize), count)
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
