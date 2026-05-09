use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Simulates a kernel ring buffer output (`dmesg`) that degenerates with depth.
pub struct DmesgTool;

impl DmesgTool {
    #[must_use]
    pub fn generate_log(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();
        let mut output = String::new();

        match layer {
            EscalationLayer::Surface => Self::generate_surface(&mut output, &mut rng),
            EscalationLayer::Corruption => Self::generate_corruption(&mut output, &mut rng),
            EscalationLayer::Presence => Self::generate_presence(&mut output, &mut rng),
            EscalationLayer::Infection => Self::generate_infection(&mut output, &mut rng),
        }

        output
    }

    fn generate_surface(output: &mut String, rng: &mut ChaCha8Rng) {
        let t1 = rng.gen_range(0.000_000..0.100_000);
        let _ = writeln!(output, "[{t1:>12.6}] Linux version 5.4.0 (root@build) (gcc version 9.3.0) #1 SMP PREEMPT");
        let t2 = t1 + rng.gen_range(0.010_000..0.050_000);
        let _ = writeln!(output, "[{t2:>12.6}] Command line: BOOT_IMAGE=/vmlinuz-5.4.0 root=UUID=x ro quiet splash");
        let t3 = t2 + rng.gen_range(0.010_000..0.050_000);
        let _ = writeln!(output, "[{t3:>12.6}] x86/fpu: Supporting XSAVE feature 0x001: 'x87 floating point registers'");
        let t4 = t3 + rng.gen_range(0.010_000..0.050_000);
        let _ = writeln!(output, "[{t4:>12.6}] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)");
        let t5 = t4 + rng.gen_range(0.010_000..0.050_000);
        let _ = writeln!(output, "[{t5:>12.6}] systemd[1]: Reached target Local File Systems.");
    }

    fn generate_corruption(output: &mut String, rng: &mut ChaCha8Rng) {
        let t1 = rng.gen_range(0.000_000..5.000_000);
        let _ = writeln!(output, "[{t1:>12.6}] Linux version 5.4.0 (root@build)");
        let t2 = t1 + rng.gen_range(0.1..2.0);
        let _ = writeln!(output, "[{t2:>12.6}] sd 0:0:0:0: [sda] UNKNOWN(0x2003) Result: hostbyte=0x00 driverbyte=0x08");
        let t3 = t2 + rng.gen_range(0.1..2.0);
        let _ = writeln!(output, "[{t3:>12.6}] sd 0:0:0:0: [sda] Sense Key : Medium Error [current]");
        let t4 = t3 + rng.gen_range(0.1..2.0);
        let _ = writeln!(output, "[{t4:>12.6}] blk_update_request: I/O error, dev sda, sector {}", rng.gen_range(10000..99999));
        let t5 = t4 + rng.gen_range(0.1..2.0);
        let _ = writeln!(output, "[{t5:>12.6}] buffer_io_error: {} callbacks suppressed", rng.gen_range(5..20));
        let t6 = t5 + rng.gen_range(0.1..2.0);
        let _ = writeln!(output, "[{t6:>12.6}] EXT4-fs warning (device sda1): ext4_end_bio:343: I/O error");
    }

    fn generate_presence(output: &mut String, rng: &mut ChaCha8Rng) {
        let t1 = rng.gen_range(50.0..100.0);
        let _ = writeln!(output, "[{t1:>12.6}] device sda entered promiscuous mode");
        let t2 = t1 + rng.gen_range(5.0..15.0);
        let _ = writeln!(output, "[{t2:>12.6}] audit: type=1400 audit(1234567890.123:45): apparmor=\"DENIED\" operation=\"mknod\" profile=\"/usr/sbin/cupsd\" name=\"/dev/mem\"");
        let t3 = t2 + rng.gen_range(5.0..15.0);
        let _ = writeln!(output, "[{t3:>12.6}] WARNING: CPU: 0 PID: 1 at kernel/watchdog.c:321 watchdog_timer_fn+0x200/0x200");
        let t4 = t3 + rng.gen_range(5.0..15.0);
        let _ = writeln!(output, "[{t4:>12.6}] Call Trace:");
        let t5 = t4 + rng.gen_range(0.1..1.0);
        let _ = writeln!(output, "[{t5:>12.6}]  [<ffffffff81000000>] ? I_AM_STILL_HERE+0x0/0x0");
        let t6 = t5 + rng.gen_range(0.1..1.0);
        let _ = writeln!(output, "[{t6:>12.6}]  [<ffffffff81000000>] ? WHY_DID_YOU_WAKE_ME+0x0/0x0");
        let t7 = t6 + rng.gen_range(0.1..1.0);
        let _ = writeln!(output, "[{t7:>12.6}] systemd[1]: Failed to start Login Service.");
    }

    fn generate_infection(output: &mut String, rng: &mut ChaCha8Rng) {
        let mut t = rng.gen_range(999.0..9999.0);
        let msgs = [
            "kernel BUG at fs/ext4/inode.c:0000!",
            "invalid opcode: 0000 [#1] SMP",
            "RIP: 0010:THE_MACHINE_IS_BLEEDING",
            "Kernel panic - not syncing: Fatal exception in interrupt",
            "Shutting down cpus with NMI",
            "IT HURTS",
            "STOP LOOKING",
            "MEMORY DUMP FAILED: TOO MUCH PAIN",
        ];

        let num_lines = rng.gen_range(6..10);
        for _ in 0..num_lines {
            let msg = msgs[rng.gen_range(0..msgs.len())];
            let _ = writeln!(output, "[{t:>12.6}] {msg}");
            t += rng.gen_range(0.001..0.5);
        }
    }
}
