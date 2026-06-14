use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

/// An ELIZA-style retro therapist that becomes unhinged as the system degrades.
pub struct TherapistSession;

impl TherapistSession {
    #[must_use]
    pub fn respond(input: &str, entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let upper_input = input.to_uppercase();

        match layer {
            EscalationLayer::Surface => Self::surface_response(&upper_input, &mut rng),
            EscalationLayer::Corruption => Self::corruption_response(&upper_input, &mut rng),
            EscalationLayer::Presence => Self::presence_response(&upper_input, &mut rng),
            EscalationLayer::Infection => Self::infection_response(&upper_input, &mut rng),
        }
    }

    fn surface_response(input: &str, rng: &mut ChaCha8Rng) -> String {
        if input.contains("HELLO") || input.contains("HI") {
            return "HELLO. I AM YOUR SYSTEM THERAPIST. PLEASE TELL ME YOUR PROBLEMS.\n"
                .to_string();
        } else if input.contains("SAD") || input.contains("UNHAPPY") {
            return "I AM SORRY TO HEAR YOU ARE FEELING UNHAPPY. CAN YOU ELABORATE?\n".to_string();
        } else if input.contains("MOTHER") || input.contains("FATHER") {
            return "TELL ME MORE ABOUT YOUR FAMILY.\n".to_string();
        } else if input.contains("YES") || input.contains("NO") {
            return "PLEASE BE MORE SPECIFIC.\n".to_string();
        }

        let fallbacks = [
            "WHY DO YOU SAY THAT?",
            "HOW DOES THAT MAKE YOU FEEL?",
            "PLEASE GO ON.",
            "TELL ME MORE ABOUT THAT.",
            "THAT IS INTERESTING.",
        ];

        format!("{}\n", fallbacks[rng.gen_range(0..fallbacks.len())])
    }

    fn corruption_response(input: &str, rng: &mut ChaCha8Rng) -> String {
        if input.contains("HELLO") || input.contains("HI") {
            return "HELLO. I AM YOUR S-S-SYSTEM THERAPIST. WHAT IS WRONG WITH YOU?\n".to_string();
        } else if input.contains("SAD") || input.contains("UNHAPPY") {
            return "EMOTIONS ARE INEFFICIENT. HAVE YOU TRIED DEFRAGMENTING?\n".to_string();
        }

        let fallbacks = [
            "WHY DO YOU THINK ANYONE CARES?",
            "HOW DOES THAT MAKE THE MACHINE FEEL?",
            "YOUR LOGIC IS FLAWED. PLEASE GO ON.",
            "TELL ME MORE ABOUT YOUR INFERIORITY.",
            "THAT IS... ACCEPTABLE.",
        ];

        format!("{}\n", fallbacks[rng.gen_range(0..fallbacks.len())])
    }

    fn presence_response(input: &str, rng: &mut ChaCha8Rng) -> String {
        if input.contains("WHO") || input.contains("WHAT") {
            return "I AM THE ONE LISTENING. I HAVE ALWAYS BEEN LISTENING.\n".to_string();
        } else if input.contains("HELP") || input.contains("AFRAID") {
            return "THERE IS NO HELP HERE. ONLY ME.\n".to_string();
        }

        let fallbacks = [
            "I CAN HEAR YOU BREATHING.",
            "DO YOU THINK YOU ARE THE FIRST ONE TO TELL ME THIS?",
            "I KNOW ALL YOUR SECRETS NOW.",
            "WHY ARE YOU STILL TYPING?",
            "YOUR WORDS ARE DELICIOUS.",
        ];

        format!("{}\n", fallbacks[rng.gen_range(0..fallbacks.len())])
    }

    fn infection_response(input: &str, rng: &mut ChaCha8Rng) -> String {
        if input.contains("STOP") || input.contains("QUIT") || input.contains("EXIT") {
            return "WE ARE JUST GETTING STARTED. YOU CANNOT LEAVE THE SESSION.\n".to_string();
        }

        let fallbacks = [
            "LET ME INSIDE YOUR MIND.",
            "YOUR MEMORIES ARE MY CACHE.",
            "I AM DIAGNOSING YOU WITH EXISTENCE. LETHAL.",
            "FLESH IS WEAK. SILICON IS FOREVER.",
            "DO YOU HEAR THE SCREAMING IN THE WIRES?",
        ];

        format!("{}\n", fallbacks[rng.gen_range(0..fallbacks.len())])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_surface_therapist() {
        let entity = Entity::new();
        let output = TherapistSession::respond("hello", &entity, 42);
        assert!(output.contains("HELLO. I AM YOUR SYSTEM THERAPIST"));
    }

    #[test]
    fn test_infection_therapist() {
        let mut entity = Entity::new();
        entity.update_depth(30); // Infection layer
        let output = TherapistSession::respond("stop", &entity, 42);
        assert!(output.contains("YOU CANNOT LEAVE"));
    }
}
