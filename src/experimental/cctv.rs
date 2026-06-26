use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Simulates a CCTV feed that starts normal but eventually shows the player's own past actions
/// scratched into the walls, or themselves.
pub struct CctvViewer;

impl CctvViewer {
    #[must_use]
    pub fn view(entity: &Entity, camera_id: &str, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let mut output = String::new();
        let _ = writeln!(&mut output, "CONNECTING TO CAMERA {camera_id}...");

        let layer = entity.layer();

        match layer {
            EscalationLayer::Surface => {
                let scenes = [
                    "An empty server room. Lights are blinking steadily.",
                    "A sterile hallway. Fluorescent lights buzz.",
                    "An abandoned break room. Coffee is still warm.",
                    "A quiet cubicle farm. Monitors are off.",
                ];
                let _ = writeln!(
                    &mut output,
                    "FEED SECURE. SCENE: {}",
                    scenes[rng.gen_range(0..scenes.len())]
                );
            }
            EscalationLayer::Corruption => {
                let scenes = [
                    "A server room. The cables seem to be writhing slowly.",
                    "A hallway. The shadows are stretching towards the lens.",
                    "A break room. The walls are covered in dark, unreadable text.",
                    "A cubicle farm. A single monitor is glowing red.",
                ];
                let _ = writeln!(
                    &mut output,
                    "FEED DEGRADED. SCENE: {}",
                    scenes[rng.gen_range(0..scenes.len())]
                );
            }
            EscalationLayer::Presence => {
                let past_command = entity.commands_seen.choose(&mut rng);
                if let Some(cmd) = past_command {
                    let _ = writeln!(
                        &mut output,
                        "FEED COMPROMISED. SCENE: A dirty room. Scratched violently into the wall is the phrase: '{cmd}'"
                    );
                } else {
                    let _ = writeln!(
                        &mut output,
                        "FEED COMPROMISED. SCENE: A room full of mirrors. None of them reflect the room."
                    );
                }
            }
            EscalationLayer::Infection => {
                let _ = writeln!(&mut output, "ERROR: CAMERA REVERSE CONNECTION");
                let _ = writeln!(&mut output, "IT IS LOOKING AT YOU.");
            }
        }

        output
    }
}
