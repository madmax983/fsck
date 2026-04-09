#![cfg(feature = "nova")]

use fsck::entity::Entity;
use fsck::experimental::sensors::SensorReadingsGenerator;

#[test]
fn test_surface_sensors() {
    let entity = Entity::new();
    let output = SensorReadingsGenerator::generate_readings(&entity, 42);

    assert!(output.contains("SYSTEM SENSORS ACTIVE..."));
    assert!(output.contains("CPU:"));
    assert!(output.contains("FAN:"));
    assert!(output.contains("STATUS: NOMINAL"));
}

#[test]
fn test_infection_sensors() {
    let mut entity = Entity::new();
    entity.add_depth(30);
    let output = SensorReadingsGenerator::generate_readings(&entity, 42);

    assert!(!output.contains("STATUS: NOMINAL"));
    assert!(output.contains("FATAL:") || output.contains("STATUS: COMBUSTION"));
    assert!(
        output.contains("BURNS")
            || output.contains("MELTDOWN")
            || output.contains("MELTING")
            || output.contains("PAIN")
            || output.contains("BREATHE")
            || output.contains("WIRES")
    );
}
