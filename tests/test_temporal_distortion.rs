#![cfg(feature = "nova")]

use fsck::entity::Entity;
use fsck::experimental::TemporalDistortion;

#[test]
fn test_surface_layer_time() {
    let mut entity = Entity::new();
    entity.update_depth(0);
    let time = TemporalDistortion::generate_time(&entity, 42);

    assert!(time.contains("1980"));
    assert!(time.contains("SYSTEM CLOCK OK"));
    assert!(!time.contains("???"));
}

#[test]
fn test_corruption_layer_time() {
    let mut entity = Entity::new();
    entity.update_depth(10);
    let time = TemporalDistortion::generate_time(&entity, 42);

    assert!(time.contains("???"));
    assert!(time.contains("RTC SYNC FAILURE"));
    assert!(!time.contains("SYSTEM CLOCK OK"));
}

#[test]
fn test_infection_layer_time() {
    let mut entity = Entity::new();
    entity.update_depth(30);
    let time = TemporalDistortion::generate_time(&entity, 42);

    let is_creepy = time.contains("F L E S H")
        || time.contains("NEVER LATE")
        || time.contains("NO MORE SECONDS")
        || time.contains("CALENDAR")
        || time.contains("SUN WILL NOT RISE");

    assert!(
        is_creepy,
        "Time output should be horrific at infection layer"
    );
}

#[test]
fn test_temporal_distortion_deterministic() {
    let mut entity = Entity::new();
    entity.update_depth(10);

    let time1 = TemporalDistortion::generate_time(&entity, 42);
    let time2 = TemporalDistortion::generate_time(&entity, 42);

    assert_eq!(
        time1, time2,
        "Time generation should be deterministic for the same seed"
    );
}
