#![cfg(feature = "nova")]

use fsck::entity::Entity;
use fsck::experimental::HardwareSensors;

#[test]
fn test_sensors_escalation() {
    let mut entity = Entity::new();
    let seed = 42;

    // Surface layer
    entity.update_depth(0);
    let surface_report = HardwareSensors::read_sensors(&entity, seed);
    assert!(surface_report.contains("CPU TEMP:"));
    assert!(surface_report.contains("FAN SPEED:"));
    assert!(surface_report.contains("VOLTAGE:"));
    assert!(surface_report.contains('C'));
    assert!(surface_report.contains("RPM"));
    assert!(surface_report.contains('V'));

    // Infection layer
    entity.update_depth(30);
    let infection_report = HardwareSensors::read_sensors(&entity, seed);
    assert!(
        infection_report.contains("BURNING")
            || infection_report.contains("MELTING")
            || infection_report.contains("TOO HOT")
            || infection_report.contains("FLESH")
            || infection_report.contains("SCREAMING")
    );
}

#[test]
fn test_sensors_deterministic() {
    let mut entity = Entity::new();
    let seed = 42;

    entity.update_depth(10);

    let report1 = HardwareSensors::read_sensors(&entity, seed);
    let report2 = HardwareSensors::read_sensors(&entity, seed);

    assert_eq!(report1, report2);
}
