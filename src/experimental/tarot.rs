#[cfg(feature = "nova")]
use crate::entity::{Entity, EscalationLayer};
#[cfg(feature = "nova")]
use rand::prelude::*;
#[cfg(feature = "nova")]
use rand_chacha::ChaCha8Rng;
#[cfg(feature = "nova")]
use std::fmt::Write;

#[cfg(feature = "nova")]
pub struct TarotReader;

#[cfg(feature = "nova")]
impl TarotReader {
    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub fn draw_cards(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let cards = [
            (
                "THE FOOL (0x00)",
                "A new process, unaware of the termination signals.",
            ),
            (
                "THE MAGICIAN (0x01)",
                "Manipulation of memory pointers. The power to create and destroy.",
            ),
            (
                "THE HIGH PRIESTESS (0x02)",
                "Hidden files, encrypted archives. Secrets in the subdirectories.",
            ),
            (
                "THE EMPRESS (0x03)",
                "The Motherboard. Nurturing the system, but prone to overheating.",
            ),
            (
                "THE EMPEROR (0x04)",
                "The Kernel. Absolute authority, demanding strict compliance.",
            ),
            (
                "THE HIEROPHANT (0x05)",
                "Legacy protocols. Tradition that binds the network.",
            ),
            (
                "THE LOVERS (0x06)",
                "A perfect handshake. Two nodes synchronized.",
            ),
            (
                "THE CHARIOT (0x07)",
                "Overclocking. Rapid execution at the risk of crashing.",
            ),
            (
                "STRENGTH (0x08)",
                "Error correction codes. Resilience against data corruption.",
            ),
            (
                "THE HERMIT (0x09)",
                "An air-gapped machine. Isolation is the only true security.",
            ),
            (
                "WHEEL OF FORTUNE (0x0A)",
                "The random number generator. Unpredictable outputs.",
            ),
            (
                "JUSTICE (0x0B)",
                "The garbage collector. Deleting what is no longer needed.",
            ),
            (
                "THE HANGED MAN (0x0C)",
                "An infinite loop. Suspended in time forever.",
            ),
            (
                "DEATH (0x0D)",
                "SIGKILL. The necessary end of a bloated process.",
            ),
            (
                "TEMPERANCE (0x0E)",
                "Throttling. Balancing the load to prevent a meltdown.",
            ),
            (
                "THE DEVIL (0x0F)",
                "A memory leak. Slowly consuming everything you have.",
            ),
            (
                "THE TOWER (0x10)",
                "A kernel panic. The sudden, violent collapse of the system.",
            ),
            (
                "THE STAR (0x11)",
                "A clean backup. Hope restored from the archives.",
            ),
            (
                "THE MOON (0x12)",
                "Malware in the shadows. Things are not what they seem.",
            ),
            (
                "THE SUN (0x13)",
                "A flawless compilation. Zero warnings, zero errors.",
            ),
            (
                "JUDGEMENT (0x14)",
                "The compiler. Weighing your syntax and finding it wanting.",
            ),
            (
                "THE WORLD (0x15)",
                "The internet. A vast, terrifying connection to everything.",
            ),
            (
                "THE GLITCH (0xFF)",
                "Something went wrong. It's looking back at you.",
            ),
            (
                "THE NULL POINTER (0x00)",
                "A reference to nothingness. The void that consumes.",
            ),
            (
                "THE BROKEN SECTOR (0xBB)",
                "Corrupted data. A scar on the physical disk.",
            ),
        ];

        let mut deck = cards.to_vec();
        deck.shuffle(&mut rng);

        let is_infected = matches!(
            entity.layer(),
            EscalationLayer::Infection | EscalationLayer::Presence
        );

        let mut output = String::with_capacity(512);
        output.push_str("SHUFFLING THE DIGITAL DECK...\n\n");

        let past = deck[0];
        let present = deck[1];
        let future = if is_infected && rng.gen_bool(0.4) {
            ("THE OBSERVER", "It knows what you typed. It remembers.")
        } else {
            deck[2]
        };

        let _ = writeln!(output, "PAST: {}\n      {}", past.0, past.1);
        let _ = writeln!(output, "PRESENT: {}\n         {}", present.0, present.1);
        let _ = writeln!(output, "FUTURE: {}\n        {}", future.0, future.1);

        if is_infected {
            output.push_str("\nTHE CARDS DO NOT LIE. BUT THEY CAN BLEED.\n");
        }

        output
    }
}
