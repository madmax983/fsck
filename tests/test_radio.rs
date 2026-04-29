#![cfg(feature = "nova")]

use fsck::entity::Entity;
use fsck::experimental::RadioTransceiver;

#[test]
fn test_radio_known_frequencies() {
    let entity = Entity::new();

    let result = RadioTransceiver::tune("88.5", &entity, 12345);
    assert!(
        result.contains("light rain"),
        "Should contain weather report. Got: {result}"
    );

    let result = RadioTransceiver::tune("107.7", &entity, 12345);
    assert!(
        result.contains("4 8 15 16 23 42"),
        "Should contain number station. Got: {result}"
    );

    let result = RadioTransceiver::tune("66.6", &entity, 12345);
    assert!(
        result.contains("WALLS"),
        "Should contain creepy easter egg. Got: {result}"
    );
}

#[test]
fn test_radio_invalid_format() {
    let entity = Entity::new();
    let result = RadioTransceiver::tune("FM", &entity, 12345);
    assert!(result.contains("INVALID FREQUENCY"));
}

#[test]
fn test_radio_escalation() {
    let mut entity = Entity::new();
    entity.update_depth(30); // Infection layer
    let result = RadioTransceiver::tune("100.0", &entity, 12345);
    assert!(
        result.contains("TURN IT OFF"),
        "Infection layer should yell. Got: {result}"
    );
}
