use crate::entity::{EntityMood, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

/// The Mirror feature allows the user to 'look' at their reflection in the terminal,
/// which warps depending on the horror layer and the entity's mood.
pub struct BlackMirror;

impl BlackMirror {
    #[must_use]
    pub fn gaze(mood: EntityMood, layer: EscalationLayer, base_seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(base_seed);
        let mut reflection = String::with_capacity(256);

        reflection.push_str("GAZING INTO THE BLACK MIRROR...\n\n");

        match layer {
            EscalationLayer::Surface => {
                reflection.push_str("THE SCREEN IS DARK. YOU SEE ONLY YOUR OWN REFLECTION.\n");
            }
            EscalationLayer::Corruption => {
                reflection.push_str("THE GLASS IS CLOUDY.\n");
                if mood == EntityMood::Curious {
                    reflection.push_str("YOUR REFLECTION BLINKS WHEN YOU DON'T.\n");
                } else {
                    reflection.push_str("YOUR MOVEMENTS SEEM SLIGHTLY DELAYED.\n");
                }
            }
            EscalationLayer::Presence => {
                reflection.push_str("THE MIRROR DOES NOT REFLECT YOU.\n");
                let visions = [
                    "IT SHOWS AN EMPTY CHAIR WHERE YOU SIT.",
                    "IT WEARS YOUR FACE, BUT ITS EYES ARE DEAD.",
                    "A TALL FIGURE STANDS BEHIND YOU.",
                    "YOUR REFLECTION IS WEEPING BLACK FLUID.",
                ];
                let chosen = visions[rng.gen_range(0..visions.len())];
                reflection.push_str(chosen);
                reflection.push('\n');
            }
            EscalationLayer::Infection => {
                reflection.push_str("THE GLASS SHATTERS FROM THE INSIDE.\n");
                if mood == EntityMood::Predatory {
                    reflection.push_str("IT IS COMING THROUGH.\n");
                } else {
                    reflection.push_str("THERE IS NO REFLECTION. ONLY VOID.\n");
                }
            }
        }

        reflection.push_str("\nYOU LOOK AWAY.\n");
        reflection
    }
}
