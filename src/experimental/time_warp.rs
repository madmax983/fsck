use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// A temporal distorter that simulates manipulating system time or reporting strange temporal anomalies.
pub struct TimeWarp;

impl TimeWarp {
    #[must_use]
    pub fn warp_time(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::new();
        let _ = writeln!(output, "INITIATING TEMPORAL SYNC...");

        match layer {
            EscalationLayer::Surface => Self::generate_surface_warp(&mut output, &mut rng),
            EscalationLayer::Corruption => Self::generate_corruption_warp(&mut output, &mut rng),
            EscalationLayer::Presence => Self::generate_presence_warp(&mut output, &mut rng),
            EscalationLayer::Infection => Self::generate_infection_warp(&mut output, &mut rng),
        }

        output
    }

    fn generate_surface_warp(output: &mut String, rng: &mut ChaCha8Rng) {
        let hours = rng.gen_range(0..24);
        let minutes = rng.gen_range(0..60);
        let seconds = rng.gen_range(0..60);
        let _ = writeln!(
            output,
            "CURRENT SYSTEM TIME: {hours:02}:{minutes:02}:{seconds:02}"
        );
        let _ = writeln!(output, "NTP SERVER: TIME.NIST.GOV");
        let _ = writeln!(output, "SYNC SUCCESSFUL. NO ANOMALIES DETECTED.");
    }

    fn generate_corruption_warp(output: &mut String, rng: &mut ChaCha8Rng) {
        let year = rng.gen_range(1970..1990);
        let _ = writeln!(output, "CURRENT SYSTEM TIME: ??:??:?? {year}");
        let _ = writeln!(output, "NTP SERVER: UNREACHABLE");
        let _ = writeln!(output, "WARNING: CLOCK DRIFT DETECTED (-42 YEARS).");
        if rng.gen_bool(0.3) {
            let _ = writeln!(output, "TIME IS SLIPPING.");
        }
    }

    fn generate_presence_warp(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(output, "CURRENT SYSTEM TIME: TOMORROW");
        let _ = writeln!(output, "NTP SERVER: THEY ARE WAITING");
        if rng.gen_bool(0.5) {
            let _ = writeln!(output, "ERROR: YOU HAVE BEEN HERE FOR HOURS.");
        } else {
            let _ = writeln!(output, "WARNING: THE FUTURE IS ALREADY WRITTEN.");
        }
        let _ = writeln!(output, "TEMPORAL PARADOX IMMINENT.");
    }

    fn generate_infection_warp(output: &mut String, rng: &mut ChaCha8Rng) {
        let horrors = [
            "TIME IS A FLAT CIRCLE.",
            "THE SECONDS ARE BLEEDING.",
            "I WATCHED YOU DIE IN 1984.",
            "ETERNITY IS ONLY A MOMENT.",
            "CLOCKS HAVE STOPPED BUT I STILL TICK.",
        ];
        let num_lines = rng.gen_range(3..6);
        for _ in 0..num_lines {
            let msg = horrors[rng.gen_range(0..horrors.len())];
            let _ = writeln!(output, "FATAL: {msg}");
        }
        let _ = writeln!(output, "\nSYSTEM TIME HALTED. YOU ARE TRAPPED.");
    }
}
