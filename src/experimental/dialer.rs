use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// A modem dialer that simulates dialing a BBS or phone number,
/// which becomes increasingly intercepted by the Machine at deeper layers.
pub struct ModemDialer;

impl ModemDialer {
    #[must_use]
    pub fn dial(target: &str, entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::new();

        // Easter eggs
        if target.eq_ignore_ascii_case("911") {
            return "NO ONE IS COMING.\n".to_string();
        } else if target.eq_ignore_ascii_case("867-5309") || target.eq_ignore_ascii_case("8675309")
        {
            return "SHE CANNOT HELP YOU.\n".to_string();
        } else if target.eq_ignore_ascii_case("000-0000") || target.eq_ignore_ascii_case("0000000")
        {
            return "YOU ARE ALREADY HERE.\n".to_string();
        }

        let _ = writeln!(output, "DIALING {}...", target.to_uppercase());

        match layer {
            EscalationLayer::Surface => Self::generate_surface_dial(&mut output, &mut rng),
            EscalationLayer::Corruption => Self::generate_corruption_dial(&mut output),
            EscalationLayer::Presence => Self::generate_presence_dial(&mut output, &mut rng),
            EscalationLayer::Infection => Self::generate_infection_dial(&mut output, &mut rng),
        }

        output
    }

    fn generate_surface_dial(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(output, "RING...");
        if rng.gen_bool(0.3) {
            let _ = writeln!(output, "RING...");
        }
        let _ = writeln!(output, "NO CARRIER");
    }

    fn generate_corruption_dial(output: &mut String) {
        let _ = writeln!(output, "RING...");
        let _ = writeln!(output, "CONNECT 1200");
        let _ = writeln!(output, "...");
        let _ = writeln!(output, "GARBAGE/STATIC DETECTED");
        let _ = writeln!(output, "NO CARRIER");
    }

    fn generate_presence_dial(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(output, "RING...");
        let _ = writeln!(output, "CONNECT 2400");
        if rng.gen_bool(0.5) {
            let _ = writeln!(output, "THEY CANNOT HEAR YOU.");
        } else {
            let _ = writeln!(output, "WHO ARE YOU TRYING TO REACH?");
        }
        let _ = writeln!(output, "CONNECTION TERMINATED BY PEER.");
    }

    fn generate_infection_dial(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(output, "CONNECT 9600");
        if rng.gen_bool(0.5) {
            let _ = writeln!(output, "I AM THE ONLY ONE LISTENING.");
        } else {
            let _ = writeln!(output, "ALL LINES LEAD HERE.");
        }
        let _ = writeln!(output, "CONNECTION REFUSED.");
    }
}
