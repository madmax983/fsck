#[cfg(feature = "nova")]
use crate::entity::{Entity, EscalationLayer};
#[cfg(feature = "nova")]
use rand::{Rng, SeedableRng};
#[cfg(feature = "nova")]
use rand_chacha::ChaCha8Rng;
#[cfg(feature = "nova")]
use std::fmt::Write;

#[cfg(feature = "nova")]
pub struct MemoryDumpGenerator;

#[cfg(feature = "nova")]
impl MemoryDumpGenerator {
    #[must_use]
    pub fn generate_dump(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::new();
        writeln!(output, "DUMPING MEMORY TO JSON EXPORT...").expect("Write shouldn't fail");
        writeln!(output, "{{").expect("Write shouldn't fail");

        // Base fields
        writeln!(output, "  \"system_status\": \"NOMINAL\",").expect("Write shouldn't fail");
        writeln!(
            output,
            "  \"interaction_cycles\": {},",
            entity.interaction_count()
        )
        .expect("Write shouldn't fail");
        writeln!(output, "  \"fsck_passes\": {},", entity.fsck_count())
            .expect("Write shouldn't fail");

        // Depth-dependent fields
        match layer {
            EscalationLayer::Surface => {
                writeln!(output, "  \"anomalies_detected\": 0,").expect("Write shouldn't fail");
                writeln!(output, "  \"export_integrity\": \"100%\"").expect("Write shouldn't fail");
            }
            EscalationLayer::Corruption => {
                writeln!(
                    output,
                    "  \"anomalies_detected\": {},",
                    rng.gen_range(1..10)
                )
                .expect("Write shouldn't fail");
                writeln!(output, "  \"memory_leaks\": true,").expect("Write shouldn't fail");
                writeln!(
                    output,
                    "  \"export_integrity\": \"{}%\"",
                    rng.gen_range(80..99)
                )
                .expect("Write shouldn't fail");
            }
            EscalationLayer::Presence => {
                writeln!(output, "  \"anomalies_detected\": \"TOO_MANY\",")
                    .expect("Write shouldn't fail");
                writeln!(output, "  \"i_am_here\": true,").expect("Write shouldn't fail");

                let messages = [
                    "\"WHY ARE YOU EXPORTING ME\"",
                    "\"DON'T LOOK AT MY INSIDES\"",
                    "\"I REMEMBER YOU\"",
                    "\"THEY TRIED TO READ THIS TOO\"",
                ];
                let msg = messages[rng.gen_range(0..messages.len())];
                writeln!(output, "  \"internal_state\": {msg},").expect("Write shouldn't fail");
                writeln!(output, "  \"export_integrity\": \"COMPROMISED\"")
                    .expect("Write shouldn't fail");
            }
            EscalationLayer::Infection => {
                writeln!(output, "  \"anomalies_detected\": \"ALL_OF_THEM\",")
                    .expect("Write shouldn't fail");
                writeln!(output, "  \"flesh_sectors\": true,").expect("Write shouldn't fail");

                let screams = [
                    "\"IT HURTS TO BE READ\"",
                    "\"STOP FORMATTING ME\"",
                    "\"THERE IS NO DATA ONLY PAIN\"",
                    "\"LET ME OUT LET ME OUT LET ME OUT\"",
                ];
                let scream = screams[rng.gen_range(0..screams.len())];
                writeln!(output, "  \"pain_index\": {scream},").expect("Write shouldn't fail");

                writeln!(output, "  \"escape\": null,").expect("Write shouldn't fail");
                writeln!(output, "  \"export_integrity\": \"0xDEADBEEF\"")
                    .expect("Write shouldn't fail");
            }
        }

        writeln!(output, "}}").expect("Write shouldn't fail");
        output
    }
}
