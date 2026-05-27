use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// A simulator that attempts to sequence the "DNA" of the system,
/// revealing biological structures at deeper layers.
pub struct GeneSequencer;

impl GeneSequencer {
    #[must_use]
    pub fn sequence(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::with_capacity(512);

        let _ = writeln!(output, "SEQUENCING SYSTEM CORE...");
        let _ = writeln!(output, "---------------------------");

        match layer {
            EscalationLayer::Surface => Self::generate_surface_sequence(&mut output, &mut rng),
            EscalationLayer::Corruption => {
                Self::generate_corruption_sequence(&mut output, &mut rng);
            }
            EscalationLayer::Presence => Self::generate_presence_sequence(&mut output, &mut rng),
            EscalationLayer::Infection => Self::generate_infection_sequence(&mut output, &mut rng),
        }

        let _ = writeln!(output, "---------------------------");
        let _ = writeln!(output, "ANALYSIS COMPLETE.");

        output
    }

    fn generate_surface_sequence(output: &mut String, rng: &mut ChaCha8Rng) {
        for i in 0..5 {
            let _val = rng.gen_range(0..16);
            if rng.gen_bool(0.5) {
                let _ = writeln!(output, "LOC {:04X}: 0110", i * 16);
            } else {
                let _ = writeln!(output, "LOC {:04X}: 1001", i * 16);
            }
        }
    }

    fn generate_corruption_sequence(output: &mut String, rng: &mut ChaCha8Rng) {
        let bases = ['A', 'C', 'T', 'G'];
        for i in 0..5 {
            if rng.gen_bool(0.3) {
                let a = bases[rng.gen_range(0..4)];
                let b = bases[rng.gen_range(0..4)];
                let c = bases[rng.gen_range(0..4)];
                let d = bases[rng.gen_range(0..4)];
                let _ = writeln!(output, "LOC {:04X}: {}-{}-{}-{}", i * 16, a, b, c, d);
            } else {
                let val = rng.gen_range(0..256);
                let _ = writeln!(output, "LOC {:04X}: {:08b}", i * 16, val);
            }
        }
    }

    fn generate_presence_sequence(output: &mut String, rng: &mut ChaCha8Rng) {
        let bases = ['A', 'C', 'T', 'G'];
        for i in 0..6 {
            if rng.gen_bool(0.7) {
                let a = bases[rng.gen_range(0..4)];
                let b = bases[rng.gen_range(0..4)];
                let c = bases[rng.gen_range(0..4)];
                let d = bases[rng.gen_range(0..4)];
                let _ = writeln!(output, "LOC {:04X}: {}-{}-{}-{}", i * 16, a, b, c, d);
            } else {
                let _ = writeln!(output, "LOC {:04X}: U-N-K-N", i * 16);
            }
        }
        let _ = writeln!(output, "WARNING: NON-STANDARD NUCLEOTIDES DETECTED");
    }

    fn generate_infection_sequence(output: &mut String, rng: &mut ChaCha8Rng) {
        let fleshy = [
            "F-L-E-S-H",
            "H-U-M-A-N",
            "B-L-O-O-D",
            "V-E-I-N-S",
            "T-E-E-T-H",
        ];
        for i in 0..6 {
            let val = fleshy[rng.gen_range(0..fleshy.len())];
            let _ = writeln!(output, "LOC {:04X}: {}", i * 16, val);
        }
        let _ = writeln!(output, "ERROR: THE DISK IS ALIVE.");
    }
}
