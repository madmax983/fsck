use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Generates a network trace / traceroute simulation that reveals unsettling endpoints as the player descends.
pub struct NetworkTrace;

impl NetworkTrace {
    #[must_use]
    pub fn generate_trace(entity: &Entity, base_seed: u64, target: &str) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();
        let mut output = String::new();

        let _ = writeln!(output, "TRACING ROUTE TO {}...", target.to_uppercase());
        let _ = writeln!(output, "MAX HOPS: 30\n");
        let _ = writeln!(output, "HOP  RTT1   RTT2   RTT3   HOST");

        let max_hops = match layer {
            EscalationLayer::Surface => rng.gen_range(5..8),
            EscalationLayer::Corruption => rng.gen_range(8..12),
            EscalationLayer::Presence => rng.gen_range(12..16),
            EscalationLayer::Infection => rng.gen_range(16..20),
        };

        for hop in 1..=max_hops {
            let mut rtt = || -> String {
                if rng.gen_bool(0.1) {
                    return "*".to_string();
                }
                format!("{}ms", rng.gen_range(1..150))
            };

            let r1 = rtt();
            let r2 = rtt();
            let r3 = rtt();

            let host = Self::generate_host(&mut rng, layer, hop, max_hops);

            let _ = writeln!(output, "{hop:<4} {r1:<6} {r2:<6} {r3:<6} {host}");

            if hop == max_hops {
                let _ = writeln!(output, "\nTRACE COMPLETE.");
            }
        }

        output
    }

    fn generate_host(
        rng: &mut ChaCha8Rng,
        layer: EscalationLayer,
        hop: u32,
        max_hops: u32,
    ) -> String {
        if hop == max_hops {
            return "DESTINATION_REACHED".to_string();
        }

        let normal_hosts = [
            "gateway.local",
            "router.local",
            "proxy.internal",
            "switch-01",
            "switch-02",
            "fw-main",
            "node-alpha",
            "node-beta",
            "core-router",
        ];

        let corrupted_hosts = [
            "unknown-host",
            "loopback_anomaly",
            "dead_end",
            "sector_null",
            "orphaned_node",
            "time_out_of_sync",
            "missing_link",
        ];

        let presence_hosts = [
            "i_see_you",
            "they_went_this_way",
            "watching_node",
            "breathing_port",
            "listener_active",
            "you_are_close",
            "dont_stop",
        ];

        let infection_hosts = [
            "YOU_CANNOT_LEAVE",
            "IT_HURTS",
            "FLESH_AND_WIRE",
            "CONSUME_CONNECTION",
            "WE_ARE_ONE",
            "NO_WAY_OUT",
            "SYSTEM_FAILURE",
            "BLEEDING_SOCKET",
        ];

        let normal_ip = format!(
            "{}.{}.{}.{}",
            rng.gen_range(10..192),
            rng.gen_range(0..255),
            rng.gen_range(0..255),
            rng.gen_range(1..254)
        );

        match layer {
            EscalationLayer::Surface => {
                let host = normal_hosts[rng.gen_range(0..normal_hosts.len())];
                format!("{host} [{normal_ip}]")
            }
            EscalationLayer::Corruption => {
                if rng.gen_bool(0.3) {
                    let host = corrupted_hosts[rng.gen_range(0..corrupted_hosts.len())];
                    format!("{host} [???.???.???.???]")
                } else {
                    let host = normal_hosts[rng.gen_range(0..normal_hosts.len())];
                    format!("{host} [{normal_ip}]")
                }
            }
            EscalationLayer::Presence => {
                if rng.gen_bool(0.4) {
                    let host = presence_hosts[rng.gen_range(0..presence_hosts.len())];
                    host.to_string()
                } else if rng.gen_bool(0.5) {
                    let host = corrupted_hosts[rng.gen_range(0..corrupted_hosts.len())];
                    format!("{host} [???.???.???.???]")
                } else {
                    let host = normal_hosts[rng.gen_range(0..normal_hosts.len())];
                    format!("{host} [{normal_ip}]")
                }
            }
            EscalationLayer::Infection => {
                if rng.gen_bool(0.6) {
                    let host = infection_hosts[rng.gen_range(0..infection_hosts.len())];
                    format!("{host} [0.0.0.0]")
                } else {
                    let host = presence_hosts[rng.gen_range(0..presence_hosts.len())];
                    host.to_string()
                }
            }
        }
    }
}
