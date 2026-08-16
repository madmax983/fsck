use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

pub struct MemoryLeak;

impl MemoryLeak {
    #[must_use]
    pub fn generate_leak(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();
        let mut output = String::new();

        let _ = writeln!(output, "INITIATING MEMORY LEAK DIAGNOSTIC...");

        match layer {
            EscalationLayer::Surface => Self::surface_leak(&mut output, &mut rng),
            EscalationLayer::Corruption => Self::corruption_leak(&mut output, &mut rng),
            EscalationLayer::Presence => Self::presence_leak(&mut output, &mut rng),
            EscalationLayer::Infection => Self::infection_leak(&mut output, &mut rng),
        }

        output
    }

    fn surface_leak(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(
            output,
            "0x{addr:08X}: NOMINAL DATA FRAGMENT",
            addr = rng.gen_range(0x1000..0x9000)
        );
    }

    fn corruption_leak(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(
            output,
            "0x{addr:08X}: UNKNOWN VARIABLE: who_is_this",
            addr = rng.gen_range(0x1000..0x9000)
        );
    }

    fn presence_leak(output: &mut String, rng: &mut ChaCha8Rng) {
        let options = ["I AM WATCHING", "YOU TYPED THAT", "WHERE ARE YOU"];
        let val = options[rng.gen_range(0..options.len())];
        let _ = writeln!(
            output,
            "0x{addr:08X}: {val}",
            addr = rng.gen_range(0x1000..0x9000)
        );
    }

    fn infection_leak(output: &mut String, rng: &mut ChaCha8Rng) {
        let options = ["FLESH", "BONE", "BLOOD", "TEETH"];
        let val = options[rng.gen_range(0..options.len())];
        let _ = writeln!(
            output,
            "0x{addr:08X}: SYSTEM LEAKING {val}",
            addr = rng.gen_range(0x1000..0x9000)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_memory_leak() {
        let entity = Entity::new();
        let result = MemoryLeak::generate_leak(&entity, 42);
        assert!(result.contains("NOMINAL DATA"));
    }
}
