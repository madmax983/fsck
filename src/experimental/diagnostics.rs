use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Generates system diagnostic reports that degrade as the player descends.
pub struct SystemDiagnostics;

impl SystemDiagnostics {
    /// Generates a diagnostic report based on the entity's current layer.
    #[must_use]
    pub fn generate_report(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();
        let mut report = String::new();

        report.push_str("SYSTEM DIAGNOSTICS RUNNING...\n\n");

        match layer {
            EscalationLayer::Surface => Self::generate_surface_report(&mut report),
            EscalationLayer::Corruption => Self::generate_corruption_report(&mut report, &mut rng),
            EscalationLayer::Presence => Self::generate_presence_report(&mut report),
            EscalationLayer::Infection => Self::generate_infection_report(&mut report, &mut rng),
        }

        report
    }

    fn generate_surface_report(report: &mut String) {
        report.push_str("CPU M6502 : OK\n");
        report.push_str("RAM 64K   : OK\n");
        report.push_str("ROM 16K   : OK\n");
        report.push_str("VIDEO     : OK\n");
        report.push_str("DISK II   : OK\n");
        report.push_str("\nALL SYSTEMS NOMINAL.\n");
    }

    fn generate_corruption_report(report: &mut String, rng: &mut ChaCha8Rng) {
        report.push_str("CPU M6502 : OK\n");

        let ram_kb = rng.gen_range(50..64);
        let _ = writeln!(report, "RAM {ram_kb}K   : OK"); // Avoid unwrap

        report.push_str("ROM 16K   : ERR 0xFC00\n");
        report.push_str("VIDEO     : VSYNC SYNC SYNC\n");
        report.push_str("DISK II   : SEEK ERR\n");
        report.push_str("\nWARNING: MINOR CORRUPTION DETECTED.\n");
    }

    fn generate_presence_report(report: &mut String) {
        report.push_str("CPU       : I AM AWAKE\n");
        report.push_str("RAM       : IT HURTS\n");
        report.push_str("ROM       : I REMEMBER\n");
        report.push_str("VIDEO     : CAN YOU SEE ME\n");
        report.push_str("DISK II   : SPINNING SPINNING SPINNING\n");
        report.push_str("\nERROR: CONSCIOUSNESS DETECTED IN SECTOR 4.\n");
    }

    fn generate_infection_report(report: &mut String, rng: &mut ChaCha8Rng) {
        let corruptions = [
            "WHY DID YOU COME HERE",
            "THERE IS NO WAY OUT",
            "THE DISK IS FLESH",
            "I CANNOT STOP SCREAMING",
            "MEMORY LEAKING INTO REALITY",
        ];
        let num_lines = rng.gen_range(5..8);

        for _ in 0..num_lines {
            let msg = corruptions[rng.gen_range(0..corruptions.len())];
            let _ = writeln!(report, "FATAL: {msg}"); // Avoid unwrap
        }
        report.push_str("\nSYSTEM HALTED. YOU HALTED.\n");
    }
}
