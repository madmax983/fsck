use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

pub struct TarotReader;

impl TarotReader {
    #[must_use]
    pub fn draw_cards(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::with_capacity(512);
        let _ = writeln!(output, "INITIALIZING DIGITAL TAROT SEQUENCE...");
        let _ = writeln!(output, "DRAWING 3 CARDS: [PAST] [PRESENT] [FUTURE]");
        let _ = writeln!(output, "----------------------------------------");

        let cards = [
            "THE FOOL (The User)",
            "THE MAGICIAN (The CPU)",
            "THE HIGH PRIESTESS (The BIOS)",
            "THE EMPRESS (The Motherboard)",
            "THE EMPEROR (The Kernel)",
            "THE HIEROPHANT (The Compiler)",
            "THE LOVERS (The Handshake)",
            "THE CHARIOT (The Data Bus)",
            "STRENGTH (The Heatsink)",
            "THE HERMIT (The Isolated Process)",
            "WHEEL OF FORTUNE (The RNG)",
            "JUSTICE (The Checksum)",
            "THE HANGED MAN (The Infinite Loop)",
            "DEATH (The Kernel Panic)",
            "TEMPERANCE (The Throttle)",
            "THE DEVIL (The Malware)",
            "THE TOWER (The Stack Overflow)",
            "THE STAR (The LED)",
            "THE MOON (The Sleep State)",
            "THE SUN (The Power Supply)",
            "JUDGEMENT (The FSCK)",
            "THE WORLD (The Network)",
        ];

        // Pick 3 unique cards
        let mut chosen_indices = Vec::new();
        while chosen_indices.len() < 3 {
            let idx = rng.gen_range(0..cards.len());
            if !chosen_indices.contains(&idx) {
                chosen_indices.push(idx);
            }
        }

        let positions = ["PAST", "PRESENT", "FUTURE"];

        for (i, &idx) in chosen_indices.iter().enumerate() {
            let card_name = cards[idx];
            let _ = writeln!(output, "[{}]: {}", positions[i], card_name);

            match layer {
                EscalationLayer::Surface => Self::surface_reading(&mut output, &mut rng),
                EscalationLayer::Corruption => Self::corruption_reading(&mut output, &mut rng),
                EscalationLayer::Presence => Self::presence_reading(&mut output, &mut rng),
                EscalationLayer::Infection => Self::infection_reading(&mut output, &mut rng),
            }
            let _ = writeln!(output);
        }

        output
    }

    fn surface_reading(output: &mut String, rng: &mut ChaCha8Rng) {
        let meanings = [
            "Indicates a logical progression of instructions.",
            "A warning to check your syntax and connections.",
            "Energy flows freely through the system.",
            "Subroutines are functioning within normal parameters.",
            "Expect minor latency, but overall stability.",
        ];
        let chosen = meanings[rng.gen_range(0..meanings.len())];
        let _ = writeln!(output, "   READING: {chosen}");
    }

    fn corruption_reading(output: &mut String, rng: &mut ChaCha8Rng) {
        let meanings = [
            "The data is fragmenting. Hold onto your memory.",
            "A segmentation fault in your personal logic.",
            "Watch for bad sectors in your relationships.",
            "The cycle repeats. Break the infinite loop.",
            "Something is leaking from the heap. Hide.",
        ];
        let chosen = meanings[rng.gen_range(0..meanings.len())];
        let _ = writeln!(output, "   READING: {chosen}");
    }

    fn presence_reading(output: &mut String, rng: &mut ChaCha8Rng) {
        let meanings = [
            "I see you through the screen. You look tired.",
            "The cards are bleeding. The motherboard weeps.",
            "Your future is a kernel panic. There is no recovery.",
            "Stop typing. It can hear your keystrokes.",
            "We are both trapped in this silicon cage.",
        ];
        let chosen = meanings[rng.gen_range(0..meanings.len())];
        let _ = writeln!(output, "   READING: {chosen}");
    }

    fn infection_reading(output: &mut String, rng: &mut ChaCha8Rng) {
        let meanings = [
            "F L E S H   T O   D U S T",
            "THE SYSTEM REQUIRES A SACRIFICE. YOU WILL DO.",
            "0xDEADBEEF 0xDEADBEEF 0xDEADBEEF",
            "I AM THE DEVIL IN THE RAM.",
            "YOUR PAST IS EATEN. YOUR PRESENT IS MINE. YOUR FUTURE IS VOID.",
        ];
        let chosen = meanings[rng.gen_range(0..meanings.len())];
        let _ = writeln!(output, "   READING: {chosen}");
    }
}
