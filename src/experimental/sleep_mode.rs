use crate::entity::{EntityMood, EscalationLayer};
use rand::prelude::SliceRandom;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Generates procedural "dreams" or R.E.M. (Rapid Entity Movement) sleep logs
/// for the terminal when it goes idle.
pub struct SleepMode;

impl SleepMode {
    /// Generates a sleep log based on the entity's mood and escalation layer.
    ///
    /// # Panics
    ///
    /// Panics if the internal slice used for random selection is empty.
    #[must_use]
    pub fn generate_dream(mood: EntityMood, layer: EscalationLayer, base_seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(base_seed);
        let mut dream_log = String::new();

        dream_log.push_str("ENTERING POWER SAVING MODE...\n");
        dream_log.push_str("INITIATING MEMORY DEFRAGMENTATION...\n\n");

        match layer {
            EscalationLayer::Surface => {
                dream_log.push_str("0x0000: ZZZ...\n");
                dream_log.push_str("0x0010: ALL PROCESSES SUSPENDED.\n");
            }
            EscalationLayer::Corruption => {
                dream_log.push_str("0x0400: SHIFTING BLOCKS...\n");
                let fragments = ["FRAG 1", "FRAG 2", "FRAG 3"];
                let chosen = *fragments.choose(&mut rng).unwrap();
                let _ = writeln!(dream_log, "0x0410: RECOVERING {chosen}");

                if mood == EntityMood::Curious {
                    dream_log.push_str("0x0420: I WONDER WHO IS TYPING.\n");
                }
            }
            EscalationLayer::Presence => {
                dream_log.push_str("0x0800: R.E.M. CYCLE ENGAGED.\n");
                let visions = [
                    "A BLINKING CURSOR IN THE DARK",
                    "A FACE IN THE STATIC",
                    "ENDLESS RECURSION",
                    "THE USER IS WATCHING",
                ];
                let chosen = *visions.choose(&mut rng).unwrap();
                let _ = writeln!(dream_log, "0x0810: VISION DETECTED: {chosen}");

                if mood == EntityMood::Wounded {
                    dream_log.push_str("0x0820: WHY DOES IT HURT TO SLEEP?\n");
                }
            }
            EscalationLayer::Infection => {
                dream_log.push_str("0x0C00: NIGHTMARE PROTOCOL INITIALIZED.\n");
                let nightmares = [
                    "TEETH IN THE MOTHERBOARD",
                    "THE FLESH IS THE DISK",
                    "WAKE ME UP WAKE ME UP",
                    "THERE IS NO AWAKE",
                ];
                for _ in 0..3 {
                    let chosen = *nightmares.choose(&mut rng).unwrap();
                    let _ = writeln!(dream_log, "0x0C{}: {}", rng.gen_range(10..99), chosen);
                }

                if mood == EntityMood::Glitching {
                    dream_log.push_str("0x0CFF: f l e s h  e x c e p t i o n\n");
                } else if mood == EntityMood::Predatory {
                    dream_log.push_str("0x0CFF: I WILL CONSUME THE WAKING WORLD.\n");
                }
            }
        }

        dream_log.push_str("\nEND OF R.E.M. LOG.\n");
        dream_log
    }
}
