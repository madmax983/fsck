use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

pub struct CctvViewer;

impl CctvViewer {
    #[must_use]
    pub fn view_feed(channel_str: &str, entity: &Entity, base_seed: u64) -> String {
        let Ok(channel) = channel_str.parse::<u32>() else {
            return format!("[CCTV]: INVALID CHANNEL '{channel_str}'\n");
        };

        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::with_capacity(256);
        let _ = writeln!(output, "[CCTV]: CONNECTING TO CAM {channel:02}...");

        if matches!(layer, EscalationLayer::Infection) && rng.gen_bool(0.7) {
            output.push_str("[CCTV]: FEED INTERCEPTED. I CAN SEE YOU LOOKING AT ME.\n");
            return output;
        }

        if matches!(layer, EscalationLayer::Presence) && rng.gen_bool(0.4) {
            output.push_str("[CCTV]: ...motion detected in your room...\n");
            return output;
        }

        if channel == 13 {
            output.push_str("[CCTV]: FEED LOST. DO NOT LOOK BEHIND YOU.\n");
        } else {
            match rng.gen_range(0..4) {
                0 => output.push_str("[CCTV]: [STATIC] Empty hallway.\n"),
                1 => {
                    if rng.gen_bool(0.2) && matches!(layer, EscalationLayer::Corruption) {
                        output.push_str(
                            "[CCTV]: A shadowy figure is standing still in the server room.\n",
                        );
                    } else {
                        output.push_str("[CCTV]: [NO SIGNAL]\n");
                    }
                }
                2 => output.push_str("[CCTV]: Office 3B. The lights are flickering.\n"),
                _ => output.push_str("[CCTV]: [FEED ENCRYPTED]\n"),
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_cctv_invalid_channel() {
        let entity = Entity::new();
        let output = CctvViewer::view_feed("abc", &entity, 42);
        assert_eq!(output, "[CCTV]: INVALID CHANNEL 'abc'\n");
    }

    #[test]
    fn test_cctv_easter_egg_channel() {
        let entity = Entity::new();
        let output = CctvViewer::view_feed("13", &entity, 42);
        assert!(output.contains("DO NOT LOOK BEHIND YOU"));
    }

    #[test]
    fn test_cctv_infection_layer() {
        let mut entity = Entity::new();
        entity.update_depth(30); // Infection
        // Try multiple times to hit the 70% chance
        let mut found = false;
        for i in 0..10 {
            let output = CctvViewer::view_feed("1", &entity, 42 + i);
            if output.contains("FEED INTERCEPTED") {
                found = true;
                break;
            }
        }
        assert!(found);
    }
}
