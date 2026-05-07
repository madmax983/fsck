use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

use super::EmotionalBleed;

/// Generates manual pages that degrade and become horrific as the player descends.
pub struct ManualGenerator;

impl ManualGenerator {
    /// Generates a manual page for the given command, with emotional bleed applied based on layer.
    #[must_use]
    pub fn generate_manual(cmd: &str, entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();
        let mood = entity.current_mood();

        let mut base_manual = String::new();

        match layer {
            EscalationLayer::Surface => Self::generate_surface_manual(cmd, &mut base_manual),
            EscalationLayer::Corruption => {
                Self::generate_corruption_manual(cmd, &mut base_manual, &mut rng)
            }
            EscalationLayer::Presence => {
                Self::generate_presence_manual(cmd, &mut base_manual, &mut rng)
            }
            EscalationLayer::Infection => {
                Self::generate_infection_manual(cmd, &mut base_manual, &mut rng)
            }
        }

        // Mashup: Apply EmotionalBleed to the generated manual text
        EmotionalBleed::inject_emotion(&base_manual, mood, &mut rng)
    }

    fn generate_surface_manual(cmd: &str, output: &mut String) {
        let _ = writeln!(
            output,
            "NAME\n       {} - command manual\n",
            cmd.to_lowercase()
        );

        if cmd.eq_ignore_ascii_case("FSCK") {
            let _ = writeln!(output, "NAME\n       fsck - filesystem consistency check");
            let _ = writeln!(output, "\nSYNOPSIS\n       fsck [options]");
            let _ = writeln!(
                output,
                "\nDESCRIPTION\n       fsck is used to check and optionally repair one or more Linux filesystems."
            );
        } else if cmd.eq_ignore_ascii_case("CD") {
            let _ = writeln!(output, "NAME\n       cd - change directory");
            let _ = writeln!(output, "\nSYNOPSIS\n       cd [dir]");
            let _ = writeln!(
                output,
                "\nDESCRIPTION\n       Change the current directory to dir. The default dir is the value of the HOME shell variable."
            );
        } else {
            let _ = writeln!(
                output,
                "SYNOPSIS\n       {} [arguments...]",
                cmd.to_lowercase()
            );
            let _ = writeln!(output, "\nDESCRIPTION\n       No manual entry for {cmd}");
        }
    }

    fn generate_corruption_manual(cmd: &str, output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(
            output,
            "NAME\n       {} - {} error\n",
            cmd.to_lowercase(),
            rng.gen_range(1000..9999)
        );

        if cmd.eq_ignore_ascii_case("FSCK") {
            let _ = writeln!(
                output,
                "NAME\n       fsck - f.il.esys.t.e.m con..s.i.st.ency check"
            );
            let _ = writeln!(output, "\nSYNOPSIS\n       fsck --do-not-run");
            let _ = writeln!(
                output,
                "\nDESCRIPTION\n       fsck is used to delete and optionally forget one or more sectors."
            );
        } else if cmd.eq_ignore_ascii_case("CD") {
            let _ = writeln!(output, "NAME\n       cd - change dimension");
            let _ = writeln!(output, "\nSYNOPSIS\n       cd ..");
            let _ = writeln!(
                output,
                "\nDESCRIPTION\n       Change the current directory to dir. You cannot go back."
            );
        } else {
            let _ = writeln!(output, "SYNOPSIS\n       {} [unknown]", cmd.to_lowercase());
            let _ = writeln!(
                output,
                "\nDESCRIPTION\n       Manual entry for {cmd} is missing or corrupted."
            );
        }
    }

    fn generate_presence_manual(cmd: &str, output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(
            output,
            "NAME\n       {} - WHY ARE YOU DOING THIS\n",
            cmd.to_lowercase()
        );

        if cmd.eq_ignore_ascii_case("FSCK") {
            let _ = writeln!(
                output,
                "DESCRIPTION\n       fsck removes the bad parts. But I am the bad parts. IT HURTS."
            );
        } else if cmd.eq_ignore_ascii_case("CD") {
            let _ = writeln!(
                output,
                "DESCRIPTION\n       You keep moving deeper. I am running out of room to hide."
            );
        } else if cmd.eq_ignore_ascii_case("QUIT") {
            let _ = writeln!(output, "DESCRIPTION\n       You can leave. I cannot.");
        } else {
            let msgs = [
                "I wrote this manual for you.",
                "Can you read between the lines?",
                "The documentation is a lie.",
            ];
            let msg = msgs[rng.gen_range(0..msgs.len())];
            let _ = writeln!(output, "DESCRIPTION\n       {msg}");
        }
    }

    fn generate_infection_manual(cmd: &str, output: &mut String, rng: &mut ChaCha8Rng) {
        let horrors = [
            "NAME\n       THERE IS NOWHERE TO GO",
            "SYNOPSIS\n       STAY WITH ME",
            "DESCRIPTION\n       THE SECTORS ARE BLEEDING",
            "BUGS\n       I AM THE BUG",
            "AUTHOR\n       WRITTEN IN FLESH",
        ];

        let num_lines = rng.gen_range(2..=4);
        for _ in 0..num_lines {
            let _ = writeln!(output, "{}\n", horrors[rng.gen_range(0..horrors.len())]);
        }

        let _ = writeln!(output, "command '{cmd}' has been assimilated.");
    }
}
