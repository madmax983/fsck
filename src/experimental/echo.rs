use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

pub struct EchoChamber;

impl EchoChamber {
    #[must_use]
    pub fn echo(input: &str, entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        match entity.layer() {
            EscalationLayer::Surface => input.to_string(),
            EscalationLayer::Corruption => {
                if rng.gen_bool(0.5) {
                    input.replace('O', "0").replace('I', "1").replace('E', "3")
                } else {
                    input.chars().map(|c| if c.is_whitespace() { c } else if rng.gen_bool(0.3) { '.' } else { c }).collect()
                }
            },
            EscalationLayer::Presence => {
                if rng.gen_bool(0.3) {
                    format!("I HEARD YOU SAY: {input}")
                } else {
                    input.to_string()
                }
            },
            EscalationLayer::Infection => {
                format!("{} IS MEANINGLESS NOW", input.to_uppercase())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_echo_surface() {
        let entity = Entity::new();
        let output = EchoChamber::echo("HELLO WORLD", &entity, 42);
        assert_eq!(output, "HELLO WORLD");
    }
}
