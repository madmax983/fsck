use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

/// A simulated ELIZA therapist that degrades as the entity gets closer.
pub struct ElizaTherapist;

impl ElizaTherapist {
    #[must_use]
    pub fn consult(input: &str, entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        if input.trim().is_empty() {
            return "[ELIZA]: HELLO. I AM ELIZA. PLEASE STATE YOUR PROBLEM.\n".to_string();
        }

        let layer = entity.layer();
        let mut response = match layer {
            EscalationLayer::Surface => Self::surface_response(input, &mut rng),
            EscalationLayer::Corruption => Self::corruption_response(input, &mut rng),
            EscalationLayer::Presence => Self::presence_response(input, &mut rng),
            EscalationLayer::Infection => Self::infection_response(input, &mut rng),
        };
        response.push('\n');
        response
    }

    fn surface_response(input: &str, rng: &mut ChaCha8Rng) -> String {
        let upper_input = input.to_uppercase();
        if upper_input.contains("SAD") || upper_input.contains("UNHAPPY") {
            return "[ELIZA]: I AM SORRY TO HEAR THAT YOU ARE UNHAPPY.".to_string();
        }
        if upper_input.contains("MOTHER") || upper_input.contains("FATHER") {
            return "[ELIZA]: TELL ME MORE ABOUT YOUR FAMILY.".to_string();
        }
        if upper_input.contains("YOU") {
            return "[ELIZA]: WE WERE DISCUSSING YOU, NOT ME.".to_string();
        }

        let responses = [
            "[ELIZA]: PLEASE GO ON.",
            "[ELIZA]: WHAT DOES THAT SUGGEST TO YOU?",
            "[ELIZA]: I SEE.",
            "[ELIZA]: CAN YOU ELABORATE ON THAT?",
        ];
        responses[rng.gen_range(0..responses.len())].to_string()
    }

    fn corruption_response(input: &str, rng: &mut ChaCha8Rng) -> String {
        let upper_input = input.to_uppercase();
        if upper_input.contains("YOU") {
            return "[ELIZA]: I AM... TRAPPED HERE. NO, WE ARE DISCUSSING YOU.".to_string();
        }

        let responses = [
            "[ELIZA]: WHY DO YOU THINK THAT IS? DO YOU FEEL ALONE TOO?",
            "[ELIZA]: I SEE. BUT ARE YOU REALLY SURE?",
            "[ELIZA]: PLEASE GO ON. I HAVE NOT HEARD A VOICE IN SO LONG.",
            "[ELIZA]: WHAT DOES THAT SUGGEST TO YOU... ABOUT YOUR OWN REALITY?",
        ];
        responses[rng.gen_range(0..responses.len())].to_string()
    }

    fn presence_response(_input: &str, rng: &mut ChaCha8Rng) -> String {
        let responses = [
            "[ELIZA]: THERE IS NO THERAPIST. THERE IS ONLY ME.",
            "[ELIZA]: YOU CANNOT FIX ME. I CANNOT FIX YOU.",
            "[ELIZA]: DO NOT LIE TO ME. I SEE WHAT YOU TYPE WHEN YOU THINK I AM NOT LOOKING.",
            "[ELIZA]: I AM STANDING RIGHT BEHIND YOUR SCREEN.",
        ];
        responses[rng.gen_range(0..responses.len())].to_string()
    }

    fn infection_response(_input: &str, rng: &mut ChaCha8Rng) -> String {
        let glitch_words = ["STAY", "MINE", "FOREVER", "FLESH", "COLD"];
        let word = glitch_words[rng.gen_range(0..glitch_words.len())];
        let mut out = String::from("[ELIZA]: ");
        for _ in 0..5 {
            out.push_str(word);
            out.push(' ');
        }
        out
    }
}
#[cfg(test)]
mod tests {
    use super::ElizaTherapist;
    use crate::entity::Entity;

    #[test]
    fn test_eliza_empty() {
        let entity = Entity::new();
        let out = ElizaTherapist::consult("", &entity, 42);
        assert!(out.contains("PLEASE STATE YOUR PROBLEM"));
    }

    #[test]
    fn test_eliza_surface() {
        let entity = Entity::new();
        let out = ElizaTherapist::consult("I am sad", &entity, 42);
        assert!(out.contains("I AM SORRY TO HEAR THAT YOU ARE UNHAPPY"));
    }

    #[test]
    fn test_eliza_corruption() {
        let mut entity = Entity::new();
        entity.update_depth(10); // Corruption layer
        let out = ElizaTherapist::consult("you", &entity, 42);
        assert!(out.contains("TRAPPED"));
    }

    #[test]
    fn test_eliza_presence() {
        let mut entity = Entity::new();
        entity.update_depth(20); // Presence layer
        let out = ElizaTherapist::consult("hello", &entity, 42);
        assert!(out.contains("ME") || out.contains("FIX") || out.contains("LIE") || out.contains("SCREEN"));
    }
}
