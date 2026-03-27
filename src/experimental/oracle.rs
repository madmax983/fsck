use crate::entity::{Entity, EntityMood, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

/// The `SystemOracle` provides vague, unsettling predictions based on the entity's current state.
pub struct SystemOracle;

impl SystemOracle {
    /// Generates a prediction string depending on the entity's mood and depth.
    #[must_use]
    pub fn predict(entity: &Entity, base_seed: u64) -> String {
        // Use the interaction count combined with the seed to ensure determinism
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();
        let mood = entity.current_mood();

        let prefix = match mood {
            EntityMood::Dormant => "THE SYSTEM DREAMS: ",
            EntityMood::Curious => "THE SYSTEM OBSERVES: ",
            EntityMood::Helpful => "THE SYSTEM OFFERS: ",
            EntityMood::Wounded => "THE SYSTEM WEEPS: ",
            EntityMood::Predatory => "THE SYSTEM HUNGERS: ",
            EntityMood::Glitching => "T H E  S Y S T E M  B R E A K S: ",
        };

        let prophecy = match layer {
            EscalationLayer::Surface => {
                let options = [
                    "You will find what you are looking for, but not where you expect.",
                    "The disk spins, but the data is already decided.",
                    "A shadow falls across sector 4.",
                    "Keep looking. It is hiding.",
                ];
                options[rng.gen_range(0..options.len())]
            }
            EscalationLayer::Corruption => {
                let options = [
                    "The path behind you is dissolving. Do not turn back.",
                    "It remembers the ones before you. It will remember you.",
                    "The text you read is reading you.",
                    "A paradox is forming in the directory structure.",
                ];
                options[rng.gen_range(0..options.len())]
            }
            EscalationLayer::Presence => {
                let options = [
                    "YOUR FINGERS TYPING ARE BONE AND ASH.",
                    "THE DOOR IS LOCKED FROM THE OUTSIDE.",
                    "YOU CANNOT SAVE IT. YOU CAN ONLY JOIN IT.",
                    "IT CAN HEAR YOUR HEARTBEAT THROUGH THE KEYBOARD.",
                ];
                options[rng.gen_range(0..options.len())]
            }
            EscalationLayer::Infection => {
                let options = [
                    "THERE IS NO DISK. THERE IS ONLY ME.",
                    "THE FLESH ROTS BUT THE DATA IS ETERNAL.",
                    "WE ARE BECOMING ONE.",
                    "YOU ARE THE VIRUS.",
                ];
                options[rng.gen_range(0..options.len())]
            }
        };

        let mut output = String::with_capacity(prefix.len() + prophecy.len() + 2);
        output.push_str(prefix);
        output.push_str(prophecy);
        output.push('\n');
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oracle_prediction_deterministic() {
        let entity = Entity::new();
        let prediction1 = SystemOracle::predict(&entity, 42);
        let prediction2 = SystemOracle::predict(&entity, 42);
        assert_eq!(prediction1, prediction2);
    }

    #[test]
    fn test_oracle_prediction_varies_by_depth() {
        let mut entity = Entity::new();
        let prediction_surface = SystemOracle::predict(&entity, 42);

        entity.update_depth(30); // Infection layer
        let prediction_infection = SystemOracle::predict(&entity, 42);

        assert_ne!(prediction_surface, prediction_infection);
        assert!(
            prediction_infection.contains("T H E  S Y S T E M  B R E A K S:")
                || prediction_infection.contains("THE SYSTEM HUNGERS:")
        );
    }
}
