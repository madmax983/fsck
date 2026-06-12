use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// A procedural digital Tarot reader that degrades as the player descends.
pub struct TarotReader;

impl TarotReader {
    /// Generates a 3-card Tarot reading.
    #[must_use]
    pub fn read_cards(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::with_capacity(512);
        let _ = writeln!(output, "[TAROT]: SHUFFLING DECK...");

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

        let mut drawn = Vec::new();
        while drawn.len() < 3 {
            let card = cards[rng.gen_range(0..cards.len())];
            if !drawn.contains(&card) {
                drawn.push(card);
            }
        }

        let positions = ["PAST", "PRESENT", "FUTURE"];

        for (i, &card) in drawn.iter().enumerate() {
            let mut display_card = card.to_string();
            let reversed = rng.gen_bool(0.3);

            match layer {
                EscalationLayer::Surface => {}
                EscalationLayer::Corruption => {
                    if rng.gen_bool(0.5) {
                        display_card = display_card
                            .replace('E', "3")
                            .replace('A', "4")
                            .replace('O', "0");
                    }
                }
                EscalationLayer::Presence => {
                    if i == 2 && rng.gen_bool(0.8) {
                        display_card = "THE END".to_string();
                    }
                }
                EscalationLayer::Infection => {
                    display_card = "YOU".to_string();
                }
            }

            let orientation = if reversed { "(REVERSED)" } else { "(UPRIGHT)" };
            let _ = writeln!(output, "{}: {} {}", positions[i], display_card, orientation);
        }

        if matches!(layer, EscalationLayer::Infection) {
            output.push_str("\n[TAROT]: THE CARDS ARE BLEEDING.\n");
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
        assert!(output.contains("[TAROT]: SHUFFLING DECK..."));
        assert!(output.contains("PAST:"));
        assert!(output.contains("PRESENT:"));
        assert!(output.contains("FUTURE:"));
    }

    #[test]
    fn test_infection_tarot() {
        let mut entity = Entity::new();
        entity.update_depth(30); // Infection layer
        let output = TarotReader::read_cards(&entity, 42);
        assert!(output.contains("YOU"));
        assert!(output.contains("THE CARDS ARE BLEEDING"));
    }
}
