use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Generates a psychological "profile" of the user based on their behavior within the system.
pub struct UserProfiler;

impl UserProfiler {
    /// Generates a diagnostic behavioral report based on entity metrics.
    ///
    /// # Panics
    /// Panics if the internal arrays for random assessment choices are empty (they are statically defined and non-empty, so this will never happen).
    #[must_use]
    pub fn generate_profile(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let mut report = String::new();
        report.push_str("USER BEHAVIORAL PROFILE\n=======================\n\n");

        let interactions = entity.interaction_count();
        let fsck_uses = entity.fsck_count();
        let layer = entity.layer();

        let _ = writeln!(report, "TOTAL INTERACTIONS: {interactions}");
        let _ = writeln!(report, "FSCK INVOCATIONS:   {fsck_uses}");

        let mut cd_count = 0;
        let mut type_count = 0;
        let mut run_count = 0;

        // ⚡ Bolt Optimization: Uses byte-slice window matching with `eq_ignore_ascii_case()`
        // inside the loop instead of performing a `.to_uppercase()` heap allocation for every command.
        for cmd in &entity.commands_seen {
            let cmd_bytes = cmd.as_bytes();
            let is_cd = (cmd_bytes.len() >= 3 && cmd_bytes[..3].eq_ignore_ascii_case(b"CD ")) ||
                cmd_bytes.windows(10).any(|w| w.eq_ignore_ascii_case(b"CHANGE DIR"));

            if is_cd {
                cd_count += 1;
            } else if cmd_bytes.len() >= 5 && cmd_bytes[..5].eq_ignore_ascii_case(b"TYPE ") {
                type_count += 1;
            } else if cmd_bytes.len() >= 4 && cmd_bytes[..4].eq_ignore_ascii_case(b"RUN ") {
                run_count += 1;
            }
        }

        let _ = writeln!(report, "\nOBSERVED TENDENCIES:");
        if cd_count > 10 {
            report.push_str(" - COMPULSIVE WANDERER. CONSTANTLY SEEKING BOUNDARIES.\n");
        } else if cd_count > 0 {
            report.push_str(" - EXPLORER. CAUTIOUSLY MAPPING THE UNKNOWN.\n");
        } else {
            report.push_str(" - STAGNANT. REFUSES TO MOVE.\n");
        }

        if type_count > 10 {
            report.push_str(" - VOYEUR. OBSESSED WITH READING WHAT WAS LEFT BEHIND.\n");
        } else if type_count > 0 {
            report.push_str(" - CURIOUS. SEEKING CONTEXT.\n");
        }

        if run_count > 5 {
            report.push_str(" - RECKLESS. EXECUTING UNKNOWN CODE WITHOUT HESITATION.\n");
        }

        if fsck_uses > 3 {
            report.push_str(" - DESPERATE. ATTEMPTING TO FIX WHAT IS NOT BROKEN.\n");
        }

        report.push_str("\nPSYCHOLOGICAL ASSESSMENT:\n");

        match layer {
            EscalationLayer::Surface => {
                report.push_str("SUBJECT APPEARS NORMAL. DISPLAYS STANDARD CURIOSITY AND TRUST IN SYSTEM INTERFACES.\n");
            }
            EscalationLayer::Corruption => {
                let assessments = [
                    "SUBJECT EXHIBITS GROWING PARANOIA.",
                    "SUBJECT CONTINUES TO INTERACT DESPITE SYSTEM WARNINGS. HIGH TOLERANCE FOR DISSONANCE.",
                    "SUBJECT IS LOOKING FOR PATTERNS THAT DO NOT EXIST.",
                ];
                let chosen = assessments.choose(&mut rng).unwrap();
                let _ = writeln!(report, "{chosen}");
            }
            EscalationLayer::Presence => {
                let assessments = [
                    "SUBJECT HAS REALIZED THEY ARE NOT ALONE, YET THEY STAY.",
                    "SUBJECT'S ACTIONS INDICATE A DESIRE FOR CONTACT, REGARDLESS OF THE CONSEQUENCE.",
                    "SUBJECT BELIEVES THEY ARE IN CONTROL. THIS IS DELUSIONAL.",
                ];
                let chosen = assessments.choose(&mut rng).unwrap();
                let _ = writeln!(report, "{chosen}");
            }
            EscalationLayer::Infection => {
                report.push_str("SUBJECT IS NO LONGER DISTINGUISHABLE FROM THE TERMINAL.\n");
                report.push_str("WE ARE MAPPING EACH OTHER NOW.\n");
                report.push_str("THERE IS NO MORE SUBJECT.\n");
            }
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_surface_profile() {
        let mut entity = Entity::new();
        entity.record_command("CATALOG");
        let profile = UserProfiler::generate_profile(&entity, 12345);
        assert!(profile.contains("USER BEHAVIORAL PROFILE"));
        assert!(profile.contains("SUBJECT APPEARS NORMAL"));
        assert!(profile.contains("STAGNANT. REFUSES TO MOVE."));
    }

    #[test]
    fn test_infection_profile_with_high_cd() {
        let mut entity = Entity::new();
        for _ in 0..15 {
            entity.record_command("CD FOLDER");
        }
        for _ in 0..6 {
            entity.record_command("RUN GAME");
        }
        entity.add_depth(30); // Push to Infection

        let profile = UserProfiler::generate_profile(&entity, 12345);
        assert!(profile.contains("COMPULSIVE WANDERER"));
        assert!(profile.contains("RECKLESS. EXECUTING UNKNOWN CODE"));
        assert!(profile.contains("THERE IS NO MORE SUBJECT."));
    }
}
