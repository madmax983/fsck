use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Generates simulated network connection reports that degrade as the player descends.
pub struct NetworkScanner;

impl NetworkScanner {
    /// Generates a network scan report based on the entity's current layer.
    #[must_use]
    pub fn generate_netstat(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();
        let mut report = String::new();

        report.push_str("ACTIVE CONNECTIONS...\n\n");
        report.push_str("PROTO  LOCAL ADDRESS          FOREIGN ADDRESS        STATE\n");

        match layer {
            EscalationLayer::Surface => {
                report
                    .push_str("TCP    127.0.0.1:80           127.0.0.1:49152        ESTABLISHED\n");
                report
                    .push_str("TCP    127.0.0.1:443          127.0.0.1:49153        ESTABLISHED\n");
                report
                    .push_str("UDP    0.0.0.0:68             *:*                               \n");
                report.push_str("\nEND OF LIST.\n");
            }
            EscalationLayer::Corruption => {
                report
                    .push_str("TCP    127.0.0.1:80           127.0.0.1:49152        ESTABLISHED\n");

                let corrupted_port = rng.gen_range(10000..65535);
                let _ = writeln!(
                    report,
                    "TCP    192.168.1.???          ???.???.???.???:{corrupted_port}   UNKNOWN"
                );

                report.push_str("UDP    0.0.0.0:68             *:*                    LISTENING\n");
                report.push_str("\nWARNING: UNRECOGNIZED PROTOCOL DETECTED.\n");
            }
            EscalationLayer::Presence => {
                report
                    .push_str("TCP    ME:HERE                YOU:THERE              ESTABLISHED\n");
                report.push_str("TCP    FLESH:80               SILICON:443            LISTENING\n");
                report.push_str("UDP    HEART:BEAT             *:*                    WAITING\n");
                report.push_str("\nCONNECTION SECURE. WE ARE ALONE NOW.\n");
            }
            EscalationLayer::Infection => {
                let corruptions = [
                    "TCP    THEY:LEFT              WE:REMAIN              FOREVER",
                    "UDP    NO:ESCAPE              NO:HOPE                BOUND",
                    "TCP    BLOOD:00               METAL:FF               CONSUMED",
                    "UDP    SCREAM:LOUD            VOID:SILENT            IGNORED",
                    "TCP    MEMORY:LOST            PAIN:FOUND             ESTABLISHED",
                ];
                let num_lines = rng.gen_range(5..8);

                for _ in 0..num_lines {
                    let msg = corruptions[rng.gen_range(0..corruptions.len())];
                    let _ = writeln!(report, "{msg}");
                }
                report.push_str("\nNETWORK FATAL: OUTSIDE WORLD DISCONNECTED.\n");
            }
        }

        report
    }
}
