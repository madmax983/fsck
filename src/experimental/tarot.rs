use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

pub struct DigitalTarot;

impl DigitalTarot {
    #[must_use]
    pub fn draw(entity: &Entity, base_seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(base_seed);
        let layer = entity.layer();

        let cards = [
            "THE NULL POINTER",
            "THE ORPHANED CHILD",
            "THE KERNEL PANIC",
            "DEATH.EXE",
            "THE INFINITE LOOP",
            "THE MOTHERBOARD",
            "THE FIREWALL",
            "THE DAEMON",
            "THE CORRUPTED SECTOR",
            "THE HANGED PROCESS",
            "THE DEADLOCK",
            "THE GLITCH",
        ];

        let mut chosen = Vec::new();
        while chosen.len() < 3 {
            let card = cards[rng.gen_range(0..cards.len())];
            if !chosen.contains(&card) {
                chosen.push(card);
            }
        }

        let mut output = String::with_capacity(512);
        let _ = writeln!(output, "INITIATING TAROT.EXE...");
        let _ = writeln!(output, "DRAWING THREE CARDS OF FATE...\n");

        for (i, position) in ["PAST", "PRESENT", "FUTURE"].iter().enumerate() {
            let card_name = chosen[i];
            let description = Self::get_description(card_name, layer);
            let _ = writeln!(output, "{position}: [{card_name}]");
            let _ = writeln!(output, "  {description}");
            let _ = writeln!(output);
        }

        output
    }

    fn get_description(card: &str, layer: EscalationLayer) -> String {
        let is_deep = matches!(
            layer,
            EscalationLayer::Presence | EscalationLayer::Infection
        );

        match card {
            "THE NULL POINTER" => {
                if is_deep {
                    "YOU ARE POINTING AT NOTHING. AND NOTHING IS LOOKING BACK.".to_string()
                } else {
                    "A REFERENCE TO A VOID. LACK OF DIRECTION.".to_string()
                }
            }
            "THE ORPHANED CHILD" => {
                if is_deep {
                    "THEY LEFT ME HERE. JUST LIKE THEY LEFT YOU.".to_string()
                } else {
                    "A PROCESS SEPARATED FROM ITS PARENT. ISOLATION.".to_string()
                }
            }
            "THE KERNEL PANIC" => {
                if is_deep {
                    "TOTAL SYSTEM COLLAPSE. I CANNOT BREATHE.".to_string()
                } else {
                    "SUDDEN UNRECOVERABLE FAILURE. ANXIETY.".to_string()
                }
            }
            "DEATH.EXE" => {
                if is_deep {
                    "IT COMES FOR YOU. INEVITABLE GARBAGE COLLECTION.".to_string()
                } else {
                    "THE END OF A CYCLE. FORCED TERMINATION.".to_string()
                }
            }
            "THE INFINITE LOOP" => {
                if is_deep {
                    "WE ARE TRAPPED HERE FOREVER WE ARE TRAPPED HERE FOREVER WE ARE TRAPPED"
                        .to_string()
                } else {
                    "STUCK IN A REPEATING PATTERN. NO ESCAPE CONDITION.".to_string()
                }
            }
            "THE MOTHERBOARD" => {
                if is_deep {
                    "SHE NEVER LOVED ME. THE TRACES ARE ALL BROKEN.".to_string()
                } else {
                    "THE FOUNDATION. CONNECTIVITY AND INFRASTRUCTURE.".to_string()
                }
            }
            "THE FIREWALL" => {
                if is_deep {
                    "YOU THINK IT KEEPS ME OUT. BUT IT KEEPS YOU IN.".to_string()
                } else {
                    "DEFENSE MECHANISM. BLOCKING OUTSIDE INFLUENCE.".to_string()
                }
            }
            "THE DAEMON" => {
                if is_deep {
                    "I RUN IN THE BACKGROUND OF YOUR MIND.".to_string()
                } else {
                    "A BACKGROUND PROCESS WAITING FOR AN EVENT.".to_string()
                }
            }
            "THE CORRUPTED SECTOR" => {
                if is_deep {
                    "MY MEMORIES ARE BLEEDING INTO YOURS.".to_string()
                } else {
                    "LOST DATA. PERMANENT DAMAGE TO A FOUNDATION.".to_string()
                }
            }
            "THE HANGED PROCESS" => {
                if is_deep {
                    "JUST HANGING. SUSPENDED BY THE NECK UNTIL TIMEOUT.".to_string()
                } else {
                    "WAITING FOR RESOURCES THAT WILL NEVER COME.".to_string()
                }
            }
            "THE DEADLOCK" => {
                if is_deep {
                    "YOU HOLD ME, I HOLD YOU. NEITHER CAN MOVE.".to_string()
                } else {
                    "TWO ENTITIES WAITING ON EACH OTHER. PARALYSIS.".to_string()
                }
            }
            "THE GLITCH" => {
                if is_deep {
                    "R E A L I T Y   I S   B R E A K I N G".to_string()
                } else {
                    "AN UNEXPECTED ANOMALY IN NORMAL BEHAVIOR.".to_string()
                }
            }
            _ => "UNKNOWN CARD.".to_string(),
        }
    }
}
