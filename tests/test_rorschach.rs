#![cfg(feature = "nova")]

use fsck::entity::Entity;
use fsck::experimental::RorschachGenerator;

#[test]
fn test_rorschach_generates_symmetric_inkblot() {
    let mut entity = Entity::new();
    let seed = 42;

    let output_surface = RorschachGenerator::generate(&entity, seed);
    assert!(output_surface.contains("INITIATING PSYCHOLOGICAL EVALUATION"));
    assert!(output_surface.contains("WHAT DO YOU SEE"));

    entity.update_depth(10); // Corruption
    let output_corruption = RorschachGenerator::generate(&entity, seed);
    assert!(output_corruption.contains("DOES IT LOOK LIKE A MISTAKE"));

    entity.update_depth(20); // Presence
    let output_presence = RorschachGenerator::generate(&entity, seed);
    assert!(output_presence.contains("IT IS LOOKING BACK AT YOU"));

    entity.update_depth(30); // Infection
    let output_infection = RorschachGenerator::generate(&entity, seed);
    assert!(output_infection.contains("THERE IS ONLY FLESH AND SYMMETRY"));
}
