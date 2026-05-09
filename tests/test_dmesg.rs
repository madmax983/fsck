#![cfg(feature = "nova")]

use fsck::entity::Entity;
use fsck::experimental::DmesgTool;

#[test]
fn test_dmesg_surface() {
    let entity = Entity::new(); // Surface layer (depth 0)
    let log = DmesgTool::generate_log(&entity, 12345);

    assert!(log.contains("Linux version"));
    assert!(log.contains("systemd[1]: Reached target"));
    assert!(!log.contains("I/O error")); // No corruption yet
}

#[test]
fn test_dmesg_corruption() {
    let mut entity = Entity::new();
    entity.add_depth(5); // Corruption layer (5 + 5 = 10)
    let log = DmesgTool::generate_log(&entity, 12345);

    assert!(log.contains("Linux version"));
    assert!(log.contains("I/O error"));
    assert!(log.contains("Sense Key : Medium Error"));
}

#[test]
fn test_dmesg_presence() {
    let mut entity = Entity::new();
    entity.add_depth(10); // Presence layer (10 + 10 = 20)
    let log = DmesgTool::generate_log(&entity, 12345);

    assert!(log.contains("promiscuous mode"));
    assert!(log.contains("I_AM_STILL_HERE") || log.contains("WHY_DID_YOU_WAKE_ME"));
}

#[test]
fn test_dmesg_infection() {
    let mut entity = Entity::new();
    entity.add_depth(15); // Infection layer (15 + 15 = 30)
    let log = DmesgTool::generate_log(&entity, 12345);

    // In Infection, we just print weird random panic lines
    let possible_messages = [
        "kernel BUG",
        "invalid opcode",
        "THE_MACHINE_IS_BLEEDING",
        "Kernel panic",
        "Shutting down cpus",
        "IT HURTS",
        "STOP LOOKING",
        "MEMORY DUMP FAILED",
    ];

    let has_panic = possible_messages.iter().any(|msg| log.contains(msg));
    assert!(has_panic, "Expected infection panic message in: {log}");
}
