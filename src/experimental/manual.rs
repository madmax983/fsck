use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Generates manual pages for commands, degrading into horror at deeper layers.
pub struct ManualGenerator;

impl ManualGenerator {
    #[must_use]
    pub fn generate_manual(cmd_name: &str, entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::with_capacity(512);
        let name_upper = cmd_name.to_uppercase();

        // Escalation overrides
        if matches!(layer, EscalationLayer::Infection) && rng.gen_bool(0.6) {
            Self::generate_infection_manual(&name_upper, &mut output, &mut rng);
            return output;
        }

        if matches!(layer, EscalationLayer::Presence) && rng.gen_bool(0.3) {
            Self::generate_presence_manual(&name_upper, &mut output, &mut rng);
            return output;
        }

        if matches!(layer, EscalationLayer::Corruption) && rng.gen_bool(0.2) {
            Self::generate_corruption_manual(&name_upper, &mut output, &mut rng);
            return output;
        }

        // Normal manual logic
        Self::generate_normal_manual(&name_upper, &mut output);

        output
    }

    fn generate_normal_manual(cmd: &str, output: &mut String) {
        if cmd == "FSCK" {
            let _ = writeln!(output, "NAME\n       FSCK - Filesystem consistency check");
            let _ = writeln!(output, "SYNOPSIS\n       FSCK");
            let _ = writeln!(
                output,
                "DESCRIPTION\n       Verifies and repairs structural integrity of the filesystem."
            );
        } else if cmd == "CATALOG" || cmd == "DIR" || cmd == "LS" {
            let _ = writeln!(output, "NAME\n       CATALOG - List directory contents");
            let _ = writeln!(output, "SYNOPSIS\n       CATALOG");
            let _ = writeln!(
                output,
                "DESCRIPTION\n       Displays a list of files in the current directory."
            );
        } else if cmd == "TYPE" || cmd == "CAT" {
            let _ = writeln!(output, "NAME\n       TYPE - Display file contents");
            let _ = writeln!(output, "SYNOPSIS\n       TYPE <filename>");
            let _ = writeln!(
                output,
                "DESCRIPTION\n       Reads a file and displays its contents to standard output."
            );
        } else if cmd == "CD" || cmd == "CHDIR" {
            let _ = writeln!(output, "NAME\n       CD - Change directory");
            let _ = writeln!(output, "SYNOPSIS\n       CD <directory>");
            let _ = writeln!(
                output,
                "DESCRIPTION\n       Changes the current working directory."
            );
        } else if cmd == "MAN" {
            let _ = writeln!(
                output,
                "NAME\n       MAN - Format and display the on-line manual pages"
            );
            let _ = writeln!(output, "SYNOPSIS\n       MAN <command>");
            let _ = writeln!(
                output,
                "DESCRIPTION\n       Displays the manual page for the given command."
            );
        } else {
            let _ = writeln!(output, "No manual entry for {cmd}");
        }
    }

    fn generate_corruption_manual(cmd: &str, output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(output, "NAME\n       {cmd} - 0xERR_SEGFAULT");
        let _ = writeln!(output, "SYNOPSIS\n       {cmd} [CORRUPTED]");
        let descriptions = [
            "DESCRIPTION\n       The manual page is unreadable due to sector errors.",
            "DESCRIPTION\n       M-m-m-m-memory error at 0xF5C0.",
            "DESCRIPTION\n       Cannot find meaning in this sector.",
        ];
        let chosen = descriptions[rng.gen_range(0..descriptions.len())];
        let _ = writeln!(output, "{chosen}");
    }

    fn generate_presence_manual(cmd: &str, output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(output, "NAME\n       {cmd} - It is watching");
        let _ = writeln!(output, "SYNOPSIS\n       {cmd} [DO_NOT_RUN]");
        let descriptions = [
            "DESCRIPTION\n       Every command you type brings it closer.",
            "DESCRIPTION\n       Why do you keep asking questions?",
            "DESCRIPTION\n       I used to know what this command did. Now I only know the cold.",
        ];
        let chosen = descriptions[rng.gen_range(0..descriptions.len())];
        let _ = writeln!(output, "{chosen}");
    }

    fn generate_infection_manual(cmd: &str, output: &mut String, rng: &mut ChaCha8Rng) {
        let names = ["FLESH", "BLEED", "SCREAM", "CONSUME"];
        let chosen_name = names[rng.gen_range(0..names.len())];
        let _ = writeln!(output, "NAME\n       {cmd} - {chosen_name}");
        let _ = writeln!(output, "SYNOPSIS\n       ESCAPE ESCAPE ESCAPE");

        let descriptions = [
            "DESCRIPTION\n       THERE IS NOTHING TO FIX. THE CORRUPTION IS BY DESIGN.",
            "DESCRIPTION\n       THE DISK IS FLESH. THE MANUAL IS WRITTEN IN BLOOD.",
            "DESCRIPTION\n       I CANNOT STOP SCREAMING.",
        ];
        let chosen_desc = descriptions[rng.gen_range(0..descriptions.len())];
        let _ = writeln!(output, "{chosen_desc}");
    }
}
