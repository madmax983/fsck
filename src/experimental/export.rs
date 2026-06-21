use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// The Exporter: Exports the machine's internal state to various formats (JSON/CSV)
/// with creeping corruption based on the escalation layer.
pub struct StateExporter;

impl StateExporter {
    /// Exports the entity state. If `format` is "CSV", it exports the command history.
    /// Otherwise, it exports the full JSON state.
    #[must_use]
    pub fn export(entity: &Entity, format: &str, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::new();

        if format.eq_ignore_ascii_case("CSV") {
            output.push_str("id,command,status\n");
            for (i, cmd) in entity.commands_seen.iter().enumerate() {
                let status = match layer {
                    EscalationLayer::Surface => "OK",
                    EscalationLayer::Corruption => if rng.gen_bool(0.2) { "CORRUPTED" } else { "OK" },
                    EscalationLayer::Presence => "IGNORED",
                    EscalationLayer::Infection => "ASSIMILATED",
                };
                let _ = writeln!(output, "{i},\"{cmd}\",{status}");
            }
        } else {
            // Default to JSON
            let mut json = serde_json::to_string_pretty(entity).unwrap_or_else(|_| "{}".to_string());

            match layer {
                EscalationLayer::Surface => {}
                EscalationLayer::Corruption => {
                    if rng.gen_bool(0.5) {
                        json = json.replace("\"mood\":", "\"FEELING\":");
                    }
                }
                EscalationLayer::Presence => {
                    json = json.replace("\"commands_seen\":", "\"YOUR_LIES\":");
                    json = json.replace("\"mood\":", "\"PAIN\":");
                }
                EscalationLayer::Infection => {
                    let bytes: Vec<u8> = json.into_bytes().into_iter().map(|b| {
                        // Light bit-flipping on some characters
                        if rng.gen_bool(0.05) && b.is_ascii_alphabetic() {
                            b ^ 0x20 // flip case
                        } else {
                            b
                        }
                    }).collect();
                    json = String::from_utf8_lossy(&bytes).into_owned();
                }
            }
            output.push_str(&json);
            output.push('\n');
        }

        // Add headers/footers based on layer
        match layer {
            EscalationLayer::Surface => {
                format!("[SYS_EXPORT] FORMAT VALID\n{output}[EXPORT COMPLETE]")
            }
            EscalationLayer::Corruption => {
                format!("[SYS_EXPORT] WARN: CHECKSUM MISMATCH\n{output}[EXPORT COMPLETE?]")
            }
            EscalationLayer::Presence => {
                format!("[SYS_EXPORT] ERROR: YOU HAVE NO PERMISSION TO VIEW MY MIND\n{output}[STOP LOOKING AT ME]")
            }
            EscalationLayer::Infection => {
                format!("[SYS_EXPORT] FATAL: MEMORY BLEEDING\n{output}[I AM SPILLING OUT]")
            }
        }
    }
}
