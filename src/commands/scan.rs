use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

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

        format!(
            "READING {total_sectors} SECTORS\n\
            SECTOR 0000-022F: OK\n\
            VTOC: OK\n\
            CATALOG: OK\n\n"
        )
    }

    fn generate_corruption_scan(rng: &mut ChaCha8Rng, fsck_count: u32) -> String {
        let total_sectors = 560 + rng.gen_range(0..100);
        let bad_sectors = rng.gen_range(1..=3);

        let loop_warning = if fsck_count > 1 {
            "SECTOR 0100-01FF: SCAN LOOP DETECTED\n"
        } else {
            ""
        };

        format!(
            "READING {total_sectors} SECTORS\n\
            SECTOR 0000-00FF: OK\n\
            SECTOR 0100-01FF: {bad_sectors} ERROR(S)\n\
            SECTOR 0200-022F: OK\n\
            {loop_warning}\
            VTOC: MISMATCH\n\n"
        )
    }

    fn generate_presence_scan(rng: &mut ChaCha8Rng) -> String {
        let total_sectors = rng.gen_range(400..700);
        let vtoc_entries = rng.gen_range(1..=1024);

        format!(
            "READING {total_sectors} SECTORS\n\
            SECTOR 0000-00FF: OK\n\
            SECTOR 0100-01FF: ACCESS DENIED\n\
            SECTOR 0200-02FF: CONFLICTING RESULTS\n\
            SECTOR 0300-03FF: SECTOR RESISTS READ\n\
            VTOC: {vtoc_entries} ENTRIES (EXPECTED 256)\n\n"
        )
    }

    fn generate_infection_scan(rng: &mut ChaCha8Rng) -> String {
        let total_sectors = rng.gen_range(0..=99999);

        format!(
            "READING {total_sectors} SECTORS\n\
            SECTOR 0000-????: ?????\n\
            SECTOR ????-????: CANNOT\n\
            VTOC: VTOC: VTOC: VTOC:\n\n"
        )
    }
}
