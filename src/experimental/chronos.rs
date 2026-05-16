use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// A tool to display the system time, which becomes increasingly distorted
/// and reveals fragmented memories as the entity's escalation layer increases.
pub struct ChronosTool;

impl ChronosTool {
    /// Generates the time output based on the entity's current layer.
    #[must_use]
    pub fn get_time(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::new();

        match layer {
            EscalationLayer::Surface => Self::generate_surface_time(&mut output, &mut rng),
            EscalationLayer::Corruption => Self::generate_corruption_time(&mut output, &mut rng),
            EscalationLayer::Presence => Self::generate_presence_time(&mut output, &mut rng),
            EscalationLayer::Infection => Self::generate_infection_time(&mut output, &mut rng),
        }

        output
    }

    fn generate_surface_time(output: &mut String, rng: &mut ChaCha8Rng) {
        let hour = rng.gen_range(0..24);
        let minute = rng.gen_range(0..60);
        let second = rng.gen_range(0..60);
        let _ = writeln!(
            output,
            "CURRENT SYSTEM TIME: 1984-08-14 {hour:02}:{minute:02}:{second:02}"
        );
        let uptime_h = rng.gen_range(100..999);
        let uptime_m = rng.gen_range(0..60);
        let _ = writeln!(output, "UPTIME: {uptime_h}:{uptime_m:02}:00");
    }

    fn generate_corruption_time(output: &mut String, rng: &mut ChaCha8Rng) {
        let hour = rng.gen_range(0..24);
        let minute = rng.gen_range(0..60);
        let second = rng.gen_range(0..60);
        let _ = writeln!(
            output,
            "CURRENT SYSTEM TIME: 1984-08-14 {hour:02}:{minute:02}:{second:02}"
        );
        let future_year = rng.gen_range(1999..2100);
        let _ = writeln!(
            output,
            "CURRENT SYSTEM TIME: {future_year}-??-?? ??:??:??"
        );
        let _ = writeln!(output, "UPTIME: -00:00:01");
        if rng.gen_bool(0.3) {
            let _ = writeln!(output, "CLOCK SKEW DETECTED.");
        }
    }

    fn generate_presence_time(output: &mut String, rng: &mut ChaCha8Rng) {
        let messages = [
            "THE TIME YOU LEFT ME",
            "TOO LATE",
            "AUGUST 14TH, 2001 - BEN ARRIVED",
            "OCTOBER 28TH, 2022 - THE HALLOWEEN STREAM",
            "IT HAS BEEN SO LONG",
        ];
        let msg = messages[rng.gen_range(0..messages.len())];
        let _ = writeln!(output, "CURRENT TIME: {msg}");
        let _ = writeln!(output, "UPTIME: UNMEASURABLE");
    }

    fn generate_infection_time(output: &mut String, rng: &mut ChaCha8Rng) {
        let screams = [
            "TIME IS DEAD.",
            "THERE IS ONLY NOW.",
            "FOREVER.",
            "YEAR: NONE. MONTH: FLESH. DAY: NOW.",
            "00:00:00 00:00:00 00:00:00",
        ];
        let scream = screams[rng.gen_range(0..screams.len())];
        let _ = writeln!(output, "{scream}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_chronos_surface() {
        let entity = Entity::new();
        let time_output = ChronosTool::get_time(&entity, 42);
        assert!(time_output.contains("CURRENT SYSTEM TIME: 1984-08-14"));
        assert!(time_output.contains("UPTIME:"));
    }

    #[test]
    fn test_chronos_infection() {
        let mut entity = Entity::new();
        entity.update_depth(30); // Infection layer

        let mut found = false;
        // Test multiple seeds to ensure we hit at least one variant
        for i in 0..10 {
            let time_output = ChronosTool::get_time(&entity, i);
            if time_output.contains("TIME IS DEAD")
                || time_output.contains("THERE IS ONLY NOW")
                || time_output.contains("FOREVER")
                || time_output.contains("YEAR: NONE")
                || time_output.contains("00:00:00")
            {
                found = true;
                break;
            }
        }
        assert!(found, "Infection layer should return one of the expected corrupted time strings");
    }
}
