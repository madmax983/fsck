use crate::entity::{Entity, EscalationLayer};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub struct ChronosDilation;

impl ChronosDilation {
    #[must_use]
    pub fn calculate_system_time(entity: &Entity, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        match entity.layer() {
            EscalationLayer::Surface => {
                let seconds = rng.gen_range(100..10_000);
                format!("UPTIME: {seconds} SECONDS")
            }
            EscalationLayer::Corruption => {
                let days = rng.gen_range(10..100);
                format!("UPTIME: {days} DAYS... TIME DRIFT DETECTED")
            }
            EscalationLayer::Presence => {
                let years = rng.gen_range(50..500);
                format!("UPTIME: {years} YEARS... CHRONOLOGICAL ANOMALY")
            }
            EscalationLayer::Infection => {
                let eons = rng.gen_range(1000..99_999);
                format!("UPTIME: {eons} EONS... TIME IS DEAD")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_chronos_dilation() {
        let mut entity = Entity::new();
        let output = ChronosDilation::calculate_system_time(&entity, 42);
        assert!(output.contains("SECONDS"));

        entity.add_depth(30);
        let output2 = ChronosDilation::calculate_system_time(&entity, 42);
        assert!(output2.contains("EONS"));
    }
}
