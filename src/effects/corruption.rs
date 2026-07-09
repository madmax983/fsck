use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

/// Characters used for visual corruption
const CORRUPTION_CHARS: &[char] = &[
    '█', '▓', '▒', '░', '▀', '▄', '▌', '▐', '■', '□', '▪', '▫', '∙', '·', '※', '?', '#', '@', '$',
    '%', '&', '*',
];

/// How intense the corruption effect should be
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorruptionIntensity {
    None,
    Mild,     // 5-10% of characters
    Moderate, // 15-25% of characters
    Severe,   // 35-50% of characters
    Total,    // 70%+ of characters
}

impl From<u32> for CorruptionIntensity {
    fn from(depth: u32) -> Self {
        match depth {
            0..=10 => Self::None,
            11..=20 => Self::Mild,
            21..=35 => Self::Moderate,
            36..=55 => Self::Severe,
            _ => Self::Total,
        }
    }
}

impl CorruptionIntensity {
    const fn corruption_rate(self) -> f32 {
        match self {
            Self::None => 0.0,
            Self::Mild => 0.075,
            Self::Moderate => 0.20,
            Self::Severe => 0.425,
            Self::Total => 0.75,
        }
    }
}

/// Applies visual corruption to text
pub struct CorruptionEffect {
    intensity: CorruptionIntensity,
}

impl CorruptionEffect {
    #[must_use]
    pub const fn new(intensity: CorruptionIntensity) -> Self {
        Self { intensity }
    }

    #[must_use]
    pub fn apply<'a>(&self, text: &'a str, seed: u64) -> std::borrow::Cow<'a, str> {
        if matches!(self.intensity, CorruptionIntensity::None) {
            return std::borrow::Cow::Borrowed(text);
        }

        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let rate = self.intensity.corruption_rate();
        // Calculate max possible capacity. 3 is the max byte size of CORRUPTION_CHARS.
        let mut result = String::with_capacity(text.len() * 3);

        for ch in text.chars() {
            if ch.is_whitespace() {
                result.push(ch);
            } else if rng.r#gen::<f32>() < rate {
                result.push(CORRUPTION_CHARS[rng.gen_range(0..CORRUPTION_CHARS.len())]);
            } else {
                result.push(ch);
            }
        }

        std::borrow::Cow::Owned(result)
    }
}
