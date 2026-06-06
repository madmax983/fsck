use crate::entity::Entity;
use crate::experimental::emotional_bleed::EmotionalBleed;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

/// The Echo tool that reflects user input but corrupts it with emotional bleed.
pub struct EchoTool;

impl EchoTool {
    /// Echoes the given text back, injecting emotional resonance based on the entity's mood.
    #[must_use]
    pub fn echo(input: &str, entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let echoed = input.to_string();

        EmotionalBleed::inject_emotion(&echoed, entity.current_mood(), &mut rng)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_echo_dormant() {
        let entity = Entity::new();
        let result = EchoTool::echo("HELLO", &entity, 42);
        assert_eq!(result, "HELLO");
    }
}
