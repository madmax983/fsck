#![cfg(all(test, feature = "nova"))]

use fsck::entity::Entity;
use fsck::experimental::AstroDisplay;

#[test]
fn test_astro_surface() {
    let mut entity = Entity::new();
    entity.update_depth(0); // Surface

    let output = AstroDisplay::generate_map(&entity, 42);

    assert!(output.contains("ASTROMETRIC SCAN OUTPUT:"));
    assert!(output.contains("STATUS: LOCAL CLUSTER MAPPED. NORMAL READINGS."));
    // Ensure no scary stuff
    assert!(!output.contains("STATUS: ANOMALOUS BACKGROUND RADIATION DETECTED."));
    assert!(!output.contains("THEY ARE NOT STARS"));
}

#[test]
fn test_astro_corruption() {
    let mut entity = Entity::new();
    entity.update_depth(10); // Corruption

    let output = AstroDisplay::generate_map(&entity, 42);

    assert!(output.contains("ASTROMETRIC SCAN OUTPUT:"));
    assert!(output.contains("STATUS: ANOMALOUS BACKGROUND RADIATION DETECTED."));
}

#[test]
fn test_astro_presence() {
    let mut entity = Entity::new();
    entity.update_depth(20); // Presence

    let output = AstroDisplay::generate_map(&entity, 42);

    assert!(output.contains("ASTROMETRIC SCAN OUTPUT:"));
    assert!(output.contains("STATUS: THEY ARE NOT STARS. THEY ARE LOOKING AT YOU."));
}

#[test]
fn test_astro_infection() {
    let mut entity = Entity::new();
    entity.update_depth(30); // Infection

    let output = AstroDisplay::generate_map(&entity, 42);

    assert!(output.contains("ASTROMETRIC SCAN OUTPUT:"));
    assert!(output.contains("STATUS: THE VOID IS HUNGRY. THE SKY IS DEAD."));
}
