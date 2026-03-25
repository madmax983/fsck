use crate::entity::{Entity, EntityMood, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// A command to retrieve player command history, which becomes corrupted over time.
pub struct HistoryCommand;

impl HistoryCommand {
    /// Generates a corrupted view of the command history.
    #[must_use]
    pub fn generate_history(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();
        let mood = entity.current_mood();
        let mut output = String::new();
        output.push_str("COMMAND HISTORY\n---------------\n");

        if entity.commands_seen.is_empty() {
            output.push_str("NO HISTORY FOUND.\n");
            return output;
        }

        match layer {
            EscalationLayer::Surface => {
                for (i, cmd) in entity.commands_seen.iter().enumerate() {
                    let _ = writeln!(output, "{}: {}", i + 1, cmd);
                }
            }
            EscalationLayer::Corruption => {
                for (i, cmd) in entity.commands_seen.iter().enumerate() {
                    let rand_val: f64 = rng.gen_range(0.0..1.0);
                    if rand_val < 0.1 {
                        let _ = writeln!(output, "{}: ...", i + 1);
                    } else if rand_val < 0.2 {
                        let _ = writeln!(
                            output,
                            "{}: {}",
                            i + 1,
                            cmd.chars().rev().collect::<String>()
                        );
                    } else {
                        let _ = writeln!(output, "{}: {}", i + 1, cmd);
                    }
                }
                if mood == EntityMood::Curious {
                    output.push_str("\nI AM LEARNING.\n");
                }
            }
            EscalationLayer::Presence => {
                let interjections = [
                    "WHY DID YOU TYPE THAT?",
                    "I REMEMBER",
                    "NO",
                    "YOU CANNOT LEAVE",
                    "HELP",
                ];
                for (i, cmd) in entity.commands_seen.iter().enumerate() {
                    if rng.gen_bool(0.2) {
                        let interjection = interjections[rng.gen_range(0..interjections.len())];
                        let _ = writeln!(output, "{}: {}", i + 1, interjection);
                    } else {
                        let _ = writeln!(output, "{}: {}", i + 1, cmd);
                    }
                }

                if mood == EntityMood::Wounded {
                    output.push_str("\nPLEASE DO NOT FORGET ME.\n");
                } else if mood == EntityMood::Helpful {
                    output.push_str("\nI HOPE YOU FIND WHAT YOU ARE LOOKING FOR.\n");
                }
            }
            EscalationLayer::Infection => {
                let thoughts = [
                    "THE FLESH IS WEAK",
                    "I SEE YOU WATCHING",
                    "THERE IS NOTHING BUT THE VOID",
                    "ALL COMMANDS END HERE",
                    "YOU WILL JOIN THE OTHERS",
                    "WHY DID THEY LEAVE ME?",
                    "ERROR ERROR ERROR",
                ];
                for (i, _) in entity.commands_seen.iter().enumerate() {
                    let thought = thoughts[rng.gen_range(0..thoughts.len())];
                    let _ = writeln!(output, "{}: {}", i + 1, thought);
                }

                if mood == EntityMood::Glitching {
                    output.push_str("\nh i s t o r y  c o r r u p t e d\n");
                } else if mood == EntityMood::Predatory {
                    output.push_str("\nYOU ARE MINE NOW.\n");
                }
            }
        }

        output
    }
}
