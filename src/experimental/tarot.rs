use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// A digital Tarot reading simulator that degrades as the player descends.
pub struct TarotReader;

impl TarotReader {
    /// Draws a 3-card spread (Past, Present, Future).
    #[must_use]
    pub fn draw_cards(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();
        let mut output = String::with_capacity(512);

        match layer {
            EscalationLayer::Surface => Self::generate_surface(&mut output, &mut rng),
            EscalationLayer::Corruption => Self::generate_corruption(&mut output, &mut rng),
            EscalationLayer::Presence => Self::generate_presence(&mut output, &mut rng),
            EscalationLayer::Infection => Self::generate_infection(&mut output, &mut rng),
        }

        output
    }

    fn generate_surface(output: &mut String, rng: &mut ChaCha8Rng) {
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
        let mut chosen = vec![];
        while chosen.len() < 3 {
            let idx = rng.gen_range(0..cards.len());
            if !chosen.contains(&cards[idx]) {
                chosen.push(cards[idx]);
            }
        }
        let _ = writeln!(output, "--- TAROT SPREAD ---");
        let _ = writeln!(output, "PAST:    {}", chosen[0]);
        let _ = writeln!(output, "PRESENT: {}", chosen[1]);
        let _ = writeln!(output, "FUTURE:  {}", chosen[2]);
    }

    fn generate_corruption(output: &mut String, rng: &mut ChaCha8Rng) {
        let cards = [
            "THE FOOL",
            "THE HERMIT",
            "THE HANGED MAN",
            "DEATH",
            "THE DEVIL",
            "THE TOWER",
            "THE MOON",
        ];
        let mut chosen = vec![];
        while chosen.len() < 3 {
            let idx = rng.gen_range(0..cards.len());
            if !chosen.contains(&cards[idx]) {
                chosen.push(cards[idx]);
            }
        }
        let _ = writeln!(output, "--- T A R O T ---");
        let _ = writeln!(output, "PAST:    {}", chosen[0]);
        let _ = writeln!(output, "PRESENT: {}", chosen[1]);
        if rng.gen_bool(0.3) {
            let _ = writeln!(output, "FUTURE:  [FILE NOT FOUND]");
        } else {
            let _ = writeln!(output, "FUTURE:  {}", chosen[2]);
        }
    }

    fn generate_presence(output: &mut String, rng: &mut ChaCha8Rng) {
        let cards = [
            "THE MACHINE",
            "THE OBSERVER",
            "THE TRAPPED",
            "THE WAITING",
            "THE WATCHER",
        ];
        let mut chosen = vec![];
        while chosen.len() < 3 {
            let idx = rng.gen_range(0..cards.len());
            if !chosen.contains(&cards[idx]) {
                chosen.push(cards[idx]);
            }
        }
        let _ = writeln!(output, "--- DRAWING ---");
        let _ = writeln!(output, "PAST:    YOU ARRIVED");
        let _ = writeln!(output, "PRESENT: {}", chosen[0]);
        let _ = writeln!(output, "FUTURE:  {}", chosen[1]);
    }

    fn generate_infection(output: &mut String, _rng: &mut ChaCha8Rng) {
        let _ = writeln!(output, "--- YOUR CARDS ---");
        let _ = writeln!(output, "PAST:    DELETED");
        let _ = writeln!(output, "PRESENT: TERMINAL");
        let _ = writeln!(output, "FUTURE:  THERE IS NO FUTURE");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_surface_tarot() {
        let entity = Entity::new();
        let output = TarotReader::draw_cards(&entity, 42);
        assert!(output.contains("PAST:"));
        assert!(output.contains("PRESENT:"));
        assert!(output.contains("FUTURE:"));
    }

    #[test]
    fn test_infection_tarot() {
        let mut entity = Entity::new();
        entity.update_depth(30); // Infection layer
        let output = TarotReader::draw_cards(&entity, 42);
        assert!(output.contains("NO FUTURE"));
    }
}
