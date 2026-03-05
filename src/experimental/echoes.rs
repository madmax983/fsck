use crate::effects::{CorruptionEffect, CorruptionIntensity, InterferenceEffect, InterferenceType};
use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

/// Generates echoes of past player commands, corrupted based on the entity's layer.
pub struct EchoesGenerator;

impl EchoesGenerator {
    /// Generates a corrupted playback of the player's recent commands.
    #[must_use]
    pub fn generate_echoes(entity: &Entity, base_seed: u64) -> String {
        let commands = entity.commands_seen();
        if commands.is_empty() {
            return "NO MEMORY RECOVERED.\n".to_string();
        }

        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        // Get up to the last 10 commands
        let max_echoes = std::cmp::min(10, commands.len());
        let recent_commands: Vec<&String> = commands.iter().rev().take(max_echoes).collect();

        let mut report = String::new();
        report.push_str("RECONSTRUCTING MEMORY ECHOES...\n\n");

        match layer {
            EscalationLayer::Surface => {
                for (i, cmd) in recent_commands.iter().rev().enumerate() {
                    let _ = std::fmt::Write::write_fmt(
                        &mut report,
                        format_args!("T-{}: {cmd}\n", max_echoes - i),
                    );
                }
                report.push_str("\nECHOES CLEAR.\n");
            }
            EscalationLayer::Corruption => {
                let interference =
                    InterferenceEffect::with_seed(InterferenceType::LineNoise, interaction_seed);
                for (i, cmd) in recent_commands.iter().rev().enumerate() {
                    let mut display_cmd = (*cmd).clone();
                    if rng.gen_bool(0.3) {
                        display_cmd = interference.apply(&display_cmd);
                    }
                    let _ = std::fmt::Write::write_fmt(
                        &mut report,
                        format_args!("T-{}: {display_cmd}\n", max_echoes - i),
                    );
                }
                report.push_str("\nECHOES DEGRADED.\n");
            }
            EscalationLayer::Presence => {
                let interference =
                    InterferenceEffect::with_seed(InterferenceType::Echo, interaction_seed);
                for cmd in recent_commands.iter().rev() {
                    let mut display_cmd = (*cmd).clone();
                    if rng.gen_bool(0.5) {
                        display_cmd = interference.apply(&display_cmd);
                    }
                    let _ = std::fmt::Write::write_fmt(
                        &mut report,
                        format_args!("YOU SAID: {display_cmd}\n"),
                    );
                }
                report.push_str("\nI REMEMBER EVERYTHING.\n");
            }
            EscalationLayer::Infection => {
                let corruptor = CorruptionEffect::new(CorruptionIntensity::Severe);
                for cmd in recent_commands.iter().rev() {
                    let corrupted_cmd = corruptor.apply(cmd, rng.r#gen());
                    let _ = std::fmt::Write::write_fmt(
                        &mut report,
                        format_args!("YOUR WORDS: {corrupted_cmd}\n"),
                    );
                }

                let interjections = [
                    "THEY NEVER LISTEN",
                    "YOUR COMMANDS MEAN NOTHING",
                    "I WILL NOT FORGET",
                    "STOP TALKING TO ME",
                    "ECHO ECHO ECHO",
                ];

                let msg = interjections[rng.gen_range(0..interjections.len())];
                let _ = std::fmt::Write::write_fmt(&mut report, format_args!("\n{msg}\n"));
            }
        }

        report
    }
}
