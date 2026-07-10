1. **Create SectorMapGenerator feature**
   - Run a bash session to create `src/experimental/map.rs`:
```bash
cat << 'EOF' > src/experimental/map.rs
#[cfg(feature = "nova")]
use crate::entity::{Entity, EscalationLayer};
#[cfg(feature = "nova")]
use rand::prelude::*;
#[cfg(feature = "nova")]
use rand_chacha::ChaCha8Rng;
#[cfg(feature = "nova")]
use std::fmt::Write;

#[cfg(feature = "nova")]
pub struct SectorMapGenerator;

#[cfg(feature = "nova")]
impl SectorMapGenerator {
    #[must_use]
    pub fn generate_map(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::new();
        let _ = writeln!(output, "SCANNING LOCAL SECTOR TOPOLOGY...");

        let size = 15;
        let player_x = rng.gen_range(2..size-2);
        let player_y = rng.gen_range(2..size-2);

        for y in 0..size {
            for x in 0..size {
                if x == player_x && y == player_y {
                    output.push('@'); // Player
                } else {
                    let mut char_to_print = if rng.gen_bool(0.15) { '#' } else { '.' };

                    match layer {
                        EscalationLayer::Surface => {}
                        EscalationLayer::Corruption => {
                            if rng.gen_bool(0.05) { char_to_print = '?'; }
                        }
                        EscalationLayer::Presence => {
                            if (x - player_x).abs() <= 3 && (y - player_y).abs() <= 3 && rng.gen_bool(0.2) {
                                char_to_print = 'O'; // Eyes watching
                            }
                        }
                        EscalationLayer::Infection => {
                            if rng.gen_bool(0.3) {
                                let horrors = ['█', '▓', '▒', '░'];
                                char_to_print = horrors[rng.gen_range(0..horrors.len())];
                            }
                            if (x - player_x).abs() <= 1 && (y - player_y).abs() <= 1 && rng.gen_bool(0.8) {
                                char_to_print = 'V'; // Teeth closing in
                            }
                        }
                    }
                    output.push(char_to_print);
                }
                output.push(' ');
            }
            output.push('\n');
        }

        match layer {
            EscalationLayer::Surface => {
                let _ = writeln!(output, "SECTOR SCAN COMPLETE. NO ANOMALIES FOUND.");
            }
            EscalationLayer::Corruption => {
                let _ = writeln!(output, "SECTOR SCAN COMPLETE. MINOR TOPOLOGICAL INCONSISTENCIES.");
            }
            EscalationLayer::Presence => {
                let _ = writeln!(output, "WARNING: MULTIPLE UNREGISTERED OBSERVERS IN SECTOR.");
            }
            EscalationLayer::Infection => {
                let _ = writeln!(output, "FATAL ERROR: THEY ARE HERE. DO NOT MOVE.");
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
    fn test_surface_map() {
        let entity = Entity::new();
        let output = SectorMapGenerator::generate_map(&entity, 42);
        assert!(output.contains("SCANNING LOCAL SECTOR TOPOLOGY"));
        assert!(output.contains('@'));
        assert!(output.contains('.'));
        assert!(output.contains("NO ANOMALIES FOUND"));
    }

    #[test]
    fn test_infection_map() {
        let mut entity = Entity::new();
        entity.update_depth(30); // Infection
        let output = SectorMapGenerator::generate_map(&entity, 42);
        assert!(output.contains("THEY ARE HERE"));
    }
}
