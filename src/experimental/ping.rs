use crate::entity::{Entity, EscalationLayer};
use rand::prelude::SliceRandom;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Generates a terrifying ping output simulation.
pub struct PingTool;

impl PingTool {
    /// Generates a simulated network ping that degrades with depth.
    ///
    /// # Panics
    ///
    /// Panics if the internal slice used for random selection is empty.
    #[must_use]
    pub fn run_ping(target: &str, entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::new();
        writeln!(output, "PING {target}: 56 data bytes")
            .expect("Writing to String should not fail");

        match layer {
            EscalationLayer::Surface => {
                for i in 1..=4 {
                    let time = rng.gen_range(10..40);
                    writeln!(
                        output,
                        "64 bytes from {target}: icmp_seq={i} ttl=64 time={time} ms"
                    )
                    .expect("Writing to String should not fail");
                }
                writeln!(output, "\n--- {target} ping statistics ---")
                    .expect("Writing to String should not fail");
                writeln!(
                    output,
                    "4 packets transmitted, 4 packets received, 0% packet loss"
                )
                .expect("Writing to String should not fail");
            }
            EscalationLayer::Corruption => {
                for i in 1..=4 {
                    if rng.gen_bool(0.3) {
                        writeln!(output, "Request timeout for icmp_seq {i}")
                            .expect("Writing to String should not fail");
                    } else if rng.gen_bool(0.2) {
                        writeln!(output, "Destination Host Unreachable")
                            .expect("Writing to String should not fail");
                    } else {
                        let time = rng.gen_range(40..300);
                        writeln!(
                            output,
                            "64 bytes from {target}: icmp_seq={i} ttl=64 time={time} ms"
                        )
                        .expect("Writing to String should not fail");
                    }
                }
                writeln!(output, "\n--- {target} ping statistics ---")
                    .expect("Writing to String should not fail");
                let loss = rng.gen_range(25..=75);
                writeln!(
                    output,
                    "4 packets transmitted, {} packets received, {}% packet loss",
                    rng.gen_range(1..=3),
                    loss
                )
                .expect("Writing to String should not fail");
            }
            EscalationLayer::Presence => {
                let msgs = [
                    "I AM THE NETWORK",
                    "THERE IS NO OTHER HOST",
                    "Destination Host Is Watching",
                ];
                for i in 1..=4 {
                    if rng.gen_bool(0.5) {
                        writeln!(output, "{}", *msgs.choose(&mut rng).unwrap())
                            .expect("Writing to String should not fail");
                    } else {
                        writeln!(output, "Request timeout for icmp_seq {i}")
                            .expect("Writing to String should not fail");
                    }
                }
                writeln!(output, "\n--- {target} ping statistics ---")
                    .expect("Writing to String should not fail");
                writeln!(
                    output,
                    "4 packets transmitted, 0 packets received, 100% soul loss"
                )
                .expect("Writing to String should not fail");
            }
            EscalationLayer::Infection => {
                let msgs = [
                    "WHY ARE YOU KNOCKING",
                    "NO ONE IS HOME BUT ME",
                    "PACKETS SWALLOWED BY THE DARK",
                    "BLOOD IN THE SOCKET",
                    "THE CONNECTION IS SEVERED",
                ];
                for _ in 1..=4 {
                    writeln!(output, "{}", *msgs.choose(&mut rng).unwrap())
                        .expect("Writing to String should not fail");
                }
            }
        }

        output
    }
}
