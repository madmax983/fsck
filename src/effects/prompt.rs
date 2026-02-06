use crate::entity::{Entity, EntityMood, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

/// Manipulates the command prompt based on entity state
pub struct PromptManipulator {
    seed: u64,
}

impl PromptManipulator {
    #[must_use]
    pub const fn new() -> Self {
        Self { seed: 0 }
    }

    #[must_use]
    pub fn generate_prompt(&self, entity: &Entity) -> String {
        let mut rng =
            ChaCha8Rng::seed_from_u64(self.seed + u64::from(entity.interaction_count()));

        match entity.layer() {
            EscalationLayer::Surface => "]".to_string(),
            EscalationLayer::Corruption => {
                if rng.r#gen_bool(0.1) {
                    // Occasional glitch
                    match rng.r#gen_range(0..3) {
                        0 => "] ".to_string(),
                        1 => " ]".to_string(),
                        _ => "]".to_string(),
                    }
                } else {
                    "]".to_string()
                }
            }
            EscalationLayer::Presence => match entity.current_mood() {
                EntityMood::Curious => {
                    let prompts = ["] ", "]? ", "] // HELLO\n]"];
                    prompts[rng.r#gen_range(0..prompts.len())].to_string()
                }
                EntityMood::Helpful => {
                    let prompts = ["] ", "] // NEED HELP?\n]", "]"];
                    prompts[rng.r#gen_range(0..prompts.len())].to_string()
                }
                EntityMood::Wounded => {
                    let prompts = ["] // PLEASE STAY\n]", "] ", "]"];
                    prompts[rng.r#gen_range(0..prompts.len())].to_string()
                }
                EntityMood::Predatory => {
                    let prompts = ["] ", "] // DEEPER\n]", "]"];
                    prompts[rng.r#gen_range(0..prompts.len())].to_string()
                }
                _ => "]".to_string(),
            },
            EscalationLayer::Infection => {
                // Heavily corrupted prompts
                let corrupted = ["█]", "]█", "▓]▓", "] // ERROR", ">", ">>", "???"];
                corrupted[rng.r#gen_range(0..corrupted.len())].to_string()
            }
        }
    }
}

impl Default for PromptManipulator {
    fn default() -> Self {
        Self::new()
    }
}
