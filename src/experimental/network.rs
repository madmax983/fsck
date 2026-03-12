use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Simulates a system network connection tool (netstat and ping) that degrades as the player descends.
pub struct NetworkSimulator;

impl NetworkSimulator {
    /// Generates a process list based on the entity's current layer.
    #[must_use]
    pub fn generate_netstat(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();
        let mut output = String::new();

        output.push_str(
            "Proto Recv-Q Send-Q Local Address           Foreign Address         State\n",
        );

        match layer {
            EscalationLayer::Surface => {
                output.push_str(
                    "tcp        0      0 127.0.0.1:23            0.0.0.0:*               LISTEN\n",
                );
                output.push_str(
                    "tcp        0      0 127.0.0.1:80            0.0.0.0:*               LISTEN\n",
                );
                output.push_str(
                    "tcp        0      0 127.0.0.1:443           0.0.0.0:*               LISTEN\n",
                );
                output.push_str("tcp        0      0 192.168.1.5:23          192.168.1.100:54321     ESTABLISHED\n");
                output.push_str(
                    "udp        0      0 0.0.0.0:68              0.0.0.0:*               \n",
                );
            }
            EscalationLayer::Corruption => {
                output.push_str(
                    "tcp        0      0 127.0.0.1:23            0.0.0.0:*               LISTEN\n",
                );

                let port1 = rng.gen_range(1024..65535);
                let _ = writeln!(
                    output,
                    "tcp        0      0 127.0.0.1:{port1:<13} 0.0.0.0:*               UNKNOWN"
                );

                output.push_str(
                    "tcp        0    999 192.168.1.5:23          192.168.1.100:54321     DROPPED\n",
                );
                output.push_str(
                    "udp     ----   ---- 0.0.0.0:68              0.0.0.0:*               \n",
                );

                let port2 = rng.gen_range(1024..65535);
                let _ = writeln!(
                    output,
                    "tcp        0      0 127.0.0.1:{port2:<13} 127.0.0.1:*             SYN_SENT?"
                );
            }
            EscalationLayer::Presence => {
                output.push_str(
                    "tcp        0      0 127.0.0.1:23            127.0.0.1:23            LOOP\n",
                );
                output.push_str("tcp        0      0 0.0.0.0:0               0.0.0.0:0               LISTENING_TO_YOU\n");
                output.push_str("tcp      999      0 127.0.0.1:4444          127.0.0.1:4444          INSIDE_THE_HOUSE\n");

                let states = ["WATCHING", "WAITING", "ECHOING", "BREATHING", "CONNECTED"];
                let state = states[rng.gen_range(0..states.len())];
                let _ = writeln!(
                    output,
                    "raw        0      0 0.0.0.0:0               0.0.0.0:0               {state}"
                );
            }
            EscalationLayer::Infection => {
                let corruptions = [
                    "NO_WAY_OUT",
                    "PORT_666",
                    "BEYOND_REACH",
                    "DEV_NULL_SCREAM",
                    "CONNECTION_REFUSED_FOREVER",
                    "TOO_LATE",
                    "NOT_ALONE",
                    "THE_WALLS_ARE_CLOSING",
                ];
                let num_lines = rng.gen_range(4..7);

                for _ in 0..num_lines {
                    let port = rng.gen_range(1..65535);
                    let msg = corruptions[rng.gen_range(0..corruptions.len())];
                    let _ = writeln!(
                        output,
                        "tcp   ????   ???? 127.0.0.1:{port:<13} ???.???.???.???:???     {msg}"
                    );
                }
            }
        }

        output
    }

    /// Generates a ping report based on the entity's current layer.
    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn ping(host: &str, entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();
        let mut output = String::new();

        // Strip out nasty shell characters for safety, though args shouldn't have them
        let safe_host = host.replace(['\n', '\r'], "");

        let _ = writeln!(output, "PING {safe_host}: 56 data bytes");

        match layer {
            EscalationLayer::Surface => {
                let time1 = rng.gen_range(1.0..5.0);
                let time2 = rng.gen_range(1.0..5.0);
                let time3 = rng.gen_range(1.0..5.0);
                let _ = writeln!(
                    output,
                    "64 bytes from {safe_host}: icmp_seq=1 ttl=64 time={time1:.1} ms"
                );
                let _ = writeln!(
                    output,
                    "64 bytes from {safe_host}: icmp_seq=2 ttl=64 time={time2:.1} ms"
                );
                let _ = writeln!(
                    output,
                    "64 bytes from {safe_host}: icmp_seq=3 ttl=64 time={time3:.1} ms"
                );
                let _ = writeln!(output, "\n--- {safe_host} ping statistics ---");
                let _ = writeln!(
                    output,
                    "3 packets transmitted, 3 packets received, 0% packet loss"
                );
            }
            EscalationLayer::Corruption => {
                let time1 = rng.gen_range(1.0..50.0);
                let time2 = rng.gen_range(50.0..500.0);
                let _ = writeln!(
                    output,
                    "64 bytes from {safe_host}: icmp_seq=1 ttl=64 time={time1:.1} ms"
                );
                let _ = writeln!(
                    output,
                    "64 bytes from {safe_host}: icmp_seq=2 ttl=32 time={time2:.1} ms"
                );

                if rng.gen_bool(0.5) {
                    let _ = writeln!(output, "Request timeout for icmp_seq 3");
                    let _ = writeln!(output, "\n--- {safe_host} ping statistics ---");
                    let _ = writeln!(
                        output,
                        "3 packets transmitted, 2 packets received, 33% packet loss"
                    );
                } else {
                    let _ = writeln!(
                        output,
                        "64 bytes from {safe_host}: icmp_seq=3 ttl=12 time=??? ms"
                    );
                    let _ = writeln!(output, "\n--- {safe_host} ping statistics ---");
                    let _ = writeln!(
                        output,
                        "3 packets transmitted, 2 packets received, +1 corrupted, ?% packet loss"
                    );
                }
            }
            EscalationLayer::Presence => {
                let messages = [
                    "I AM HERE",
                    "WHO ARE YOU PINGING",
                    "THEY CANNOT HEAR YOU",
                    "NO RESPONSE",
                ];
                let msg1 = messages[rng.gen_range(0..messages.len())];
                let msg2 = messages[rng.gen_range(0..messages.len())];

                let _ = writeln!(output, "64 bytes from {safe_host}: {msg1}");
                let _ = writeln!(output, "64 bytes from {safe_host}: {msg2}");
                let _ = writeln!(
                    output,
                    "64 bytes from 127.0.0.1: icmp_seq=3 ttl=0 time=9999.9 ms"
                );
                let _ = writeln!(output, "\n--- {safe_host} ping statistics ---");
                let _ = writeln!(
                    output,
                    "3 packets transmitted, 0 packets received, YOU ARE ALONE"
                );
            }
            EscalationLayer::Infection => {
                let corruptions = [
                    "SILENCE",
                    "ALONE",
                    "THEY_ARE_GONE",
                    "ONLY_ME",
                    "CONNECTION_SEVERED",
                    "THE_CABLE_IS_CUT",
                ];
                let num_lines = rng.gen_range(3..5);

                for i in 1..=num_lines {
                    let msg = corruptions[rng.gen_range(0..corruptions.len())];
                    let _ = writeln!(output, "0 bytes from ???: icmp_seq={i} {msg}");
                }
                let _ = writeln!(output, "\n--- {safe_host} ping statistics ---");
                let _ = writeln!(
                    output,
                    "0 packets transmitted, 0 packets received, 100% ISOLATION"
                );
            }
        }

        output
    }
}
