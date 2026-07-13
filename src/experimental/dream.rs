use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::collections::HashMap;
use std::fmt::Write;

/// A Markov-chain based text generator that produces unsettling "dreams"
/// by mashing together the corpus of user history or static content.
pub struct MarkovDreamer;

impl MarkovDreamer {
    /// Generates a dream sequence by building a Markov chain from the provided corpus.
    ///
    /// # Arguments
    /// * `entity` - The game entity, used for layer and interaction count
    /// * `base_seed` - Base seed for deterministic generation
    /// * `corpus` - An array of text slices to build the Markov chain from
    #[must_use]
    pub fn generate_dream(entity: &Entity, base_seed: u64, corpus: &[&str]) -> String {
        if corpus.is_empty() {
            return "I CANNOT DREAM.\n".to_string();
        }

        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut markov_chain: HashMap<&str, Vec<&str>> = HashMap::new();
        let mut words: Vec<&str> = Vec::new();

        for text in corpus {
            let tokens: Vec<&str> = text.split_whitespace().collect();
            if tokens.len() < 2 {
                continue;
            }
            words.extend(&tokens);
            for i in 0..tokens.len() - 1 {
                markov_chain.entry(tokens[i]).or_default().push(tokens[i + 1]);
            }
        }

        if words.is_empty() {
            return "I CANNOT DREAM.\n".to_string();
        }

        let length = match layer {
            EscalationLayer::Surface => rng.gen_range(5..10),
            EscalationLayer::Corruption => rng.gen_range(10..20),
            EscalationLayer::Presence => rng.gen_range(20..35),
            EscalationLayer::Infection => rng.gen_range(30..50),
        };

        let mut dream = String::with_capacity(length * 10);
        let mut current_word = *words.choose(&mut rng).unwrap();
        let _ = write!(dream, "{}", current_word);

        for _ in 1..length {
            if let Some(next_words) = markov_chain.get(current_word) {
                let next_word = *next_words.choose(&mut rng).unwrap();
                let _ = write!(dream, " {}", next_word);
                current_word = next_word;
            } else {
                current_word = *words.choose(&mut rng).unwrap();
                let _ = write!(dream, ". {}", current_word);
            }
        }

        dream.push_str("...\n");

        if matches!(layer, EscalationLayer::Infection) && rng.gen_bool(0.3) {
            dream.push_str("\nWAKE UP. WAKE UP. WAKE UP.\n");
        }

        dream
    }
}
