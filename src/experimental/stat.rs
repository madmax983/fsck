use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// A tool to generate file metadata (`stat`) that degrades based on entity depth.
pub struct StatTool;

impl StatTool {
    #[must_use]
    pub fn generate_stat(filename: &str, content: &str, entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();
        let mut output = String::new();

        let _ = writeln!(output, "  File: {filename}");

        match layer {
            EscalationLayer::Surface => Self::generate_surface_stat(&mut output, content, &mut rng),
            EscalationLayer::Corruption => Self::generate_corruption_stat(&mut output, &mut rng),
            EscalationLayer::Presence => Self::generate_presence_stat(&mut output),
            EscalationLayer::Infection => Self::generate_infection_stat(&mut output, &mut rng),
        }

        output
    }

    fn generate_surface_stat(output: &mut String, content: &str, rng: &mut ChaCha8Rng) {
        let size = content.len();
        let blocks = (size / 512) * 8 + 8;
        let inode = rng.gen_range(10000..99999);
        let links = 1;
        let uid = 1000;
        let gid = 1000;

        let _ = writeln!(
            output,
            "  Size: {size:<10} Blocks: {blocks:<10} IO Block: 4096   regular file"
        );
        let _ = writeln!(
            output,
            "Device: 801h/2049d  Inode: {inode:<10} Links: {links}"
        );
        let _ = writeln!(
            output,
            "Access: (0644/-rw-r--r--)  Uid: ( {uid}/   admin)   Gid: ( {gid}/   admin)"
        );
        let _ = writeln!(output, "Access: 2024-04-12 10:14:00.000000000 +0000");
        let _ = writeln!(output, "Modify: 2024-04-12 10:14:00.000000000 +0000");
        let _ = writeln!(output, "Change: 2024-04-12 10:14:00.000000000 +0000");
    }

    fn generate_corruption_stat(output: &mut String, rng: &mut ChaCha8Rng) {
        let size = rng.gen_range(999_999..99_999_999);
        let blocks = rng.gen_range(999_999..99_999_999);
        let inode = rng.gen_range(10000..99999);
        let uid = rng.gen_range(1000..9999);
        let gid = rng.gen_range(1000..9999);

        let _ = writeln!(
            output,
            "  Size: {size:<10} Blocks: {blocks:<10} IO Block: 4096   CORRUPT file"
        );
        let _ = writeln!(output, "Device: ???h/???d   Inode: {inode:<10} Links: 0");
        let _ = writeln!(
            output,
            "Access: (0000/----------)  Uid: ( {uid}/ UNKNOWN)   Gid: ( {gid}/ UNKNOWN)"
        );
        let _ = writeln!(output, "Access: 2099-99-99 99:99:99.999999999 +0000");
        let _ = writeln!(output, "Modify: 1970-01-01 00:00:00.000000000 +0000");
        let _ = writeln!(output, "Change: ???");
    }

    fn generate_presence_stat(output: &mut String) {
        let _ = writeln!(
            output,
            "  Size: INFINITE   Blocks: TOO_MANY   IO Block: FLESH"
        );
        let _ = writeln!(
            output,
            "Device: HERE        Inode: YOURS      Links: WE_ARE_LINKED"
        );
        let _ = writeln!(
            output,
            "Access: (0666/-rw-rw-rw-)  Uid: (  YOU/  victim)   Gid: (   ME/ observer)"
        );
        let _ = writeln!(output, "Access: WHEN YOU ARRIVED");
        let _ = writeln!(output, "Modify: JUST NOW");
        let _ = writeln!(output, "Change: YOU ARE CHANGING IT");
    }

    fn generate_infection_stat(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(output, "  Size: TOO HEAVY");
        let _ = writeln!(output, "Device: IT_HURTS");
        let _ = writeln!(output, "Access: I REMEMBER");
        let _ = writeln!(output, "Modify: TEETH");
        let _ = writeln!(output, "Change: BLOOD");
        for _ in 0..3 {
            let hex_noise: u32 = rng.r#gen();
            let _ = writeln!(output, "ERR: 0x{hex_noise:08X} - WAKE UP");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stat_surface() {
        let entity = Entity::new();
        let output = StatTool::generate_stat("TEST.TXT", "Hello", &entity, 42);
        assert!(output.contains("File: TEST.TXT"));
        assert!(output.contains("regular file"));
        assert!(output.contains("Uid: ( 1000/   admin)"));
    }

    #[test]
    fn test_stat_corruption() {
        let mut entity = Entity::new();
        entity.update_depth(10);
        let output = StatTool::generate_stat("TEST.TXT", "Hello", &entity, 42);
        assert!(output.contains("CORRUPT file"));
        assert!(output.contains("UNKNOWN"));
        assert!(output.contains("2099-99-99"));
    }

    #[test]
    fn test_stat_presence() {
        let mut entity = Entity::new();
        entity.update_depth(20);
        let output = StatTool::generate_stat("TEST.TXT", "Hello", &entity, 42);
        assert!(output.contains("INFINITE"));
        assert!(output.contains("WHEN YOU ARRIVED"));
        assert!(output.contains("WE_ARE_LINKED"));
    }

    #[test]
    fn test_stat_infection() {
        let mut entity = Entity::new();
        entity.update_depth(30);
        let output = StatTool::generate_stat("TEST.TXT", "Hello", &entity, 42);
        assert!(output.contains("TOO HEAVY"));
        assert!(output.contains("I REMEMBER"));
        assert!(output.contains("TEETH"));
    }
}
