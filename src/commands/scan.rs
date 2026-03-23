use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

use crate::entity::EscalationLayer;

pub struct FsckScanGenerator;

impl FsckScanGenerator {
    /// Generate sector scan output appropriate to the current layer
    #[must_use]
    pub fn generate_fsck_scan(layer: EscalationLayer, fsck_count: u32, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        let scan_result = match layer {
            EscalationLayer::Surface => Self::generate_surface_scan(),
            EscalationLayer::Corruption => Self::generate_corruption_scan(&mut rng, fsck_count),
            EscalationLayer::Presence => Self::generate_presence_scan(&mut rng),
            EscalationLayer::Infection => Self::generate_infection_scan(&mut rng),
        };

        format!("CHECKING DISK...\n\n{scan_result}")
    }

    fn generate_surface_scan() -> String {
        let total_sectors = 560;

        let mut output = String::with_capacity(128);
        let _ = writeln!(output, "READING {total_sectors} SECTORS");
        let _ = writeln!(output, "SECTOR 0000-022F: OK");
        let _ = writeln!(output, "VTOC: OK");
        let _ = writeln!(output, "CATALOG: OK\n");
        output
    }

    fn generate_corruption_scan(rng: &mut ChaCha8Rng, fsck_count: u32) -> String {
        let total_sectors = 560 + rng.gen_range(0..100);
        let bad_sectors = rng.gen_range(1..=3);

        let mut output = String::with_capacity(256);
        let _ = writeln!(output, "READING {total_sectors} SECTORS");
        let _ = writeln!(output, "SECTOR 0000-00FF: OK");
        let _ = writeln!(output, "SECTOR 0100-01FF: {bad_sectors} ERROR(S)");
        let _ = writeln!(output, "SECTOR 0200-022F: OK");

        if fsck_count > 1 {
            let _ = writeln!(output, "SECTOR 0100-01FF: SCAN LOOP DETECTED");
        }

        let _ = writeln!(output, "VTOC: MISMATCH\n");
        output
    }

    fn generate_presence_scan(rng: &mut ChaCha8Rng) -> String {
        let total_sectors = rng.gen_range(400..700);
        let vtoc_entries = rng.gen_range(1..=1024);

        let mut output = String::with_capacity(256);
        let _ = writeln!(output, "READING {total_sectors} SECTORS");
        let _ = writeln!(output, "SECTOR 0000-00FF: OK");
        let _ = writeln!(output, "SECTOR 0100-01FF: ACCESS DENIED");
        let _ = writeln!(output, "SECTOR 0200-02FF: CONFLICTING RESULTS");
        let _ = writeln!(output, "SECTOR 0300-03FF: SECTOR RESISTS READ");
        let _ = writeln!(output, "VTOC: {vtoc_entries} ENTRIES (EXPECTED 256)\n");
        output
    }

    fn generate_infection_scan(rng: &mut ChaCha8Rng) -> String {
        let total_sectors = rng.gen_range(0..=99999);

        let mut output = String::with_capacity(128);
        let _ = writeln!(output, "READING {total_sectors} SECTORS");
        let _ = writeln!(output, "SECTOR 0000-????: ?????");
        let _ = writeln!(output, "SECTOR ????-????: CANNOT");
        let _ = writeln!(output, "VTOC: VTOC: VTOC: VTOC:\n");
        output
    }
}
