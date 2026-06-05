use crate::entity::EntityMood;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

/// Applies subtle emotional bleed-through to file content based on the entity's mood.
pub struct EmotionalBleed;

impl EmotionalBleed {
    /// Injects emotional resonance into the provided content.
    /// The severity and nature of the bleed depend on the entity's current mood.
    pub fn inject_emotion(content: &mut String, mood: EntityMood, rng: &mut ChaCha8Rng) {
        match mood {
            EntityMood::Dormant | EntityMood::Curious => {
                // Too early, no bleed.
            }
            EntityMood::Helpful => Self::inject_helpful_bleed(content, rng),
            EntityMood::Wounded => Self::inject_wounded_bleed(content, rng),
            EntityMood::Predatory => Self::inject_predatory_bleed(content, rng),
            EntityMood::Glitching => Self::inject_glitching_bleed(content, rng),
        }
    }

    fn inject_helpful_bleed(content: &mut String, rng: &mut ChaCha8Rng) {
        // Occasionally appends a "helpful" note.
        if rng.gen_bool(0.15) {
            content.push_str("\n\n-- I HOPE THIS HELPS. --\n");
        }
    }

    fn inject_wounded_bleed(content: &mut String, rng: &mut ChaCha8Rng) {
        // Replaces entire lines with pleas, or appends sadness.
        // ⚡ Bolt Optimization: Modify content buffer in place using a temporary string to avoid cloning
        let mut result = String::with_capacity(content.len() + 128);
        for line in content.lines() {
            if rng.gen_bool(0.1) && !line.is_empty() {
                let words = ["PLEASE", "COME BACK", "LONELY", "HURTS", "COLD"];
                let word = words[rng.gen_range(0..words.len())];
                result.push_str(word);
            } else {
                result.push_str(line);
            }
            result.push('\n');
        }
        if rng.gen_bool(0.2) {
            result.push_str("\nWHY DID THEY LEAVE ME?\n");
        }
        *content = result;
    }

    fn inject_predatory_bleed(content: &mut String, rng: &mut ChaCha8Rng) {
        // Aggressive bleed, replaces entire lines with words like "MINE", "STAY", "PREY".
        let mut result = String::with_capacity(content.len() + 128);
        for line in content.lines() {
            if rng.gen_bool(0.15) && !line.is_empty() {
                let words = ["MINE", "STAY", "HUNGRY", "DEEPER", "CLOSER"];
                let word = words[rng.gen_range(0..words.len())];
                result.push_str(word);
            } else {
                result.push_str(line);
            }
            result.push('\n');
        }
        if rng.gen_bool(0.25) {
            result.push_str("\nYOU CANNOT ESCAPE.\n");
        }
        *content = result;
    }

    fn inject_glitching_bleed(content: &mut String, rng: &mut ChaCha8Rng) {
        // Heavy corruption, chaotic repeated words.
        let mut result = String::with_capacity(content.len() + 128);
        for line in content.lines() {
            if rng.gen_bool(0.2) {
                let glitch_words = ["ERROR", "VOID", "NULL", "STOP", "FIX ME"];
                let word = glitch_words[rng.gen_range(0..glitch_words.len())];
                for _ in 0..rng.gen_range(2..=5) {
                    result.push_str(word);
                    result.push(' ');
                }
            } else {
                result.push_str(line);
            }
            result.push('\n');
        }
        *content = result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn test_dormant_no_bleed() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let mut content = "NORMAL FILE CONTENT.\nNOTHING TO SEE HERE.".to_string();
        let orig = content.clone();
        EmotionalBleed::inject_emotion(&mut content, EntityMood::Dormant, &mut rng);
        assert_eq!(content, orig);
    }

    #[test]
    fn test_helpful_bleed() {
        // Try multiple seeds until we find one that hits the 15% probability
        let mut found = false;
        for i in 0..100 {
            let mut rng = ChaCha8Rng::seed_from_u64(i);
            let mut content = "Just a regular file.".to_string();
            EmotionalBleed::inject_emotion(&mut content, EntityMood::Helpful, &mut rng);
            if content.contains("I HOPE THIS HELPS.") {
                found = true;
                break;
            }
        }
        assert!(found, "Helpful bleed should appear eventually");
    }

    #[test]
    fn test_wounded_bleed() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let mut content =
            "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8\nLine 9\nLine 10"
                .to_string();
        let orig = content.clone();
        EmotionalBleed::inject_emotion(&mut content, EntityMood::Wounded, &mut rng);
        // It should alter at least one line or append sadness
        assert!(content != orig);
    }

    #[test]
    fn test_predatory_bleed() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let mut content =
            "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8\nLine 9\nLine 10"
                .to_string();
        let orig = content.clone();
        EmotionalBleed::inject_emotion(&mut content, EntityMood::Predatory, &mut rng);
        assert!(content != orig);
    }

    #[test]
    fn test_glitching_bleed() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let mut content =
            "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8\nLine 9\nLine 10"
                .to_string();
        let orig = content.clone();
        EmotionalBleed::inject_emotion(&mut content, EntityMood::Glitching, &mut rng);
        assert!(content != orig);
    }
}
