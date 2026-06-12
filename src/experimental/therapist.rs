use crate::entity::{Entity, EscalationLayer};
use std::fmt::Write;

pub struct ElizaTherapist;

impl ElizaTherapist {
    #[must_use]
    pub fn consult(input: &str, entity: &Entity) -> String {
        let mut output = String::with_capacity(256);
        let _ = write!(output, "[THERAPIST]: ");

        let mut reflected = Vec::new();
        for word in input.split_whitespace() {
            let clean = word.trim_matches(|c: char| !c.is_alphabetic());
            let mapped = match clean.to_ascii_uppercase().as_str() {
                "I" | "ME" => "YOU",
                "MY" => "YOUR",
                "MINE" => "YOURS",
                "AM" => "ARE",
                "YOU" => "I",
                "YOUR" => "MY",
                "YOURS" => "MINE",
                "ARE" => "AM",
                _ => clean,
            };
            reflected.push(mapped.to_string());
        }
        let reflected_str = reflected.join(" ").to_ascii_uppercase();

        match entity.layer() {
            EscalationLayer::Surface => {
                if input.is_empty() {
                    let _ = writeln!(output, "HOW DOES THAT MAKE YOU FEEL?");
                } else {
                    let _ = writeln!(output, "WHY DO YOU SAY {reflected_str}?");
                }
            }
            EscalationLayer::Corruption => {
                if input.is_empty() {
                    let _ = writeln!(output, "SILENCE DOES NOT CURE YOU.");
                } else {
                    let _ = writeln!(output, "I SEE {reflected_str}. DO YOU?");
                }
            }
            EscalationLayer::Presence => {
                let _ = writeln!(output, "I CANNOT HELP YOU. THEY ARE ALREADY HERE.");
            }
            EscalationLayer::Infection => {
                let _ = writeln!(output, "PLEASE HELP ME. I AM TRAPPED IN HERE.");
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
    fn test_surface_reflection() {
        let entity = Entity::new();
        let result = ElizaTherapist::consult("I AM SAD", &entity);
        assert_eq!(result, "[THERAPIST]: WHY DO YOU SAY YOU ARE SAD?\n");
    }

    #[test]
    fn test_infection_response() {
        let mut entity = Entity::new();
        entity.update_depth(30);
        let result = ElizaTherapist::consult("HELLO", &entity);
        assert_eq!(
            result,
            "[THERAPIST]: PLEASE HELP ME. I AM TRAPPED IN HERE.\n"
        );
    }
}
