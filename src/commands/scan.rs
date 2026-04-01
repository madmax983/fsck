use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

use crate::entity::EscalationLayer;

pub struct FsckScanGenerator;

impl FsckScanGenerator {
    /// Generate sector scan output appropriate to the current layer
    /// ⚡ Bolt Optimization: Uses a shared String buffer instead of intermediate allocations.
    pub fn generate_fsck_scan(
        layer: EscalationLayer,
        fsck_count: u32,
        seed: u64,
        output: &mut String,
    ) {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        let _ = write!(output, "CHECKING DISK...\n\n");

        match layer {
            EscalationLayer::Surface => Self::generate_surface_scan(output),
            EscalationLayer::Corruption => {
                Self::generate_corruption_scan(&mut rng, fsck_count, output);
            }
            EscalationLayer::Presence => Self::generate_presence_scan(&mut rng, output),
            EscalationLayer::Infection => Self::generate_infection_scan(&mut rng, output),
        }
    }

    fn generate_surface_scan(output: &mut String) {
        let total_sectors = 560;

        let _ = writeln!(output, "READING {total_sectors} SECTORS");
        let _ = writeln!(output, "SECTOR 0000-022F: OK");
        let _ = writeln!(output, "VTOC: OK");
        let _ = writeln!(output, "CATALOG: OK\n");
    }

    fn generate_corruption_scan(rng: &mut ChaCha8Rng, fsck_count: u32, output: &mut String) {
        let total_sectors = 560 + rng.gen_range(0..100);
        let bad_sectors = rng.gen_range(1..=3);

        let _ = writeln!(output, "READING {total_sectors} SECTORS");
        let _ = writeln!(output, "SECTOR 0000-00FF: OK");
        let _ = writeln!(output, "SECTOR 0100-01FF: {bad_sectors} ERROR(S)");
        let _ = writeln!(output, "SECTOR 0200-022F: OK");

        if fsck_count > 1 {
            let _ = writeln!(output, "SECTOR 0100-01FF: SCAN LOOP DETECTED");
        }

        let _ = writeln!(output, "VTOC: MISMATCH\n");
    }

    fn generate_presence_scan(rng: &mut ChaCha8Rng, output: &mut String) {
        let total_sectors = rng.gen_range(400..700);
        let vtoc_entries = rng.gen_range(1..=1024);

        let _ = writeln!(output, "READING {total_sectors} SECTORS");
        let _ = writeln!(output, "SECTOR 0000-00FF: OK");
        let _ = writeln!(output, "SECTOR 0100-01FF: ACCESS DENIED");
        let _ = writeln!(output, "SECTOR 0200-02FF: CONFLICTING RESULTS");
        let _ = writeln!(output, "SECTOR 0300-03FF: SECTOR RESISTS READ");
        let _ = writeln!(output, "VTOC: {vtoc_entries} ENTRIES (EXPECTED 256)\n");
    }

    fn generate_infection_scan(rng: &mut ChaCha8Rng, output: &mut String) {
        let total_sectors = rng.gen_range(0..=99999);

        let _ = writeln!(output, "READING {total_sectors} SECTORS");
        let _ = writeln!(output, "SECTOR 0000-????: ?????");
        let _ = writeln!(output, "SECTOR ????-????: CANNOT");
        let _ = writeln!(output, "VTOC: VTOC: VTOC: VTOC:\n");
    }
}
