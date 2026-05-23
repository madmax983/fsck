use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

pub struct DmesgGenerator;

impl DmesgGenerator {
    #[must_use]
    pub fn generate_dmesg(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();
        let mut output = String::with_capacity(1024);
        let num_lines = match layer {
            EscalationLayer::Surface => rng.gen_range(10..20),
            EscalationLayer::Corruption => rng.gen_range(15..25),
            EscalationLayer::Presence => rng.gen_range(20..30),
            EscalationLayer::Infection => rng.gen_range(25..40),
        };
        let mut uptime = rng.gen_range(0.0..5.0);
        for _ in 0..num_lines {
            uptime += rng.gen_range(0.01..1.5);
            let _ = write!(output, "[{uptime:>8.4}] ");
            Self::generate_log_line(&mut output, &mut rng, layer);
            output.push('\n');
        }
        output
    }

    fn generate_log_line(output: &mut String, rng: &mut ChaCha8Rng, layer: EscalationLayer) {
        let surface_logs = [
            "kernel: Booting Node...",
            "kernel: CPU0 microcode updated",
            "ACPI: Core revision 20240101",
            "usb 1-1: new high-speed USB device",
            "sd 2:0:0:0: [sda] Attached SCSI disk",
            "EXT4-fs (sda1): mounted filesystem with ordered data mode",
        ];
        let corruption_logs = [
            "kernel: memory block 0x00A00000 corrupted",
            "ACPI: parse error in thermal zone",
            "usb 1-1: device descriptor read/64, error -110",
            "sd 2:0:0:0: [sda] Unhandled sense code",
            "EXT4-fs (sda1): warning: mounting unchecked fs",
        ];
        let presence_logs = [
            "kernel: WARNING: unknown entity in process table",
            "audit: user '???' elevated to root",
            "net_ratelimit: 400 callbacks suppressed",
            "kernel: BUG: soft lockup - CPU#0 stuck for 23s!",
            "kernel: I/O error, dev sda, sector 666",
        ];
        let infection_logs = [
            "FATAL: THEY ARE HERE",
            "kernel: kernel panic - not syncing: VITAL SIGNS FAILING",
            "ACPI: Critical trip point exceeded",
            "systemd[1]: Caught <SEGV>, dumped core as pid 1",
            "IT HURTS IT HURTS IT HURTS",
        ];

        let selected_log = match layer {
            EscalationLayer::Surface => surface_logs.choose(rng).unwrap(),
            EscalationLayer::Corruption => {
                if rng.gen_bool(0.3) {
                    corruption_logs.choose(rng).unwrap()
                } else {
                    surface_logs.choose(rng).unwrap()
                }
            }
            EscalationLayer::Presence => {
                let p = rng.r#gen::<f64>();
                if p < 0.5 {
                    presence_logs.choose(rng).unwrap()
                } else if p < 0.75 {
                    corruption_logs.choose(rng).unwrap()
                } else {
                    surface_logs.choose(rng).unwrap()
                }
            }
            EscalationLayer::Infection => {
                if rng.gen_bool(0.7) {
                    infection_logs.choose(rng).unwrap()
                } else {
                    presence_logs.choose(rng).unwrap()
                }
            }
        };
        output.push_str(selected_log);
    }
}
