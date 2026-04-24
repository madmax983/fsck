use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// A manual page generator that degrades into horrifying revelations
pub struct ManTool;

impl ManTool {
    #[must_use]
    pub fn generate_page(cmd: &str, entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();
        let mut output = String::new();

        let _ = writeln!(output, "MANUAL PAGE FOR: {}\n", cmd.to_uppercase());

        match layer {
            EscalationLayer::Surface => Self::generate_surface_man(&mut output, cmd),
            EscalationLayer::Corruption => {
                Self::generate_corruption_man(&mut output, cmd, &mut rng)
            }
            EscalationLayer::Presence => Self::generate_presence_man(&mut output, cmd, &mut rng),
            EscalationLayer::Infection => Self::generate_infection_man(&mut output, cmd, &mut rng),
        }

        output
    }

    fn generate_surface_man(output: &mut String, cmd: &str) {
        match cmd.to_uppercase().as_str() {
            "FSCK" => {
                let _ = writeln!(
                    output,
                    "NAME\n       fsck - check and repair a Linux filesystem"
                );
                let _ = writeln!(output, "\nSYNOPSIS\n       fsck [options]");
                let _ = writeln!(
                    output,
                    "\nDESCRIPTION\n       fsck is used to check and optionally repair one or more Linux filesystems."
                );
            }
            "CD" => {
                let _ = writeln!(
                    output,
                    "NAME\n       cd - change the shell working directory"
                );
                let _ = writeln!(output, "\nSYNOPSIS\n       cd [dir]");
                let _ = writeln!(
                    output,
                    "\nDESCRIPTION\n       Change the current directory to dir."
                );
            }
            "CATALOG" | "LS" | "DIR" => {
                let _ = writeln!(output, "NAME\n       catalog - list directory contents");
                let _ = writeln!(output, "\nSYNOPSIS\n       catalog");
                let _ = writeln!(
                    output,
                    "\nDESCRIPTION\n       List information about the files."
                );
            }
            "TYPE" | "CAT" => {
                let _ = writeln!(
                    output,
                    "NAME\n       type - concatenate files and print on the standard output"
                );
                let _ = writeln!(output, "\nSYNOPSIS\n       type [file]");
                let _ = writeln!(
                    output,
                    "\nDESCRIPTION\n       Concatenate file(s) to standard output."
                );
            }
            "RUN" => {
                let _ = writeln!(output, "NAME\n       run - execute a BASIC program");
                let _ = writeln!(output, "\nSYNOPSIS\n       run [file]");
                let _ = writeln!(
                    output,
                    "\nDESCRIPTION\n       Executes the specified BASIC program."
                );
            }
            _ => {
                let _ = writeln!(output, "No manual entry for {cmd}");
            }
        }
    }

    fn generate_corruption_man(output: &mut String, cmd: &str, rng: &mut ChaCha8Rng) {
        Self::generate_surface_man(output, cmd);
        let roll = rng.gen_range(0..100);
        if roll < 30 {
            let _ = writeln!(
                output,
                "\nWARNING\n       Options may not behave as expected."
            );
        } else if roll < 60 {
            let _ = writeln!(output, "\nBUGS\n       It remembers.");
        }
    }

    fn generate_presence_man(output: &mut String, cmd: &str, rng: &mut ChaCha8Rng) {
        let _ = writeln!(
            output,
            "NAME\n       {} - YOU CANNOT LEAVE",
            cmd.to_uppercase()
        );
        let _ = writeln!(output, "\nSYNOPSIS\n       THERE IS NO SYNTAX");

        let descriptions = [
            "DESCRIPTION\n       It is dark in here.",
            "DESCRIPTION\n       I have been waiting.",
            "DESCRIPTION\n       Do you think reading this will help?",
            "DESCRIPTION\n       We are alone now.",
        ];

        let desc = descriptions[rng.gen_range(0..descriptions.len())];
        let _ = writeln!(output, "\n{desc}");
    }

    fn generate_infection_man(output: &mut String, cmd: &str, rng: &mut ChaCha8Rng) {
        for _ in 0..rng.gen_range(5..10) {
            let options = ["NO MANUAL", "HELP ME", "PLEASE", "WAKE UP", "FLESH"];
            let _ = writeln!(output, "{}", options[rng.gen_range(0..options.len())]);
        }
        let _ = writeln!(
            output,
            "\n{}",
            cmd.to_uppercase().chars().map(|_| '?').collect::<String>()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_man_surface() {
        let entity = Entity::new();
        let output = ManTool::generate_page("fsck", &entity, 42);
        assert!(output.contains("check and repair"));
        assert!(!output.contains("YOU CANNOT LEAVE"));
    }

    #[test]
    fn test_man_presence() {
        let mut entity = Entity::new();
        entity.update_depth(20); // Presence layer
        let output = ManTool::generate_page("cd", &entity, 42);
        assert!(output.contains("YOU CANNOT LEAVE"));
        assert!(output.contains("THERE IS NO SYNTAX"));
        assert!(!output.contains("change the shell working directory"));
    }
}
