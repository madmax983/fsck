#[cfg(feature = "nova")]
use crate::entity::{Entity, EscalationLayer};
#[cfg(feature = "nova")]
use rand::{Rng, SeedableRng};
#[cfg(feature = "nova")]
use rand_chacha::ChaCha8Rng;

#[cfg(feature = "nova")]
pub struct CameraDriver;

#[cfg(feature = "nova")]
impl CameraDriver {
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    pub fn capture(entity: &Entity, base_seed: u64) -> String {
        let seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut output = String::from(
            "[ CAPTURING WEBCAM FEED... ]

",
        );

        if entity.layer() == EscalationLayer::Infection {
            output.push_str(
                "... FEED CORRUPTED ...
",
            );
            output.push_str(
                "I M   L O O K I N G   A T   Y O U
",
            );
        } else {
            output.push_str(
                "-----------|
",
            );
            output.push_str(
                "|  .   .  |
",
            );
            output.push_str(
                "|    -    |
",
            );
            output.push_str(
                "-----------|
",
            );
            if rng.gen_bool(0.2) {
                output.push_str(
                    "
(Motion detected in background)
",
                );
            }
        }
        output
    }
}

#[cfg(test)]
#[cfg(feature = "nova")]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_camera_capture() {
        let entity = Entity::new();
        let frame = CameraDriver::capture(&entity, 42);
        assert!(!frame.is_empty());
    }
}
