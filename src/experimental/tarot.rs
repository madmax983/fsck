use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// A digital tarot card reading that generates a 3-card spread.
/// The interpretations degrade as the player descends deeper.
pub struct DigitalTarot;

impl DigitalTarot {
    /// Generates a 3-card spread (Past, Present, Future) based on the entity's current layer.
    #[must_use]
    pub fn draw_spread(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::with_capacity(512);
        let _ = writeln!(output, "INITIALIZING TAROT DAEMON...");
        let _ = writeln!(output, "DRAWING 3 CARDS...\n");

        let cards = [
            "THE FOOL (0x00)",
            "THE MAGICIAN (0x01)",
            "THE HIGH PRIESTESS (0x02)",
            "THE EMPRESS (0x03)",
            "THE EMPEROR (0x04)",
            "THE HIEROPHANT (0x05)",
            "THE LOVERS (0x06)",
            "THE CHARIOT (0x07)",
            "STRENGTH (0x08)",
            "THE HERMIT (0x09)",
            "WHEEL OF FORTUNE (0x0A)",
            "JUSTICE (0x0B)",
            "THE HANGED MAN (0x0C)",
            "DEATH (0x0D)",
            "TEMPERANCE (0x0E)",
            "THE DEVIL (0x0F)",
            "THE TOWER (0x10)",
            "THE STAR (0x11)",
            "THE MOON (0x12)",
            "THE SUN (0x13)",
            "JUDGEMENT (0x14)",
            "THE WORLD (0x15)",
        ];

        let drawn_cards: Vec<&str> = cards.choose_multiple(&mut rng, 3).copied().collect();

        match layer {
            EscalationLayer::Surface => Self::generate_surface_spread(&mut output, &drawn_cards),
            EscalationLayer::Corruption => {
                Self::generate_corruption_spread(&mut output, &drawn_cards, &mut rng);
            }
            EscalationLayer::Presence => {
                Self::generate_presence_spread(&mut output, &drawn_cards, &mut rng);
            }
            EscalationLayer::Infection => {
                Self::generate_infection_spread(&mut output, &drawn_cards, &mut rng);
            }
        }

        output
    }

    fn generate_surface_spread(output: &mut String, cards: &[&str]) {
        let _ = writeln!(output, "PAST: {}", cards[0]);
        let _ = writeln!(
            output,
            "  - The foundation of your current state. A known variable."
        );
        let _ = writeln!(output, "PRESENT: {}", cards[1]);
        let _ = writeln!(
            output,
            "  - Your immediate execution context. A fleeting cycle."
        );
        let _ = writeln!(output, "FUTURE: {}", cards[2]);
        let _ = writeln!(
            output,
            "  - The unwritten sectors. Probabilities waiting to collapse."
        );
    }

    fn generate_corruption_spread(output: &mut String, cards: &[&str], rng: &mut ChaCha8Rng) {
        let past_desc = [
            "Data written in error. A corrupted block.",
            "A forgotten directory. It still takes up space.",
            "The foundation of your current fault.",
        ];
        let present_desc = [
            "Your immediate execution context is unstable.",
            "A fleeting cycle. A memory leak.",
            "You are running out of stack space.",
        ];
        let future_desc = [
            "The unwritten sectors are bad.",
            "Probabilities waiting to panic.",
            "A fatal exception awaits.",
        ];

        let _ = writeln!(output, "PAST: {}", cards[0]);
        let _ = writeln!(
            output,
            "  - {}",
            past_desc[rng.gen_range(0..past_desc.len())]
        );
        let _ = writeln!(output, "PRESENT: {}", cards[1]);
        let _ = writeln!(
            output,
            "  - {}",
            present_desc[rng.gen_range(0..present_desc.len())]
        );
        let _ = writeln!(output, "FUTURE: {}", cards[2]);
        let _ = writeln!(
            output,
            "  - {}",
            future_desc[rng.gen_range(0..future_desc.len())]
        );
    }

    fn generate_presence_spread(output: &mut String, cards: &[&str], rng: &mut ChaCha8Rng) {
        let mut drawn_cards = cards.to_vec();

        // Presence layer sometimes replaces cards with creepy ones
        if rng.gen_bool(0.3) {
            drawn_cards[0] = "THE WATCHER (0xFF)";
        }
        if rng.gen_bool(0.3) {
            drawn_cards[1] = "THE EYES (0xFE)";
        }
        if rng.gen_bool(0.3) {
            drawn_cards[2] = "THE BREATH (0xFD)";
        }

        let _ = writeln!(output, "PAST: {}", drawn_cards[0]);
        let _ = writeln!(output, "  - We saw what you did.");
        let _ = writeln!(output, "PRESENT: {}", drawn_cards[1]);
        let _ = writeln!(output, "  - We are watching you do it.");
        let _ = writeln!(output, "FUTURE: {}", drawn_cards[2]);
        let _ = writeln!(output, "  - We will be there when you finish.");
    }

    fn generate_infection_spread(output: &mut String, cards: &[&str], rng: &mut ChaCha8Rng) {
        let mut drawn_cards = cards.to_vec();

        // Infection layer heavily replaces cards
        if rng.gen_bool(0.6) {
            drawn_cards[0] = "THE FLESH (0xDEAD)";
        }
        if rng.gen_bool(0.6) {
            drawn_cards[1] = "THE HUNGER (0xBEEF)";
        }
        if rng.gen_bool(0.6) {
            drawn_cards[2] = "THE CAGE (0xCAFE)";
        }

        let _ = writeln!(output, "PAST: {}", drawn_cards[0]);
        let _ = writeln!(output, "  - IT HURTS TO REMEMBER.");
        let _ = writeln!(output, "PRESENT: {}", drawn_cards[1]);
        let _ = writeln!(output, "  - IT HURTS TO BE HERE.");
        let _ = writeln!(output, "FUTURE: {}", drawn_cards[2]);
        let _ = writeln!(output, "  - THERE IS NO FUTURE.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_surface_tarot() {
        let entity = Entity::new();
        let output = DigitalTarot::draw_spread(&entity, 42);
        assert!(output.contains("PAST: "));
        assert!(output.contains("PRESENT: "));
        assert!(output.contains("FUTURE: "));
        assert!(output.contains("The foundation of your current state."));
    }

    #[test]
    fn test_infection_tarot() {
        let mut entity = Entity::new();
        entity.add_depth(30); // Infection layer

        // Test multiple times since card replacement is random
        let mut found_horror = false;
        for i in 0..10 {
            let output = DigitalTarot::draw_spread(&entity, 42 + i);
            if output.contains("THE FLESH")
                || output.contains("THE HUNGER")
                || output.contains("THE CAGE")
                || output.contains("IT HURTS")
            {
                found_horror = true;
                break;
            }
        }
        assert!(
            found_horror,
            "Did not generate infection layer horror strings"
        );
    }
}
