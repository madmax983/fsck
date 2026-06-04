use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Simulates a GPS location lock that degrades into horror
pub struct GpsTracker;

impl GpsTracker {
    #[must_use]
    pub fn get_location(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();
        let mut output = String::new();

        let _ = writeln!(output, "ACQUIRING SATELLITE LOCK...");

        match layer {
            EscalationLayer::Surface => Self::generate_surface_location(&mut output, &mut rng),
            EscalationLayer::Corruption => {
                Self::generate_corruption_location(&mut output, &mut rng)
            }
            EscalationLayer::Presence => Self::generate_presence_location(&mut output, &mut rng),
            EscalationLayer::Infection => Self::generate_infection_location(&mut output, &mut rng),
        }

        output
    }

    fn generate_surface_location(output: &mut String, rng: &mut ChaCha8Rng) {
        let lat = rng.gen_range(30.0..50.0);
        let lon = rng.gen_range(-120.0..-70.0);
        let _ = writeln!(output, "LOCK ACQUIRED. 4 SATELLITES.");
        let _ = writeln!(output, "LATITUDE : {lat:.4} N");
        let _ = writeln!(output, "LONGITUDE: {lon:.4} W");
        let _ = writeln!(output, "ELEVATION: {}m", rng.gen_range(10..500));
    }

    fn generate_corruption_location(output: &mut String, rng: &mut ChaCha8Rng) {
        let lat = rng.gen_range(10.0..80.0);
        let lon = rng.gen_range(-180.0..180.0);
        let _ = writeln!(output, "LOCK ACQUIRED. 1 SATELLITE.");
        let _ = writeln!(output, "LATITUDE : {lat:.4} ?");
        let _ = writeln!(output, "LONGITUDE: {lon:.4} ?");
        let _ = writeln!(output, "ELEVATION: {}m (SINKING)", rng.gen_range(-1000..0));
    }

    fn generate_presence_location(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(output, "NO SATELLITES FOUND.");
        if rng.gen_bool(0.5) {
            let _ = writeln!(output, "LOCATION : RIGHT BEHIND YOU");
        } else {
            let _ = writeln!(output, "LOCATION : GETTING CLOSER");
        }
        let _ = writeln!(output, "DISTANCE : 0.0m");
    }

    fn generate_infection_location(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(output, "SIGNAL LOST.");
        let messages = [
            "LOCATION : INSIDE",
            "LOCATION : YOU ARE ALREADY BURIED",
            "LOCATION : NOWHERE",
            "LOCATION : THE FLESH",
        ];
        let chosen = messages[rng.gen_range(0..messages.len())];
        let _ = writeln!(output, "{chosen}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_surface_location() {
        let entity = Entity::new();
        let output = GpsTracker::get_location(&entity, 42);
        assert!(output.contains("LATITUDE"));
        assert!(output.contains("LONGITUDE"));
    }

    #[test]
    fn test_infection_location() {
        let mut entity = Entity::new();
        entity.update_depth(30); // Infection layer
        let output = GpsTracker::get_location(&entity, 42);
        assert!(output.contains("LOCATION"));
        assert!(
            output.contains("INSIDE")
                || output.contains("BURIED")
                || output.contains("NOWHERE")
                || output.contains("FLESH")
        );
    }
}
