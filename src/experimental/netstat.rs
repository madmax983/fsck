use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Generates a network statistics (netstat) simulation showing active connections that degrade.
pub struct NetStatGenerator;

impl NetStatGenerator {
    #[must_use]
    pub fn generate_netstat(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::new();
        let _ = writeln!(output, "ACTIVE INTERNET CONNECTIONS");
        let _ = writeln!(
            output,
            "PROTO  LOCAL ADDRESS          FOREIGN ADDRESS        STATE"
        );

        let num_connections = match layer {
            EscalationLayer::Surface => rng.gen_range(3..6),
            EscalationLayer::Corruption => rng.gen_range(5..9),
            EscalationLayer::Presence => rng.gen_range(7..12),
            EscalationLayer::Infection => rng.gen_range(10..15),
        };

        for _ in 0..num_connections {
            Self::generate_connection(&mut output, &mut rng, layer);
        }

        output
    }

    fn generate_connection(output: &mut String, rng: &mut ChaCha8Rng, layer: EscalationLayer) {
        let normal_states = ["ESTABLISHED", "TIME_WAIT", "LISTEN", "CLOSE_WAIT"];
        let corrupted_states = ["ORPHANED", "TIMEOUT", "SYNC_ERR", "UNREACHABLE"];
        let presence_states = ["WATCHING", "LISTENING", "BREATHING", "WAITING"];
        let infection_states = ["BLEEDING", "CONSUMED", "NO_ESCAPE", "FLESH_BOUND", "DEAD"];

        let proto = if rng.gen_bool(0.8) { "TCP" } else { "UDP" };
        let local_port = rng.gen_range(1024..65535);
        let remote_port = rng.gen_range(80..443);
        let local_ip = format!("192.168.1.{}", rng.gen_range(10..20));

        let state;
        let remote_addr;

        match layer {
            EscalationLayer::Surface => {
                state = normal_states[rng.gen_range(0..normal_states.len())];
                remote_addr = format!(
                    "{}.{}.{}.{}:{}",
                    rng.gen_range(10..192),
                    rng.gen_range(0..255),
                    rng.gen_range(0..255),
                    rng.gen_range(1..254),
                    remote_port
                );
            }
            EscalationLayer::Corruption => {
                if rng.gen_bool(0.4) {
                    state = corrupted_states[rng.gen_range(0..corrupted_states.len())];
                    remote_addr = format!("???.???.???.???:{remote_port}");
                } else {
                    state = normal_states[rng.gen_range(0..normal_states.len())];
                    remote_addr = format!(
                        "{}.{}.{}.{}:{}",
                        rng.gen_range(10..192),
                        rng.gen_range(0..255),
                        rng.gen_range(0..255),
                        rng.gen_range(1..254),
                        remote_port
                    );
                }
            }
            EscalationLayer::Presence => {
                let presence_roll = rng.gen_range(0..100);
                if presence_roll < 40 {
                    state = presence_states[rng.gen_range(0..presence_states.len())];
                    remote_addr = String::from("SOMEWHERE_CLOSE");
                } else if presence_roll < 80 {
                    state = corrupted_states[rng.gen_range(0..corrupted_states.len())];
                    remote_addr = String::from("UNKNOWN_HOST");
                } else {
                    state = normal_states[rng.gen_range(0..normal_states.len())];
                    remote_addr = format!("10.0.0.{}:{remote_port}", rng.gen_range(1..254));
                }
            }
            EscalationLayer::Infection => {
                if rng.gen_bool(0.7) {
                    state = infection_states[rng.gen_range(0..infection_states.len())];
                    remote_addr = String::from("INSIDE_YOUR_WALLS");
                } else {
                    state = presence_states[rng.gen_range(0..presence_states.len())];
                    remote_addr = String::from("RIGHT_BEHIND_YOU");
                }
            }
        }

        let _ = writeln!(
            output,
            "{:<5}  {:<21}  {:<21}  {}",
            proto,
            format!("{}:{}", local_ip, local_port),
            remote_addr,
            state
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surface_netstat() {
        let entity = Entity::new();
        let output = NetStatGenerator::generate_netstat(&entity, 42);
        assert!(output.contains("ACTIVE INTERNET CONNECTIONS"));
        assert!(
            output.contains("ESTABLISHED")
                || output.contains("LISTEN")
                || output.contains("TIME_WAIT")
                || output.contains("CLOSE_WAIT")
        );
        assert!(!output.contains("WATCHING"));
    }

    #[test]
    fn test_infection_netstat() {
        let mut entity = Entity::new();
        entity.update_depth(30); // Infection layer
        let output = NetStatGenerator::generate_netstat(&entity, 42);

        // Output should not contain normal states
        assert!(!output.contains("ESTABLISHED"));
        // Should contain one of the horror states/addresses
        assert!(
            output.contains("BLEEDING")
                || output.contains("CONSUMED")
                || output.contains("INSIDE_YOUR_WALLS")
                || output.contains("RIGHT_BEHIND_YOU")
        );
    }
}
