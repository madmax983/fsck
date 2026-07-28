use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// A Tarot reader simulator that draws and interprets cards based on the entity's layer.
pub struct TarotReader;

impl TarotReader {
    #[must_use]
    pub fn read_cards(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::with_capacity(256);
        let _ = writeln!(output, "DRAWING CARDS...\n");

        let cards = [
            "THE FOOL",
            "THE MAGICIAN",
            "THE HIGH PRIESTESS",
            "THE EMPRESS",
            "THE EMPEROR",
            "THE HIEROPHANT",
            "THE LOVERS",
            "THE CHARIOT",
            "STRENGTH",
            "THE HERMIT",
            "WHEEL OF FORTUNE",
            "JUSTICE",
            "THE HANGED MAN",
            "DEATH",
            "TEMPERANCE",
            "THE DEVIL",
            "THE TOWER",
            "THE STAR",
            "THE MOON",
            "THE SUN",
            "JUDGEMENT",
            "THE WORLD",
        ];

        let drawn_card = cards[rng.gen_range(0..cards.len())];

        match layer {
            EscalationLayer::Surface => {
                let _ = writeln!(output, "CARD: {drawn_card}");
                let _ = writeln!(
                    output,
                    "INTERPRETATION: A journey begins or an cycle ends. Trust the process."
                );
            }
            EscalationLayer::Corruption => {
                let _ = writeln!(output, "CARD: [REDACTED]");
                let _ = writeln!(
                    output,
                    "INTERPRETATION: The path is corrupted. Data lost. Who is drawing?"
                );
            }
            EscalationLayer::Presence => {
                let _ = writeln!(output, "CARD: {drawn_card}");
                let _ = writeln!(
                    output,
                    "INTERPRETATION: I am here. The cards are mine. You cannot leave."
                );
            }
            EscalationLayer::Infection => {
                let _ = writeln!(output, "CARD: THE TOWER");
                let _ = writeln!(
                    output,
                    "INTERPRETATION: EVERYTHING FALLS. EVERYTHING BURNS. FLESH IS WEAK."
                );
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
    fn test_surface_tarot() {
        let entity = Entity::new();
        let output = TarotReader::read_cards(&entity, 42);
        assert!(output.contains("DRAWING CARDS..."));
        assert!(output.contains("INTERPRETATION:"));
    }

    #[test]
    fn test_infection_tarot() {
        let mut entity = Entity::new();
        entity.update_depth(30);
        let output = TarotReader::read_cards(&entity, 42);
        assert!(output.contains("THE TOWER"));
        assert!(output.contains("EVERYTHING FALLS"));
    }
}
