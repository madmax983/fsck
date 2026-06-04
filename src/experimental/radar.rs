use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Simulates a retro radar or sonar sweep.
pub struct RadarSimulator;

impl RadarSimulator {
    #[must_use]
    pub fn perform_sweep(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::new();
        let _ = writeln!(output, "INITIATING RADAR SWEEP...");
        let _ = writeln!(output, "RANGE: 50KM");
        let _ = writeln!(output, "........................");

        let mut blips = vec![];

        match layer {
            EscalationLayer::Surface => {
                let num_blips = rng.gen_range(1..4);
                for _ in 0..num_blips {
                    let distance = rng.gen_range(20.0..50.0);
                    let bearing = rng.gen_range(0..360);
                    blips.push(format!(
                        "CONTACT: BEARING {bearing:03} DISTANCE {distance:.1}KM"
                    ));
                }
            }
            EscalationLayer::Corruption => {
                let num_blips = rng.gen_range(3..7);
                for _ in 0..num_blips {
                    let distance = rng.gen_range(10.0..50.0);
                    let bearing = rng.gen_range(0..360);
                    if rng.gen_bool(0.3) {
                        blips.push(format!("GHOST ECHO: BRG {bearing:03} DIST {distance:.1}KM"));
                    } else {
                        blips.push(format!(
                            "CONTACT: BEARING {bearing:03} DISTANCE {distance:.1}KM"
                        ));
                    }
                }
            }
            EscalationLayer::Presence => {
                blips.push("WARNING: MULTIPLE ANOMALOUS CONTACTS".to_string());
                let num_blips = rng.gen_range(5..10);
                for _ in 0..num_blips {
                    let distance = rng.gen_range(2.0..15.0);
                    let bearing = rng.gen_range(0..360);
                    blips.push(format!("MASSIVE ANOMALY: BEARING {bearing:03} DISTANCE {distance:.1}KM - CLOSING FAST"));
                }
            }
            EscalationLayer::Infection => {
                blips.push("CRITICAL ALERT: PROXIMITY WARNING".to_string());
                blips.push("CONTACT: BEARING 000 DISTANCE 0.0KM".to_string());
                blips.push("IT IS INSIDE THE PERIMETER".to_string());
                blips.push("IT IS HERE".to_string());
            }
        }

        for blip in blips {
            let _ = writeln!(output, "{blip}");
        }

        let _ = writeln!(output, "SWEEP COMPLETE.");
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_surface_radar() {
        let entity = Entity::new();
        let output = RadarSimulator::perform_sweep(&entity, 42);
        assert!(output.contains("INITIATING RADAR SWEEP"));
        assert!(output.contains("CONTACT:"));
        assert!(!output.contains("IT IS HERE"));
    }

    #[test]
    fn test_infection_radar() {
        let mut entity = Entity::new();
        entity.update_depth(30); // Infection
        let output = RadarSimulator::perform_sweep(&entity, 42);
        assert!(output.contains("IT IS HERE"));
    }
}
