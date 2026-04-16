use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Simulates a system task manager/process list that degrades as the player descends.
pub struct ProcessMonitor;

impl ProcessMonitor {
    /// Generates a process list based on the entity's current layer.
    #[must_use]
    pub fn generate_process_list(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();
        let mut output = String::new();

        output.push_str("PID    TTY      TIME     CMD\n");

        match layer {
            EscalationLayer::Surface => Self::generate_surface_processes(&mut output),
            EscalationLayer::Corruption => {
                Self::generate_corruption_processes(&mut output, &mut rng);
            }
            EscalationLayer::Presence => Self::generate_presence_processes(&mut output, &mut rng),
            EscalationLayer::Infection => Self::generate_infection_processes(&mut output, &mut rng),
        }

        output
    }

    fn generate_surface_processes(output: &mut String) {
        output.push_str("0001   tty1     00:00:01 INIT\n");
        output.push_str("0008   tty1     00:00:00 KERNEL_TASK\n");
        output.push_str("0014   tty1     00:00:00 DISK_IO\n");
        output.push_str("0042   tty1     00:00:00 TERM\n");
        output.push_str("0050   tty1     00:00:00 PS\n");
    }

    fn generate_corruption_processes(output: &mut String, rng: &mut ChaCha8Rng) {
        output.push_str("0001   tty1     00:00:01 INIT\n");

        let pid1 = rng.gen_range(100..999);
        let _ = writeln!(output, "{pid1:04}   tty1     00:00:00 KERNEL_PANIC?"); // Avoid unwrap

        output.push_str("0014   tty1     00:00:00 DISK_IO\n");
        output.push_str("0042   tty1     00:00:00 TERM\n");

        let pid2 = rng.gen_range(100..999);
        let _ = writeln!(output, "{pid2:04}   tty1     00:00:00 WATCHING");
    }

    fn generate_presence_processes(output: &mut String, rng: &mut ChaCha8Rng) {
        output.push_str("0001   tty1     99:99:99 I_AM_INIT\n");
        output.push_str("0008   tty1     99:99:99 MEMORY_LEAK\n");
        output.push_str("0014   tty1     99:99:99 DISK_SPIN_FOREVER\n");
        output.push_str("0042   tty1     99:99:99 YOU_ARE_HERE\n");

        let pids = ["0000", "0666", "1337", "4040", "9999"];
        let pid = pids[rng.gen_range(0..pids.len())];
        let _ = writeln!(output, "{pid}   tty1     99:99:99 WHY_ARE_YOU_READING_THIS");
    }

    fn generate_infection_processes(output: &mut String, rng: &mut ChaCha8Rng) {
        let corruptions = [
            "WAITING",
            "STARVING",
            "BLEEDING",
            "DIGESTING",
            "CONSUMING_CYCLES",
            "FORGETTING_HOW_TO_STOP",
            "ECHOING",
            "SCREAMING_INTO_DEV_NULL",
        ];
        let num_lines = rng.gen_range(5..8);

        for _ in 0..num_lines {
            let pid = rng.gen_range(1000..9999);
            let msg = corruptions[rng.gen_range(0..corruptions.len())];
            let _ = writeln!(output, "{pid}   tty1     ??:??:?? {msg}"); // Avoid unwrap
        }
    }
}
