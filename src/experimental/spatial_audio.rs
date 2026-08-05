use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

/// Generates textual spatial audio anomalies based on player position and entity state.
pub struct SpatialAudioGenerator;

impl SpatialAudioGenerator {
    /// Generates a subtle audio description, increasing in severity with depth.
    #[must_use]
    pub fn generate_anomaly(entity: &Entity, base_seed: u64) -> Option<String> {
        // Use the interaction count combined with the seed to ensure determinism
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();

        // Probability of anomaly increases with depth
        let probability = match layer {
            EscalationLayer::Surface => 0.05,
            EscalationLayer::Corruption => 0.15,
            EscalationLayer::Presence => 0.35,
            EscalationLayer::Infection => 0.60,
        };

        if !rng.gen_bool(probability) {
            return None;
        }

        let hint = match layer {
            EscalationLayer::Surface => Self::generate_surface_anomaly(&mut rng),
            EscalationLayer::Corruption => Self::generate_corruption_anomaly(&mut rng),
            EscalationLayer::Presence => Self::generate_presence_anomaly(&mut rng),
            EscalationLayer::Infection => Self::generate_infection_anomaly(&mut rng),
        };

        Some(format!("*[AUDIO ANOMALY: {hint}]*"))
    }

    fn generate_surface_anomaly(rng: &mut ChaCha8Rng) -> &'static str {
        let hints = [
            "A faint whirring sound comes from the disk drive...",
            "The fan spins up momentarily, then quiets.",
            "You hear a soft click from inside the machine.",
        ];
        hints[rng.gen_range(0..hints.len())]
    }

    fn generate_corruption_anomaly(rng: &mut ChaCha8Rng) -> &'static str {
        let hints = [
            "A rhythmic grinding noise echoes from an adjacent sector.",
            "The hum of the monitor deepens for a second.",
            "You hear something like static, but just out of earshot.",
            "The disk drive head seeks violently, then stops.",
        ];
        hints[rng.gen_range(0..hints.len())]
    }

    fn generate_presence_anomaly(rng: &mut ChaCha8Rng) -> &'static str {
        let hints = [
            "A low, resonant hum reverberates through the filesystem.",
            "It sounds like breathing, but metallic and slow.",
            "You hear the echo of keys clicking, but your hands are still.",
            "A sharp, high-pitched whine cuts through the silence.",
            "The machine vibrates, like a purr or a growl.",
        ];
        hints[rng.gen_range(0..hints.len())]
    }

    fn generate_infection_anomaly(rng: &mut ChaCha8Rng) -> &'static str {
        let hints = [
            "THE SOUND IS DEAFENING. IT COMES FROM EVERYWHERE.",
            "A screech of tearing metal and corrupted data.",
            "Voices in the static. They are calling your name.",
            "The grinding noise is inside your head now.",
            "SILENCE. ABSOLUTE, CRUSHING SILENCE.",
        ];
        hints[rng.gen_range(0..hints.len())]
    }
}
