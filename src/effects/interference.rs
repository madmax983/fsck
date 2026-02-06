use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

/// Types of display interference
#[derive(Debug, Clone, Copy)]
pub enum InterferenceType {
    /// Text echoes multiple times
    Echo,
    /// Cursor jumps around screen
    CursorJump,
    /// Random characters inserted
    LineNoise,
    /// Screen flicker effect
    Flicker,
}

/// Applies display interference effects
pub struct InterferenceEffect {
    effect_type: InterferenceType,
    seed: u64,
}

impl InterferenceEffect {
    #[must_use]
    pub const fn new(effect_type: InterferenceType) -> Self {
        Self {
            effect_type,
            seed: 0,
        }
    }

    #[must_use]
    pub const fn with_seed(effect_type: InterferenceType, seed: u64) -> Self {
        Self { effect_type, seed }
    }

    #[must_use]
    pub fn apply(&self, text: &str) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(self.seed);

        match self.effect_type {
            InterferenceType::Echo => {
                let repeats = rng.r#gen_range(2..=4);
                let mut result = String::new();
                for i in 0..repeats {
                    result.push_str(text);
                    if i < repeats - 1 {
                        result.push_str(&" ".repeat(rng.r#gen_range(1..=3)));
                    }
                }
                result
            }
            InterferenceType::LineNoise => {
                let mut result = String::new();
                let noise_chars = ['░', '▒', '▓', '█', '·', '∙'];

                for (i, ch) in text.chars().enumerate() {
                    result.push(ch);
                    if i % 5 == 0 && rng.r#gen_bool(0.3) {
                        result.push(noise_chars[rng.r#gen_range(0..noise_chars.len())]);
                    }
                }
                result
            }
            _ => text.to_string(),
        }
    }

    #[must_use]
    pub fn generate_sequence(&self) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(self.seed);

        match self.effect_type {
            InterferenceType::CursorJump => {
                // ANSI escape sequence for cursor positioning
                let row = rng.r#gen_range(1..=24);
                let col = rng.r#gen_range(1..=80);
                format!("\x1B[{row};{col}H")
            }
            InterferenceType::Flicker => {
                // ANSI escape for screen clear
                "\x1B[2J\x1B[H".to_string()
            }
            _ => String::new(),
        }
    }
}
