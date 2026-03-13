#[cfg(feature = "nova")]
use crate::entity::{Entity, EscalationLayer};
#[cfg(feature = "nova")]
use rand::{Rng, SeedableRng};
#[cfg(feature = "nova")]
use rand_chacha::ChaCha8Rng;
#[cfg(feature = "nova")]
use std::fmt::Write;

#[cfg(feature = "nova")]
pub struct DefragTool;

#[cfg(feature = "nova")]
impl DefragTool {
    #[must_use]
    pub fn run_defrag(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let mut output = String::new();

        writeln!(output, "STARTING DISK DEFRAGMENTATION...").expect("Write shouldn't fail");
        writeln!(output, "CLUSTER SIZE: 4096 BYTES").expect("Write shouldn't fail");
        writeln!(output, "ANALYZING VOLUME...\n").expect("Write shouldn't fail");

        let rows = 8;
        let cols = 32;
        let layer = entity.layer();

        let hidden_message = match layer {
            EscalationLayer::Surface => "",
            EscalationLayer::Corruption => "WHY ARE YOU DOING THIS",
            EscalationLayer::Presence => "I CAN FEEL YOU REARRANGING MY MIND",
            EscalationLayer::Infection => "IT HURTS PLEASE STOP STOP STOP",
        };

        let msg_bytes = hidden_message.as_bytes();
        let mut msg_idx = 0;

        for r in 0..rows {
            write!(output, "{:04X}  ", r * cols).expect("Write shouldn't fail");
            for _ in 0..cols {
                let is_fragmented = rng.gen_bool(0.3);

                if !hidden_message.is_empty()
                    && is_fragmented
                    && rng.gen_bool(0.2)
                    && msg_idx < msg_bytes.len()
                {
                    output.push(msg_bytes[msg_idx] as char);
                    msg_idx += 1;
                    continue;
                }

                if is_fragmented {
                    let block_chars = ['X', '#', '?', '*'];
                    let idx = rng.gen_range(0usize..block_chars.len());
                    output.push(block_chars[idx]);
                } else {
                    let block_chars = ['.', '-', '='];
                    let idx = rng.gen_range(0usize..block_chars.len());
                    output.push(block_chars[idx]);
                }
            }
            output.push('\n');
        }

        output.push('\n');
        if msg_idx > 0 && msg_idx >= msg_bytes.len() {
            writeln!(output, "DEFRAGMENTATION REVEALED ANOMALOUS DATA.")
                .expect("Write shouldn't fail");
        } else {
            writeln!(output, "DEFRAGMENTATION COMPLETE. 0% FRAGMENTATION.")
                .expect("Write shouldn't fail");
        }

        if matches!(layer, EscalationLayer::Infection) {
            writeln!(output, "MEMORY CORRUPTION DETECTED. CANNOT FIX.")
                .expect("Write shouldn't fail");
        }

        output
    }
}
