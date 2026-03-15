#![allow(clippy::collapsible_if)]
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

/// Corrupts timestamps and filenames
pub struct MetadataCorruptor {
    depth: u32,
}

impl MetadataCorruptor {
    #[must_use]
    pub const fn new(depth: u32) -> Self {
        Self { depth }
    }

    #[must_use]
    pub fn corrupt_timestamp(&self, original: &str) -> String {
        if self.depth < 11 {
            return original.to_string();
        }

        let mut rng = ChaCha8Rng::seed_from_u64(u64::from(self.depth));

        match self.depth {
            11..=30 => {
                // Mild corruption - change a digit
                // ⚡ Bolt Optimization: Removes intermediate .collect::<Vec<_>>() heap allocation when corrupting timestamps.
                if let Some((idx, ch)) = original.char_indices().find(|(_, c)| c.is_ascii_digit()) {
                    if let Some(digit) = char::from_digit(rng.r#gen_range(0..10), 10) {
                        let mut result = String::with_capacity(original.len());
                        result.push_str(&original[..idx]);
                        result.push(digit);
                        result.push_str(&original[idx + ch.len_utf8()..]);
                        return result;
                    }
                }
                original.to_string()
            }
            31..=60 => {
                // Presence - impossible dates
                self.generate_impossible_date(rng.r#gen())
            }
            _ => {
                // Infection - completely scrambled
                format!("{}???-??-??", rng.r#gen_range(1900..=2099))
            }
        }
    }

    #[must_use]
    pub fn generate_impossible_date(&self, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        let year = match rng.r#gen_range(0..5) {
            0 => rng.r#gen_range(1943..=1950), // Before Apple IIe existed
            1 => rng.r#gen_range(2050..=2099), // Future
            2 => rng.r#gen_range(1800..=1900), // Very old
            _ => rng.r#gen_range(1984..=2024), // Normal but still suspicious
        };

        let month = rng.r#gen_range(1..=12);
        let day = rng.r#gen_range(1..=28);

        format!("{year:04}-{month:02}-{day:02}")
    }

    #[must_use]
    pub fn corrupt_filename(&self, original: &str, seed: u64) -> String {
        if self.depth < 21 {
            return original.to_string();
        }

        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        // ⚡ Bolt Optimization: Pre-allocate String capacity based on the original length
        // to eliminate dynamic heap reallocations as the corrupted string is constructed.
        let mut result = String::with_capacity(original.len());

        let corruption_rate = match self.depth {
            21..=35 => 0.15,
            36..=60 => 0.30,
            _ => 0.50,
        };

        for ch in original.chars() {
            if ch == '.' || ch == ' ' {
                result.push(ch);
            } else if rng.r#gen::<f32>() < corruption_rate {
                let corrupt = ['?', '#', '@', '_', '-', '~'];
                result.push(corrupt[rng.r#gen_range(0..corrupt.len())]);
            } else {
                result.push(ch);
            }
        }

        result
    }
}
