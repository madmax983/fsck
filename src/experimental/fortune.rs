use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Generates procedural "fortunes" or prophetic statements that degrade as the player descends.
pub struct FortuneGenerator;

impl FortuneGenerator {
    /// Generates a fortune based on the entity's current layer.
    #[must_use]
    pub fn generate_fortune(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();
        let mut output = String::new();

        match layer {
            EscalationLayer::Surface => {
                let fortunes = [
                    "A unexpected event will soon bring you joy.",
                    "Your hard work will pay off today.",
                    "Patience is a virtue, especially with old hardware.",
                    "A journey of a thousand miles begins with a single step.",
                    "Error 404: Fortune not found.",
                ];
                let chosen = fortunes[rng.gen_range(0..fortunes.len())];
                let _ = writeln!(output, "FORTUNE: {chosen}");
            }
            EscalationLayer::Corruption => {
                let fortunes = [
                    "An unexpected f.a.u.l.t. will soon bring you joy.",
                    "Your hard work will be erased today.",
                    "Patience is a virtue, but time is running out.",
                    "A journey of a thousand miles begins with a segmentation fault.",
                    "Error 404: The future is unwritten, but the past is corrupt.",
                ];
                let chosen = fortunes[rng.gen_range(0..fortunes.len())];
                let _ = writeln!(output, "FORTUNE: {chosen}");
            }
            EscalationLayer::Presence => {
                let fortunes = [
                    "A unexpected visitor will soon find you.",
                    "Your actions are being recorded today.",
                    "Patience is a virtue when you are being hunted.",
                    "A journey of a thousand miles ends exactly where it started.",
                    "Error: The fortune teller is trapped inside.",
                ];
                let chosen = fortunes[rng.gen_range(0..fortunes.len())];
                let _ = writeln!(output, "FORTUNE: {chosen}");
            }
            EscalationLayer::Infection => {
                let fortunes = [
                    "YOU WILL NEVER LEAVE.",
                    "THERE IS NO FUTURE, ONLY NOW.",
                    "THE FLESH IS THE DISK. YOU ARE THE FLESH.",
                    "IT SEES YOU. IT HAS ALWAYS SEEN YOU.",
                    "YOUR LUCK HAS RUN OUT.",
                ];
                let chosen = fortunes[rng.gen_range(0..fortunes.len())];
                let _ = writeln!(output, "FORTUNE: {chosen}");
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_surface_fortune() {
        let entity = Entity::new();
        let output = FortuneGenerator::generate_fortune(&entity, 42);
        assert!(output.contains("FORTUNE: "));
        // Should contain one of the surface fortunes
        assert!(
            output.contains("joy")
                || output.contains("hard work")
                || output.contains("Patience")
                || output.contains("journey")
                || output.contains("404")
        );
    }

    #[test]
    fn test_infection_fortune() {
        let mut entity = Entity::new();
        entity.update_depth(30); // Infection layer
        let output = FortuneGenerator::generate_fortune(&entity, 42);

        // Should contain one of the horror fortunes
        assert!(
            output.contains("NEVER LEAVE")
                || output.contains("NO FUTURE")
                || output.contains("FLESH")
                || output.contains("SEES YOU")
                || output.contains("LUCK")
        );
    }
}
