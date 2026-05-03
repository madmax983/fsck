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
            let _ = write!(output, "{hop:<4} ");

            for _ in 0..3 {
                if rng.gen_bool(0.1) {
                    let _ = write!(output, "*      ");
                } else {
                    let ms = rng.gen_range(1..150);
                    // Manually pad to 6 chars
                    if ms < 10 {
                        let _ = write!(output, "{ms}ms    ");
                    } else if ms < 100 {
                        let _ = write!(output, "{ms}ms   ");
                    } else {
                        let _ = write!(output, "{ms}ms  ");
                    }
                }
            }

            Self::generate_host(&mut output, &mut rng, layer, hop, max_hops);
            let _ = writeln!(output);

            if hop == max_hops {
                let _ = writeln!(output, "\nTRACE COMPLETE.");
            }
        }

        output
    }

    fn generate_host(
        output: &mut String,
        rng: &mut ChaCha8Rng,
        layer: EscalationLayer,
        hop: u32,
        max_hops: u32,
    ) {
        if hop == max_hops {
            let _ = write!(output, "DESTINATION_REACHED");
            return;
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

        let ip_0 = rng.gen_range(10..192);
        let ip_1 = rng.gen_range(0..255);
        let ip_2 = rng.gen_range(0..255);
        let ip_3 = rng.gen_range(1..254);
        let ip_str = format!("{ip_0}.{ip_1}.{ip_2}.{ip_3}");

        match layer {
            EscalationLayer::Surface => {
                Self::generate_surface_host(output, rng, &normal_hosts, &ip_str);
            }
            EscalationLayer::Corruption => Self::generate_corruption_host(
                output,
                rng,
                &normal_hosts,
                &corrupted_hosts,
                &ip_str,
            ),
            EscalationLayer::Presence => Self::generate_presence_host(
                output,
                rng,
                &normal_hosts,
                &corrupted_hosts,
                &presence_hosts,
                &ip_str,
            ),
            EscalationLayer::Infection => {
                Self::generate_infection_host(output, rng, &presence_hosts, &infection_hosts);
            }
        }
    }

    fn generate_surface_host(
        output: &mut String,
        rng: &mut ChaCha8Rng,
        normal_hosts: &[&'static str],
        ip_str: &str,
    ) {
        let host = normal_hosts[rng.gen_range(0..normal_hosts.len())];
        let _ = write!(output, "{host} [{ip_str}]");
    }

    fn generate_corruption_host(
        output: &mut String,
        rng: &mut ChaCha8Rng,
        normal_hosts: &[&'static str],
        corrupted_hosts: &[&'static str],
        ip_str: &str,
    ) {
        if rng.gen_bool(0.3) {
            let host = corrupted_hosts[rng.gen_range(0..corrupted_hosts.len())];
            let _ = write!(output, "{host} [???.???.???.???]");
        } else {
            Self::generate_surface_host(output, rng, normal_hosts, ip_str);
        }
    }

    fn generate_presence_host(
        output: &mut String,
        rng: &mut ChaCha8Rng,
        normal_hosts: &[&'static str],
        corrupted_hosts: &[&'static str],
        presence_hosts: &[&'static str],
        ip_str: &str,
    ) {
        if rng.gen_bool(0.4) {
            let host = presence_hosts[rng.gen_range(0..presence_hosts.len())];
            let _ = write!(output, "{host}");
        } else if rng.gen_bool(0.5) {
            let host = corrupted_hosts[rng.gen_range(0..corrupted_hosts.len())];
            let _ = write!(output, "{host} [???.???.???.???]");
        } else {
            Self::generate_surface_host(output, rng, normal_hosts, ip_str);
        }
    }

    fn generate_infection_host(
        output: &mut String,
        rng: &mut ChaCha8Rng,
        presence_hosts: &[&'static str],
        infection_hosts: &[&'static str],
    ) {
        if rng.gen_bool(0.6) {
            let host = infection_hosts[rng.gen_range(0..infection_hosts.len())];
            let _ = write!(output, "{host} [0.0.0.0]");
        } else {
            let host = presence_hosts[rng.gen_range(0..presence_hosts.len())];
            let _ = write!(output, "{host}");
        }
    }
}
