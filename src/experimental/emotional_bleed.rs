use crate::entity::EntityMood;
use rand::prelude::SliceRandom;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

/// Applies subtle emotional bleed-through to file content based on the entity's mood.
pub struct EmotionalBleed;

impl EmotionalBleed {
    /// Injects emotional resonance into the provided content.
    /// The severity and nature of the bleed depend on the entity's current mood.
    ///
    /// # Panics
    ///
    /// Panics if the internal slice used for random selection is empty.
    #[must_use]
    pub fn inject_emotion(content: &str, mood: EntityMood, rng: &mut ChaCha8Rng) -> String {
        match mood {
            EntityMood::Dormant | EntityMood::Curious => {
                // Too early, no bleed.
                content.to_string()
            }
            EntityMood::Helpful => {
                // Occasionally appends a "helpful" note.
                // ⚡ Bolt Optimization: Uses `String::with_capacity` to prevent re-allocations when appending the note.
                let mut result = String::with_capacity(content.len() + 32);
                result.push_str(content);
                if rng.gen_bool(0.15) {
                    result.push_str("\n\n-- I HOPE THIS HELPS. --\n");
                }
                result
            }
            EntityMood::Wounded => {
                // Replaces entire lines with pleas, or appends sadness.
                // ⚡ Bolt Optimization: Pre-allocates string capacity to avoid multiple heap re-allocations.
                let mut result = String::with_capacity(content.len() + 128);
                for line in content.lines() {
                    if rng.gen_bool(0.1) && !line.is_empty() {
                        let words = ["PLEASE", "COME BACK", "LONELY", "HURTS", "COLD"];
                        let word = *words.choose(rng).unwrap();
                        result.push_str(word);
                    } else {
                        result.push_str(line);
                    }
                    result.push('\n');
                }
                if rng.gen_bool(0.2) {
                    result.push_str("\nWHY DID THEY LEAVE ME?\n");
                }
                result
            }
            EntityMood::Predatory => {
                // Aggressive bleed, replaces entire lines with words like "MINE", "STAY", "PREY".
                // ⚡ Bolt Optimization: Pre-allocates string capacity to avoid multiple heap re-allocations.
                let mut result = String::with_capacity(content.len() + 128);
                for line in content.lines() {
                    if rng.gen_bool(0.15) && !line.is_empty() {
                        let words = ["MINE", "STAY", "HUNGRY", "DEEPER", "CLOSER"];
                        let word = *words.choose(rng).unwrap();
                        result.push_str(word);
                    } else {
                        result.push_str(line);
                    }
                    result.push('\n');
                }
                if rng.gen_bool(0.25) {
                    result.push_str("\nYOU CANNOT ESCAPE.\n");
                }
                result
            }
            EntityMood::Glitching => {
                // Heavy corruption, chaotic repeated words.
                // ⚡ Bolt Optimization: Pre-allocates string capacity to avoid multiple heap re-allocations.
                let mut result = String::with_capacity(content.len() + 128);
                for line in content.lines() {
                    if rng.gen_bool(0.2) {
                        let glitch_words = ["ERROR", "VOID", "NULL", "STOP", "FIX ME"];
                        let word = *glitch_words.choose(rng).unwrap();
                        for _ in 0..rng.gen_range(2..=5) {
                            result.push_str(word);
                            result.push(' ');
                        }
                    } else {
                        result.push_str(line);
                    }
                    result.push('\n');
                }
                result
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn test_dormant_no_bleed() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let content = "NORMAL FILE CONTENT.\nNOTHING TO SEE HERE.";
        let result = EmotionalBleed::inject_emotion(content, EntityMood::Dormant, &mut rng);
        assert_eq!(result, content);
    }

    #[test]
    fn test_helpful_bleed() {
        // Try multiple seeds until we find one that hits the 15% probability
        let mut found = false;
        for i in 0..100 {
            let mut rng = ChaCha8Rng::seed_from_u64(i);
            let content = "Just a regular file.";
            let result = EmotionalBleed::inject_emotion(content, EntityMood::Helpful, &mut rng);
            if result.contains("I HOPE THIS HELPS.") {
                found = true;
                break;
            }
        }
        assert!(found, "Helpful bleed should appear eventually");
    }

    #[test]
    fn test_wounded_bleed() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let content =
            "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8\nLine 9\nLine 10";
        let result = EmotionalBleed::inject_emotion(content, EntityMood::Wounded, &mut rng);
        // It should alter at least one line or append sadness
        assert!(result != content);
    }

    #[test]
    fn test_predatory_bleed() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let content =
            "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8\nLine 9\nLine 10";
        let result = EmotionalBleed::inject_emotion(content, EntityMood::Predatory, &mut rng);
        assert!(result != content);
    }

    #[test]
    fn test_glitching_bleed() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let content =
            "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8\nLine 9\nLine 10";
        let result = EmotionalBleed::inject_emotion(content, EntityMood::Glitching, &mut rng);
        assert!(result != content);
    }
}
