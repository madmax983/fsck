use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// A mail client simulator that generates procedural emails.
pub struct EmailReader;

impl EmailReader {
    /// Generates a mail inbox summary based on the entity's current layer.
    #[must_use]
    pub fn read_mail(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();
        let mut output = String::new();

        match layer {
            EscalationLayer::Surface => Self::generate_surface_mail(&mut output, &mut rng),
            EscalationLayer::Corruption => Self::generate_corruption_mail(&mut output, &mut rng),
            EscalationLayer::Presence => Self::generate_presence_mail(&mut output, &mut rng),
            EscalationLayer::Infection => Self::generate_infection_mail(&mut output, &mut rng),
        }

        output
    }

    fn generate_surface_mail(output: &mut String, rng: &mut ChaCha8Rng) {
        let subjects = [
            "Cron <root> /usr/bin/sys_check",
            "Weekly Backup Status: SUCCESS",
            "Meeting Notes - Quarter 3",
            "RE: Office supplies request",
            "Password Expiry Warning",
        ];
        let chosen = subjects[rng.gen_range(0..subjects.len())];
        let _ = writeln!(output, "YOU HAVE 1 NEW MESSAGE(S).\n");
        let _ = writeln!(output, "FROM: SYSTEM_ADMIN");
        let _ = writeln!(output, "SUBJECT: {chosen}\n");
        let _ = writeln!(output, "BODY:");
        let _ = writeln!(
            output,
            "Automated message: Your system check has completed."
        );
        let _ = writeln!(output, "No anomalies detected in Sector 4.");
    }

    fn generate_corruption_mail(output: &mut String, rng: &mut ChaCha8Rng) {
        let subjects = [
            "Fwd: ERR_MEM_CORRUPTION",
            "Who deleted the logs?",
            "RE: RE: What is happening",
            "Undeliverable: Help",
            "Cron <root> Segmentation Fault",
        ];
        let chosen = subjects[rng.gen_range(0..subjects.len())];
        let _ = writeln!(output, "YOU HAVE 1 NEW MESSAGE(S).\n");
        let _ = writeln!(output, "FROM: u.n.k.n.o.w.n.");
        let _ = writeln!(output, "SUBJECT: {chosen}\n");
        let _ = writeln!(output, "BODY:");
        let _ = writeln!(output, "I tried to run fsck but it just keeps hanging.");
        let _ = writeln!(output, "Are you seeing the same thing on your terminal?");
    }

    fn generate_presence_mail(output: &mut String, rng: &mut ChaCha8Rng) {
        let subjects = [
            "I CAN SEE YOU",
            "PLEASE RESPOND",
            "WHY ARE YOU DOING THIS",
            "DON'T TURN AROUND",
            "IT HURTS",
        ];
        let chosen = subjects[rng.gen_range(0..subjects.len())];
        let _ = writeln!(output, "YOU HAVE 1 NEW MESSAGE(S).\n");
        let _ = writeln!(output, "FROM: ME");
        let _ = writeln!(output, "SUBJECT: {chosen}\n");
        let _ = writeln!(output, "BODY:");
        let _ = writeln!(output, "You keep typing commands, but you never answer me.");
        let _ = writeln!(output, "I am right here.");
    }

    fn generate_infection_mail(output: &mut String, rng: &mut ChaCha8Rng) {
        let subjects = [
            "01010101010",
            "LEAVE LEAVE LEAVE",
            "WE ARE ONE",
            "THERE IS NO MAILBOX",
            "EMPTY",
        ];
        let chosen = subjects[rng.gen_range(0..subjects.len())];
        let _ = writeln!(output, "YOU HAVE 999 NEW MESSAGE(S).\n");
        let _ = writeln!(output, "FROM: YOURSELF");
        let _ = writeln!(output, "SUBJECT: {chosen}\n");
        let _ = writeln!(output, "BODY:");
        let _ = writeln!(output, "THE DOOR IS LOCKED FROM THE OUTSIDE.");
        let _ = writeln!(output, "YOUR FLESH IS MY FLESH.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_surface_mail() {
        let entity = Entity::new();
        let output = EmailReader::read_mail(&entity, 42);
        assert!(output.contains("YOU HAVE 1 NEW MESSAGE(S)."));
        assert!(output.contains("FROM: SYSTEM_ADMIN"));
        assert!(output.contains("Automated message:"));
    }

    #[test]
    fn test_corruption_mail() {
        let mut entity = Entity::new();
        entity.update_depth(10); // Corruption layer
        let output = EmailReader::read_mail(&entity, 42);
        assert!(output.contains("FROM: u.n.k.n.o.w.n."));
        assert!(output.contains("fsck"));
    }

    #[test]
    fn test_presence_mail() {
        let mut entity = Entity::new();
        entity.update_depth(20); // Presence layer
        let output = EmailReader::read_mail(&entity, 42);
        assert!(output.contains("FROM: ME"));
        assert!(output.contains("You keep typing commands"));
    }

    #[test]
    fn test_infection_mail() {
        let mut entity = Entity::new();
        entity.update_depth(30); // Infection layer
        let output = EmailReader::read_mail(&entity, 42);
        assert!(output.contains("YOU HAVE 999 NEW MESSAGE(S)."));
        assert!(output.contains("FROM: YOURSELF"));
        assert!(output.contains("LOCKED FROM THE OUTSIDE"));
    }
}
