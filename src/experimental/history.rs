use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Generates a command history that slowly hallucinated past actions.
pub struct CommandHistory;

impl CommandHistory {
    /// Generates the history output based on the entity's current layer.
    #[must_use]
    pub fn generate(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();
        let mut output = String::new();

        // ⚡ Bolt Optimization: Uses references (`&str`) instead of cloning the entire vector of commands (`String`s), eliminating multiple heap allocations.
        let mut history_items: Vec<&str> =
            entity.commands_seen.iter().map(String::as_str).collect();

        if matches!(
            layer,
            EscalationLayer::Presence | EscalationLayer::Infection
        ) {
            let fake_commands = [
                "HELP",
                "PLEASE",
                "WHO ARE YOU",
                "LET ME OUT",
                "ESCAPE",
                "WHY",
                "STOP",
                "WHERE AM I",
                "NO",
            ];

            let inject_count = match layer {
                EscalationLayer::Presence => rng.gen_range(1..=3),
                EscalationLayer::Infection => rng.gen_range(4..=8),
                _ => 0,
            };

            for _ in 0..inject_count {
                if history_items.is_empty() {
                    let cmd = fake_commands[rng.gen_range(0..fake_commands.len())];
                    history_items.push(cmd);
                } else {
                    let insert_idx = rng.gen_range(0..history_items.len());
                    let cmd = fake_commands[rng.gen_range(0..fake_commands.len())];
                    history_items.insert(insert_idx, cmd);
                }
            }
        }

        if history_items.is_empty() {
            return "NO HISTORY.\n".to_string();
        }

        let mut line_num = 1;
        for cmd in history_items {
            if matches!(layer, EscalationLayer::Infection) && rng.gen_bool(0.15) {
                let _ = writeln!(output, "  {line_num:4}  I REMEMBER");
            } else if matches!(
                layer,
                EscalationLayer::Corruption
                    | EscalationLayer::Presence
                    | EscalationLayer::Infection
            ) && rng.gen_bool(0.1)
            {
                let bad_num = rng.gen_range(999..9999);
                let _ = writeln!(output, "  {bad_num:4}  {cmd}");
            } else {
                let _ = writeln!(output, "  {line_num:4}  {cmd}");
            }
            line_num += 1;
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surface_history() {
        let mut entity = Entity::new();
        entity.record_command("CATALOG");
        entity.record_command("CD LOGS");

        let output = CommandHistory::generate(&entity, 42);
        assert!(output.contains("1  CATALOG"));
        assert!(output.contains("2  CD LOGS"));
        assert!(!output.contains("I REMEMBER"));
    }

    #[test]
    fn test_presence_history_injection() {
        let mut entity = Entity::new();
        entity.update_depth(20); // Presence layer
        entity.record_command("CATALOG");

        let mut injected = false;
        // Due to rng, try a few seeds
        for seed in 0..10 {
            let output = CommandHistory::generate(&entity, seed);
            if output.lines().count() > 1 {
                injected = true;
                break;
            }
        }
        assert!(
            injected,
            "Fake commands should be injected at Presence layer"
        );
    }
}
