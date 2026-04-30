use crate::entity::{Entity, EntityMood, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

/// A procedural Rorschach inkblot generator for psychological evaluation of the machine.
pub struct RorschachTest;

impl RorschachTest {
    #[must_use]
    pub fn generate(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();

        // Define our block characters based on the layer
        let chars = match layer {
            EscalationLayer::Surface => [' ', ' ', '.', ':', '░'],
            EscalationLayer::Corruption | EscalationLayer::Presence => [' ', '░', '▒', '▓', '█'],
            EscalationLayer::Infection => [' ', 'X', '▒', '▓', '█'],
        };

        // Use Bolt optimization: pre-allocate enough space for 10 lines of 20 characters + newlines + analysis string
        let mut output = String::with_capacity(512);

        // Generate a 10x10 half-grid
        for _ in 0..10 {
            let mut left_half = String::with_capacity(10);
            for _ in 0..10 {
                let c = chars[rng.gen_range(0..chars.len())];
                left_half.push(c);
            }
            // Add left half
            output.push_str(&left_half);
            // Add right half (mirrored)
            output.push_str(&left_half.chars().rev().collect::<String>());
            output.push('\n');
        }

        // Append analysis based on mood and layer
        output.push('\n');
        let mood = entity.current_mood();
        if matches!(layer, EscalationLayer::Infection) {
            output.push_str("ANALYSIS: IT SEES YOU SEEING IT");
        } else if matches!(mood, EntityMood::Predatory | EntityMood::Glitching) {
            output.push_str("ANALYSIS: HOSTILE PATTERNS DETECTED");
        } else if matches!(mood, EntityMood::Wounded | EntityMood::Curious) {
            output.push_str("ANALYSIS: CALM BUT UNSTABLE");
        } else {
            output.push_str("ANALYSIS: STANDARD SYMMETRY OBSERVED");
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_rorschach_symmetry() {
        let mut entity = Entity::new();
        // Record interaction to test deterministic change
        let output1 = RorschachTest::generate(&entity, 12345);
        entity.record_interaction();
        let output2 = RorschachTest::generate(&entity, 12345);

        assert!(!output1.is_empty(), "Output should not be empty");
        assert_ne!(
            output1, output2,
            "Output must be deterministically random based on interaction count"
        );

        // We look for lines containing the block character or just lines of length > 0 in the inkblot part.
        let lines: Vec<&str> = output1
            .lines()
            .filter(|l| l.chars().count() == 20)
            .collect();
        assert!(
            !lines.is_empty(),
            "Inkblot should contain 20 character lines"
        );

        for line in lines {
            let half_len = line.chars().count() / 2;
            let left: String = line.chars().take(half_len).collect();
            let right: String = line.chars().skip(half_len).collect();
            let right_reversed: String = right.chars().rev().collect();
            assert_eq!(
                left, right_reversed,
                "Line is not perfectly horizontally mirrored: {line}"
            );
        }
    }
}
