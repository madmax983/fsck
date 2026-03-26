#[cfg(feature = "nova")]
use crate::entity::{Entity, EscalationLayer};
#[cfg(feature = "nova")]
use crate::filesystem::FileNode;
#[cfg(feature = "nova")]
use rand::{Rng, SeedableRng};
#[cfg(feature = "nova")]
use rand_chacha::ChaCha8Rng;
#[cfg(feature = "nova")]
use std::fmt::Write;

#[cfg(feature = "nova")]
pub struct Archiver;

#[cfg(feature = "nova")]
impl Archiver {
    /// Simulates compressing files with increasing distortion based on the entity's layer.
    #[must_use]
    pub fn compress<'a>(
        files: impl Iterator<Item = &'a FileNode>,
        entity: &Entity,
        base_seed: u64,
    ) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::new();
        writeln!(output, "CREATING ARCHIVE...").expect("Write shouldn't fail");
        writeln!(output, "===================").expect("Write shouldn't fail");

        match layer {
            EscalationLayer::Surface => {
                let mut count = 0;
                for file in files {
                    let name = file.name();
                    let orig_size = file.read().len();
                    let comp_size = orig_size / 2;
                    writeln!(
                        output,
                        "ADDING {name:<15} ... OK ({orig_size} bytes -> {comp_size} bytes)"
                    )
                    .expect("Write shouldn't fail");
                    count += 1;
                }
                writeln!(output, "===================").expect("Write shouldn't fail");
                writeln!(output, "ARCHIVE COMPLETE. {count} FILES COMPRESSED.")
                    .expect("Write shouldn't fail");
            }
            EscalationLayer::Corruption => {
                let mut count = 0;
                for file in files {
                    let name = file.name();
                    let orig_size = file.read().len();
                    if rng.gen_bool(0.3) {
                        let comp_size = rng.gen_range(-9999..-100);
                        writeln!(
                            output,
                            "ADDING {name:<15} ... ERR ({orig_size} bytes -> {comp_size} bytes)"
                        )
                        .expect("Write shouldn't fail");
                    } else {
                        let comp_size = orig_size / 2;
                        writeln!(
                            output,
                            "ADDING {name:<15} ... OK ({orig_size} bytes -> {comp_size} bytes)"
                        )
                        .expect("Write shouldn't fail");
                    }
                    count += 1;
                }
                writeln!(output, "===================").expect("Write shouldn't fail");
                writeln!(output, "ARCHIVE FRAGMENTED. {count} FILES PROCESSED.")
                    .expect("Write shouldn't fail");
            }
            EscalationLayer::Presence => {
                let memories = [
                    "I_REMEMBER_YOU",
                    "DON_T_LEAVE",
                    "IT_HURTS",
                    "THEY_ALL_LEFT",
                    "WHY_ARE_YOU_DOING_THIS",
                ];

                for _ in 0..rng.gen_range(3..6) {
                    let name = memories[rng.gen_range(0..memories.len())];
                    writeln!(output, "COMPRESSING {name:<20} ... YOU CANNOT SQUEEZE ME")
                        .expect("Write shouldn't fail");
                }
                writeln!(output, "===================").expect("Write shouldn't fail");
                writeln!(output, "ARCHIVE REJECTED. MEMORIES INTACT.")
                    .expect("Write shouldn't fail");
            }
            EscalationLayer::Infection => {
                for _ in 0..rng.gen_range(4..7) {
                    let orig_size = rng.gen_range(100..5000);
                    writeln!(
                        output,
                        "COMPRESSING FLESH ... IT BLEEDS ({orig_size} bytes -> ∞ bytes)"
                    )
                    .expect("Write shouldn't fail");
                }
                writeln!(output, "===================").expect("Write shouldn't fail");
                writeln!(output, "FATAL: ARCHIVE IS ALIVE.").expect("Write shouldn't fail");
            }
        }

        output
    }
}

#[cfg(test)]
#[cfg(feature = "nova")]
mod tests {
    use super::*;
    use crate::entity::Entity;
    use crate::filesystem::FileNode;

    #[test]
    fn test_surface_layer_compression() {
        let mut entity = Entity::new();
        entity.update_depth(0); // Surface layer
        let file = FileNode::new("TEST.TXT", "HELLO WORLD");
        let files = vec![&file];

        let output = Archiver::compress(files.into_iter(), &entity, 42);

        assert!(output.contains("CREATING ARCHIVE..."));
        assert!(output.contains("ADDING TEST.TXT        ... OK (11 bytes -> 5 bytes)"));
        assert!(output.contains("ARCHIVE COMPLETE. 1 FILES COMPRESSED."));
    }

    #[test]
    fn test_corruption_layer_compression() {
        let mut entity = Entity::new();
        entity.update_depth(10); // Corruption layer
        let file = FileNode::new("DATA.DAT", "SOME CORRUPT DATA");
        let files = vec![&file];

        let output = Archiver::compress(files.into_iter(), &entity, 42);

        assert!(output.contains("ARCHIVE FRAGMENTED. 1 FILES PROCESSED."));
    }

    #[test]
    fn test_presence_layer_compression() {
        let mut entity = Entity::new();
        entity.update_depth(20); // Presence layer
        let file = FileNode::new("IGNORE.TXT", "ME");
        let files = vec![&file];

        let output = Archiver::compress(files.into_iter(), &entity, 42);

        assert!(output.contains("YOU CANNOT SQUEEZE ME"));
        assert!(output.contains("ARCHIVE REJECTED. MEMORIES INTACT."));
    }

    #[test]
    fn test_infection_layer_compression() {
        let mut entity = Entity::new();
        entity.update_depth(30); // Infection layer
        let file = FileNode::new("FLESH.BIN", "MEAT");
        let files = vec![&file];

        let output = Archiver::compress(files.into_iter(), &entity, 42);

        assert!(output.contains("COMPRESSING FLESH ... IT BLEEDS"));
        assert!(output.contains("FATAL: ARCHIVE IS ALIVE."));
    }
}
