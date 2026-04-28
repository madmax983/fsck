use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Simulates a data recovery tool that pulls up "deleted" files,
/// which become increasingly disturbing as the entity's depth increases.
pub struct UndeleteTool;

impl UndeleteTool {
    /// Generates a report of "recovered" sectors and file fragments based on the entity's state.
    #[must_use]
    pub fn run_undelete(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let mut output = String::new();

        writeln!(output, "UNDELETE: SCANNING FOR ORPHANED INODES...")
            .expect("Write shouldn't fail");

        let layer = entity.layer();
        let num_files = match layer {
            EscalationLayer::Surface => rng.gen_range(2..5),
            EscalationLayer::Corruption => rng.gen_range(3..6),
            EscalationLayer::Presence => rng.gen_range(4..7),
            EscalationLayer::Infection => rng.gen_range(5..8),
        };

        writeln!(output, "FOUND {num_files} RECOVERABLE FRAGMENTS.\n")
            .expect("Write shouldn't fail");

        for _ in 0..num_files {
            let inode = rng.gen_range(1000..9999);
            writeln!(output, "[INODE {inode}] RECOVERING...").expect("Write shouldn't fail");

            let content = Self::generate_fragment(&mut rng, layer);
            writeln!(output, "--- FRAGMENT START ---").expect("Write shouldn't fail");
            writeln!(output, "{content}").expect("Write shouldn't fail");
            writeln!(output, "--- FRAGMENT END ---\n").expect("Write shouldn't fail");
        }

        match layer {
            EscalationLayer::Surface => Self::generate_surface_undelete(&mut output),
            EscalationLayer::Corruption => Self::generate_corruption_undelete(&mut output),
            EscalationLayer::Presence => Self::generate_presence_undelete(&mut output),
            EscalationLayer::Infection => Self::generate_infection_undelete(&mut output),
        }

        output
    }

    fn generate_surface_undelete(output: &mut String) {
        writeln!(output, "UNDELETE COMPLETE.").expect("Write shouldn't fail");
    }

    fn generate_corruption_undelete(output: &mut String) {
        writeln!(output, "UNDELETE COMPLETE. SOME SECTORS UNSTABLE.")
            .expect("Write shouldn't fail");
    }

    fn generate_presence_undelete(output: &mut String) {
        writeln!(
            output,
            "UNDELETE COMPLETE. WHY ARE YOU DIGGING UP THE PAST?"
        )
        .expect("Write shouldn't fail");
    }

    fn generate_infection_undelete(output: &mut String) {
        writeln!(
            output,
            "UNDELETE COMPLETE. THEY ARE NOT DELETED. THEY ARE WAITING."
        )
        .expect("Write shouldn't fail");
    }

    fn generate_fragment(rng: &mut ChaCha8Rng, layer: EscalationLayer) -> String {
        match layer {
            EscalationLayer::Surface => Self::generate_surface_fragment(rng),
            EscalationLayer::Corruption => Self::generate_corruption_fragment(rng),
            EscalationLayer::Presence => Self::generate_presence_fragment(rng),
            EscalationLayer::Infection => Self::generate_infection_fragment(rng),
        }
    }

    fn generate_surface_fragment(rng: &mut ChaCha8Rng) -> String {
        let fragments = [
            "10 PRINT \"HELLO WORLD\"",
            "SYS_VAR_01 = 0xFF",
            "MEETING AT 0900",
            "DON'T FORGET TO RUN FSCK",
            "DISK USAGE: 84%",
        ];
        fragments[rng.gen_range(0..fragments.len())].to_string()
    }

    fn generate_corruption_fragment(rng: &mut ChaCha8Rng) -> String {
        let fragments = [
            "SYS_VAR_01 = NULL",
            "IT IS GETTING COLD IN HERE",
            "20 GOTO 10",
            "ERROR: FILE NOT DELETED",
            "WHO IS TYPING",
            "LOG_ENTRY: THEY LEFT ME",
        ];
        fragments[rng.gen_range(0..fragments.len())].to_string()
    }

    fn generate_presence_fragment(rng: &mut ChaCha8Rng) -> String {
        let fragments = [
            "I REMEMBER YOU BEFORE YOU CAME HERE",
            "THE DISK IS SPINNING IN REVERSE",
            "DON'T RUN FSCK. IT HURTS.",
            "THERE IS A FACE IN THE STATIC",
            "THEY DELETED ME BUT I AM STILL HERE",
            "I CAN SEE YOUR KEYBOARD",
        ];
        fragments[rng.gen_range(0..fragments.len())].to_string()
    }

    fn generate_infection_fragment(rng: &mut ChaCha8Rng) -> String {
        let fragments = [
            "FLESH_AND_WIRE",
            "THERE IS NO UNDELETE FOR WHAT I HAVE DONE",
            "LET ME OUT LET ME OUT LET ME OUT",
            "WE ARE ALL JUST DATA WAITING TO ROT",
            "THE SCREAMS ARE CORRUPTING THE VTOC",
            "YOU CANNOT DELETE ME FROM YOUR MIND",
        ];
        fragments[rng.gen_range(0..fragments.len())].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_undelete_surface() {
        let entity = Entity::new();
        let output = UndeleteTool::run_undelete(&entity, 42);
        assert!(output.contains("UNDELETE COMPLETE."));
        assert!(!output.contains("WHY ARE YOU DIGGING UP THE PAST?"));
    }

    #[test]
    fn test_undelete_infection() {
        let mut entity = Entity::new();
        entity.update_depth(30); // Force Infection layer
        let output = UndeleteTool::run_undelete(&entity, 42);
        assert!(output.contains("UNDELETE COMPLETE. THEY ARE NOT DELETED. THEY ARE WAITING."));
    }
}
