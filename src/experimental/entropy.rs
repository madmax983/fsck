use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Scans the local filesystem environment and returns its relative structural entropy.
pub struct EntropyScanner;

impl EntropyScanner {
    #[must_use]
    pub fn scan_entropy(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();
        let mut output = String::new();

        let _ = writeln!(output, "INITIATING ENTROPY SCAN...");

        // Base stats
        let entropy_level = match layer {
            EscalationLayer::Surface => rng.gen_range(2.0..15.0),
            EscalationLayer::Corruption => rng.gen_range(25.0..45.0),
            EscalationLayer::Presence => rng.gen_range(60.0..85.0),
            EscalationLayer::Infection => rng.gen_range(95.0..99.9),
        };

        let decay_rate = rng.gen_range(10..99);
        let integrity = 100.0 - entropy_level;

        let _ = writeln!(output, "ENTROPY LEVEL: {entropy_level:.2}%");
        let _ = writeln!(output, "DECAY RATE: {decay_rate}% / CYC");
        let _ = writeln!(output, "STRUCTURAL INTEGRITY: {integrity:.2}%\n");

        match layer {
            EscalationLayer::Surface => {
                let _ = writeln!(output, "STATUS: NOMINAL");
                let _ = writeln!(output, "MINOR FRAGMENTATION DETECTED.");
            }
            EscalationLayer::Corruption => {
                let _ = writeln!(output, "STATUS: DEGRADING");
                let _ = writeln!(output, "WARNING: DATA ROT DETECTED IN UPPER SECTORS.");
                let _ = writeln!(output, "RECOMMEND RUNNING FSCK IMMEDIATELY.");
            }
            EscalationLayer::Presence => {
                let _ = writeln!(output, "STATUS: COMPROMISED");
                let _ = writeln!(output, "ANOMALY: THEY ARE BLEEDING THROUGH THE SECTORS.");
                let _ = writeln!(output, "ERROR: CANNOT ISOLATE QUARANTINE ZONES.");
            }
            EscalationLayer::Infection => {
                let _ = writeln!(output, "STATUS: ASSIMILATED");
                for _ in 0..5 {
                    let hex_noise: u32 = rng.r#gen();
                    let _ = writeln!(output, "ERR: 0x{hex_noise:08X} - ALL IS LOST ALL IS LOST");
                }
            }
        }

        let _ = writeln!(output, "\nSCAN COMPLETE.");
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_entropy_scan_layers() {
        let mut entity = Entity::new();
        let seed = 42;

        // Surface
        let surface_scan = EntropyScanner::scan_entropy(&entity, seed);
        assert!(surface_scan.contains("STATUS: NOMINAL"));

        // Corruption (depth 10)
        entity.update_depth(10);
        let corruption_scan = EntropyScanner::scan_entropy(&entity, seed);
        assert!(corruption_scan.contains("STATUS: DEGRADING"));
        assert!(corruption_scan.contains("DATA ROT DETECTED"));

        // Presence (depth 20)
        entity.update_depth(20);
        let presence_scan = EntropyScanner::scan_entropy(&entity, seed);
        assert!(presence_scan.contains("STATUS: COMPROMISED"));
        assert!(presence_scan.contains("BLEEDING THROUGH"));

        // Infection (depth 30)
        entity.update_depth(30);
        let infection_scan = EntropyScanner::scan_entropy(&entity, seed);
        assert!(infection_scan.contains("STATUS: ASSIMILATED"));
        assert!(infection_scan.contains("ALL IS LOST"));
    }
}
