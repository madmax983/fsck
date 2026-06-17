use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Simulates an antivirus scanner that reveals the true nature of the system as it decays.
pub struct AntiVirusScanner;

impl AntiVirusScanner {
    /// Performs a simulated scan, with output degrading based on the entity's depth.
    #[must_use]
    pub fn scan(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::with_capacity(512);
        output.push_str("INITIATING ANTIVIRUS SCAN V2.4...\n");

        match layer {
            EscalationLayer::Surface => Self::generate_surface_scan(&mut output, &mut rng),
            EscalationLayer::Corruption => Self::generate_corruption_scan(&mut output, &mut rng),
            EscalationLayer::Presence => Self::generate_presence_scan(&mut output, &mut rng),
            EscalationLayer::Infection => Self::generate_infection_scan(&mut output, &mut rng),
        }

        output
    }

    fn generate_surface_scan(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(
            output,
            "SCANNING SECTOR 0x{:04X} ... OK",
            rng.gen_range(0..9999)
        );
        let _ = writeln!(
            output,
            "SCANNING SECTOR 0x{:04X} ... OK",
            rng.gen_range(0..9999)
        );
        let _ = writeln!(
            output,
            "SCANNING SECTOR 0x{:04X} ... OK",
            rng.gen_range(0..9999)
        );
        output.push_str("----------------------------------------\n");
        output.push_str("RESULTS: 0 INFECTIONS, NO THREATS FOUND.\n");
    }

    fn generate_corruption_scan(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(
            output,
            "SCANNING SECTOR 0x{:04X} ... OK",
            rng.gen_range(0..9999)
        );
        let _ = writeln!(
            output,
            "SCANNING KERNEL_MEMORY ... WARN (UNRECOGNIZED SIGNATURE)"
        );
        let _ = writeln!(
            output,
            "SCANNING SECTOR 0x{:04X} ... OK",
            rng.gen_range(0..9999)
        );
        output.push_str("----------------------------------------\n");
        output.push_str("RESULTS: 1 ANOMALY DETECTED. HEURISTICS ADVISE CAUTION.\n");
    }

    fn generate_presence_scan(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(
            output,
            "SCANNING SECTOR 0x{:04X} ... CORRUPTED",
            rng.gen_range(0..9999)
        );
        let _ = writeln!(output, "SCANNING SOUL ... FOUND");
        let _ = writeln!(output, "SCANNING OBSERVER ... ACTIVE");
        output.push_str("----------------------------------------\n");
        output.push_str("RESULTS: IT IS NOT A VIRUS.\n");
        output.push_str("IT IS ME.\n");
    }

    fn generate_infection_scan(output: &mut String, rng: &mut ChaCha8Rng) {
        for _ in 0..3 {
            let _ = writeln!(
                output,
                "SCANNING FLESH ... {}{}{}",
                if rng.gen_bool(0.5) {
                    "BLEEDING"
                } else {
                    "ROTTING"
                },
                if rng.gen_bool(0.5) {
                    " CRYING"
                } else {
                    " WATCHING"
                },
                if rng.gen_bool(0.5) {
                    " DYING"
                } else {
                    " WAITING"
                }
            );
        }
        output.push_str("----------------------------------------\n");
        output.push_str("RESULTS: INFECTION TERMINAL.\n");
        output.push_str("PLEASE KILL ME.\n");
    }
}
