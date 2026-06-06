#[cfg(feature = "nova")]
use crate::entity::Entity;
#[cfg(feature = "nova")]
use crate::experimental::EmotionalBleed;
#[cfg(feature = "nova")]
use crate::filesystem::FilesystemGraph;
#[cfg(feature = "nova")]
use rand::prelude::*;
#[cfg(feature = "nova")]
use rand_chacha::ChaCha8Rng;

#[cfg(feature = "nova")]
pub struct SystemOracle;

#[cfg(feature = "nova")]
impl SystemOracle {
    #[must_use]
    pub fn consult(
        question: &str,
        fs: &FilesystemGraph,
        entity: &Entity,
        base_seed: u64,
    ) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let files: Vec<_> = fs.current_node().visible_files().collect();

        let answer_base = if files.is_empty() {
            "THE EMPTINESS OFFERS NO ANSWERS.".to_string()
        } else {
            let chosen_file = files[rng.gen_range(0..files.len())];
            let content = chosen_file.read();

            let lines: Vec<&str> = content.lines().filter(|l| !l.trim().is_empty()).collect();
            if lines.is_empty() {
                "SILENCE.".to_string()
            } else {
                let chosen_line = lines[rng.gen_range(0..lines.len())];
                chosen_line.to_string()
            }
        };

        let bled_answer =
            EmotionalBleed::inject_emotion(&answer_base, entity.current_mood(), &mut rng);

        let mut output = String::with_capacity(bled_answer.len() + 128);
        output.push_str("[ORACLE CONSULTATION]\n");
        output.push_str("QUESTION: ");
        output.push_str(if question.is_empty() { "..." } else { question });
        output.push('\n');
        output.push_str("ANSWER: ");
        output.push_str(&bled_answer);
        output.push('\n');

        output
    }
}

#[cfg(all(test, feature = "nova"))]
mod tests {
    use super::*;
    use crate::filesystem::FileNode;

    #[test]
    fn test_oracle_answers() {
        let mut fs = FilesystemGraph::new();
        let entity = Entity::new();

        fs.current_node_mut()
            .add_file(FileNode::new("TEST.TXT", "I am the answer.\nAnother line."));

        let answer = SystemOracle::consult("Who are you?", &fs, &entity, 42);
        assert!(answer.contains("[ORACLE CONSULTATION]"));
        assert!(answer.contains("QUESTION: Who are you?"));
        assert!(answer.contains("ANSWER: "));
        assert!(answer.contains("I am the answer.") || answer.contains("Another line."));
    }
}
