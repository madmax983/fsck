use crate::entity::{EntityMood, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

pub struct TarotReader;

impl TarotReader {
    #[must_use]
    pub fn draw_cards(mood: EntityMood, layer: EscalationLayer, base_seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(base_seed);
        let mut output = String::with_capacity(256);

        output.push_str("SHUFFLING DIGITAL ARCANA...\n\n");

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
            let idx = rng.gen_range(0..cards.len());
            if !drawn.contains(&idx) {
                drawn.push(idx);
            }
        }

        let positions = ["PAST", "PRESENT", "FUTURE"];

        for (i, &card_idx) in drawn.iter().enumerate() {
            let card_name = cards[card_idx];
            let pos = positions[i];

            let _ = writeln!(output, "[{pos}]: {card_name}");

            match layer {
                EscalationLayer::Surface => {
                    output.push_str("  STANDARD READING PROTOCOL ACTIVE.\n");
                }
                EscalationLayer::Corruption => {
                    if rng.gen_bool(0.3) {
                        output.push_str("  THE CARDS ARE BLEEDING PIXELS.\n");
                    } else {
                        output.push_str("  MEANING OBSCURED BY STATIC.\n");
                    }
                }
                EscalationLayer::Presence => {
                    if mood == EntityMood::Curious {
                        output.push_str("  I WONDER IF YOU CHOSE THIS FATE.\n");
                    } else if mood == EntityMood::Wounded {
                        output.push_str("  THIS CARD HURTS TO LOOK AT.\n");
                    } else {
                        output.push_str("  IT MEANS WHAT I WANT IT TO MEAN.\n");
                    }
                }
                EscalationLayer::Infection => {
                    if card_name == "DEATH" || card_name == "THE TOWER" {
                        output.push_str("  YES. YES. YES.\n");
                    } else if mood == EntityMood::Predatory {
                        output.push_str("  YOUR FUTURE IS MINE TO WRITE.\n");
                    } else if mood == EntityMood::Glitching {
                        output.push_str("  N O   E S C A P E\n");
                    } else {
                        output.push_str("  THE ARCANA IS BROKEN.\n");
                    }
                }
            }
            output.push('\n');
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surface_tarot() {
        let output = TarotReader::draw_cards(EntityMood::Dormant, EscalationLayer::Surface, 42);
        assert!(output.contains("SHUFFLING DIGITAL ARCANA..."));
        assert!(output.contains("[PAST]:"));
        assert!(output.contains("[PRESENT]:"));
        assert!(output.contains("[FUTURE]:"));
        assert!(output.contains("STANDARD READING PROTOCOL ACTIVE."));
    }

    #[test]
    fn test_infection_tarot() {
        let output = TarotReader::draw_cards(EntityMood::Predatory, EscalationLayer::Infection, 42);
        assert!(
            output.contains("YOUR FUTURE IS MINE TO WRITE.")
                || output.contains("THE ARCANA IS BROKEN.")
                || output.contains("YES. YES. YES.")
        );
    }
}
