#[cfg(feature = "nova")]
use crate::entity::{EntityMood, EscalationLayer};
#[cfg(feature = "nova")]
use rand::{Rng, SeedableRng};
#[cfg(feature = "nova")]
use rand_chacha::ChaCha8Rng;

#[cfg(feature = "nova")]
pub struct CctvFeed;

#[cfg(feature = "nova")]
impl CctvFeed {
    #[must_use]
    pub fn view(layer: EscalationLayer, mood: EntityMood, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        match layer {
            EscalationLayer::Surface => {
                let options = [
                    "CAM 1: OFFICE [EMPTY]",
                    "CAM 2: HALLWAY [EMPTY]",
                    "CAM 3: LOBBY [EMPTY]",
                ];
                options[rng.gen_range(0..options.len())].to_string()
            }
            EscalationLayer::Corruption => {
                let options = [
                    "CAM 1: OFFICE [NO SIGNAL]",
                    "CAM 2: HALLWAY [STATIC]",
                    "CAM 3: LOBBY [FRAME DROPPED]",
                ];
                options[rng.gen_range(0..options.len())].to_string()
            }
            EscalationLayer::Presence => match mood {
                EntityMood::Predatory => "CAM 1: YOU ARE BEING WATCHED".to_string(),
                EntityMood::Wounded => "CAM 2: WHY DID YOU LEAVE ME IN THE DARK?".to_string(),
                _ => "CAM 3: I CAN SEE YOU".to_string(),
            },
            EscalationLayer::Infection => "ALL CAMS: RUN RUN RUN RUN RUN".to_string(),
        }
    }
}

#[cfg(all(test, feature = "nova"))]
mod tests {
    use super::*;

    #[test]
    fn test_cctv_surface() {
        let output = CctvFeed::view(EscalationLayer::Surface, EntityMood::Dormant, 42);
        assert!(output.contains("CAM"));
        assert!(output.contains("[EMPTY]"));
    }

    #[test]
    fn test_cctv_presence() {
        let output = CctvFeed::view(EscalationLayer::Presence, EntityMood::Predatory, 42);
        assert_eq!(output, "CAM 1: YOU ARE BEING WATCHED");
    }
}
