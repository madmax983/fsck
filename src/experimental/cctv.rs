use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// A CCTV simulator that allows viewing simulated camera feeds.
pub struct CctvNetwork;

impl CctvNetwork {
    /// Connects to a given camera feed and returns the text description.
    ///
    /// # Arguments
    /// * `cam_str` - The camera number as a string (e.g. "1")
    /// * `entity` - The game entity, used for layer and interaction count
    /// * `base_seed` - Base seed for deterministic generation
    #[must_use]
    pub fn view(cam_str: &str, entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::with_capacity(256);

        if cam_str.is_empty() {
            output.push_str("AVAILABLE CAMERAS:\n");
            output.push_str("  CAM 1: FRONT ENTRANCE\n");
            output.push_str("  CAM 2: SERVER ROOM\n");
            output.push_str("  CAM 3: BREAK ROOM\n");
            output.push_str("  CAM 4: HALLWAY B\n");
            return output;
        }

        let Ok(cam) = cam_str.parse::<u32>() else {
            return format!("[CCTV]: INVALID CAMERA '{cam_str}'\n");
        };

        if !(1..=4).contains(&cam) {
            return format!("[CCTV]: NO SIGNAL FROM CAM {cam}\n");
        }

        let _ = writeln!(output, "[CCTV]: CONNECTING TO CAM {cam}...");

        match layer {
            EscalationLayer::Surface => match cam {
                1 => output.push_str("VIEW: RAIN FALLING OUTSIDE. STREET IS EMPTY.\n"),
                2 => output.push_str("VIEW: ROWS OF SERVERS. BLINKING LIGHTS.\n"),
                3 => output.push_str("VIEW: EMPTY CHAIRS. COFFEE MACHINE IS OFF.\n"),
                4 => output.push_str("VIEW: LONG FLUORESCENT-LIT HALLWAY. NO MOVEMENT.\n"),
                _ => unreachable!(),
            },
            EscalationLayer::Corruption => match cam {
                1 => output.push_str("VIEW: HEAVY STATIC... SHADOW NEAR THE DOOR?\n"),
                2 => output.push_str("VIEW: SERVER LIGHTS ARE BLINKING IN UNISON.\n"),
                3 => output.push_str("VIEW: CHAIR IS KNOCKED OVER. COFFEE SPILLED.\n"),
                4 => {
                    if rng.gen_bool(0.5) {
                        output.push_str("VIEW: SOMEONE IS STANDING AT THE FAR END.\n");
                    } else {
                        output.push_str("VIEW: HALLWAY LIGHTS FLICKERING.\n");
                    }
                }
                _ => unreachable!(),
            },
            EscalationLayer::Presence => match cam {
                1 => output.push_str("VIEW: IT IS LOOKING UP AT THE CAMERA.\n"),
                2 => output.push_str("VIEW: ALL SERVER RACKS ARE OPEN.\n"),
                3 => output.push_str("VIEW: [FEED INTERRUPTED] ...BREATHING...\n"),
                4 => output.push_str("VIEW: THE FIGURE IS MUCH CLOSER NOW.\n"),
                _ => unreachable!(),
            },
            EscalationLayer::Infection => {
                if rng.gen_bool(0.7) {
                    output.push_str("VIEW: YOU ARE SITTING AT A TERMINAL. YOU ARE TYPING.\n");
                } else {
                    output.push_str("VIEW: [FEED LOST] THE WALLS ARE LISTENING\n");
                }
            }
        }

        output
    }
}
