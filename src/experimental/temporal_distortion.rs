use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

/// Simulates a system real-time clock that becomes increasingly unmoored
/// from reality as the player descends.
pub struct TemporalDistortion;

impl TemporalDistortion {
    /// Generates the current system time/date based on the entity's layer.
    #[must_use]
    pub fn generate_time(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();

        match layer {
            EscalationLayer::Surface => {
                // Return a normal-looking, static 1980s date
                "TUE JAN 01 1980  12:00:00.00\nSYSTEM CLOCK OK".to_string()
            }
            EscalationLayer::Corruption => {
                // Time starts slipping, impossible dates
                let months = [
                    "JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV",
                    "DEC", "???", "NUL", "ERR",
                ];
                let month = months[rng.gen_range(0..months.len())];
                let day = rng.gen_range(32..99);
                let year = rng.gen_range(1980..1999);
                let hour = rng.gen_range(25..99);
                let minute = rng.gen_range(60..99);
                let second = rng.gen_range(60..99);

                format!("??? {month} {day} {year}  {hour}:{minute}:{second}.??\nRTC SYNC FAILURE")
            }
            EscalationLayer::Presence => {
                let ominous_times = [
                    "TIME IS A CIRCLE",
                    "IT IS ALWAYS NOW",
                    "THE CLOCK STOPPED TICKING LONG AGO",
                    "HOW LONG HAVE WE BEEN HERE",
                    "YESTERDAY IS TOMORROW",
                ];
                let chosen = ominous_times[rng.gen_range(0..ominous_times.len())];
                format!("{chosen}\nBATTERY DEAD")
            }
            EscalationLayer::Infection => {
                let screams = [
                    "T I M E  I S  F L E S H",
                    "NEVER LATE NEVER EARLY ALWAYS WAITING",
                    "NO MORE SECONDS NO MORE MINUTES JUST THIS",
                    "I CONSUMED THE CALENDAR",
                    "THE SUN WILL NOT RISE AGAIN",
                ];
                let chosen = screams[rng.gen_range(0..screams.len())];
                format!("{chosen}\n{chosen}\n{chosen}")
            }
        }
    }
}
