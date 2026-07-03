use crate::entity::{EntityMood, EscalationLayer};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub struct CctvViewer;

impl CctvViewer {
    #[must_use]
    pub fn view_feed(
        seed: u64,
        camera_id: u32,
        layer: EscalationLayer,
        mood: EntityMood,
    ) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed.wrapping_add(u64::from(camera_id)));

        let header = format!("CAM-{camera_id:02}");

        let status = match layer {
            EscalationLayer::Surface => "Status: Normal",
            EscalationLayer::Corruption => {
                let corruptions = [
                    "Status: NO_SIGNAL",
                    "Status: RECONNECTING...",
                    "Status: INTERFERENCE_DETECTED",
                ];
                corruptions[rng.gen_range(0..corruptions.len())]
            }
            EscalationLayer::Presence => {
                let presences = [
                    "Status: MOTION_DETECTED",
                    "Status: SOMEONE_IS_THERE",
                    "Status: THEY_ARE_WATCHING",
                ];
                presences[rng.gen_range(0..presences.len())]
            }
            EscalationLayer::Infection => {
                let infections = [
                    "Status: ERROR_404_NOT_FOUND",
                    "Status: DATA_CORRUPTED",
                    "Status: I_SEE_YOU",
                ];
                infections[rng.gen_range(0..infections.len())]
            }
        };

        let mood_indicator = match mood {
            EntityMood::Dormant => "",
            EntityMood::Curious => {
                "
[Feed stabilized. A faint shape is visible in the background.]"
            }
            EntityMood::Helpful => {
                "
[The feed is crystal clear. The hallway is empty, but a door is open.]"
            }
            EntityMood::Wounded => {
                "
[Static. A high-pitched whine. The image is distorted by tears in the code.]"
            }
            EntityMood::Predatory => {
                "
[The camera pans on its own. It centers on a dark corner. Something moves.]"
            }
            EntityMood::Glitching => {
                "
[FEED_OFFLINE. SYSTEM_OVERRIDE. I_AM_HERE.]"
            }
        };

        format!("{header}\n{status}{mood_indicator}")
    }
}
