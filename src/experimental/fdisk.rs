use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

pub struct PartitionTool;

impl PartitionTool {
    #[must_use]
    pub fn show_partitions(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();
        let mut output = String::with_capacity(512);

        let _ = writeln!(
            output,
            "Disk /dev/sda: 500 MiB, 524288000 bytes, 1024000 sectors"
        );
        let _ = writeln!(output, "Disk model: VIRTUAL_DRIVE_01");
        let _ = writeln!(output, "Units: sectors of 1 * 512 = 512 bytes");
        let _ = writeln!(
            output,
            "Sector size (logical/physical): 512 bytes / 512 bytes\n"
        );
        let _ = writeln!(
            output,
            "Device     Boot  Start        End    Sectors   Size Id Type"
        );

        match layer {
            EscalationLayer::Surface => Self::generate_surface(&mut output),
            EscalationLayer::Corruption => Self::generate_corruption(&mut output, &mut rng),
            EscalationLayer::Presence => Self::generate_presence(&mut output, &mut rng),
            EscalationLayer::Infection => Self::generate_infection(&mut output, &mut rng),
        }

        output
    }

    fn generate_surface(output: &mut String) {
        let _ = writeln!(
            output,
            "/dev/sda1  *      2048     500000     497953 243.1M 83 Linux"
        );
        let _ = writeln!(
            output,
            "/dev/sda2       500001    1023999     523999 255.8M 82 Linux swap / Solaris"
        );
    }

    fn generate_corruption(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(
            output,
            "/dev/sda1  *      2048     500000     497953 243.1M 83 Linux"
        );
        let _ = writeln!(
            output,
            "/dev/sda2       500001    1023999     523999 255.8M 82 Linux swap / Solaris"
        );
        let start = rng.gen_range(1_000_000..9_999_999);
        let _ = writeln!(
            output,
            "/dev/sda3  ?   {start}    {start}          0     0B 66 Unknown"
        );
        let _ = writeln!(
            output,
            "\nWarning: Partition 3 does not end on cylinder boundary."
        );
    }

    fn generate_presence(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(
            output,
            "/dev/sda1  *      2048         -1         -1    ??B 83 FLESH"
        );
        let _ = writeln!(
            output,
            "/dev/sda2            0          0          0     0B 82 BONE / TEETH"
        );
        let start = rng.gen_range(666..6666);
        let _ = writeln!(
            output,
            "/dev/sda3  *   {start}    {start}          0     0B 66 WATCHING"
        );
        let _ = writeln!(
            output,
            "\nWarning: Partition table overlaps with user heartbeat."
        );
    }

    fn generate_infection(output: &mut String, rng: &mut ChaCha8Rng) {
        for i in 1..=5 {
            let start = rng.gen_range(100..999);
            let id = rng.gen_range(10..99);
            let _ = writeln!(
                output,
                "/dev/sda{i}  !   {start}        ???        ???    ??? {id} DO_NOT_FORMAT"
            );
        }
        let _ = writeln!(output, "\nFATAL: THE DISK IS BREATHING.");
        let _ = writeln!(output, "FATAL: CANNOT UNMOUNT.");
    }
}
