#![cfg(feature = "nova")]

use fsck::entity::Entity;
use fsck::experimental::DmesgGenerator;

#[test]
fn test_dmesg_escalation() {
    let mut entity = Entity::new();
    let surface_log = DmesgGenerator::generate_dmesg(&entity, 42);
    assert!(surface_log.contains("kernel:"));
    assert!(
        surface_log.contains("CPU0")
            || surface_log.contains("ACPI")
            || surface_log.contains("usb")
            || surface_log.contains("sd")
            || surface_log.contains("EXT4")
    );

    entity.add_depth(10); // Presence layer
    let presence_log = DmesgGenerator::generate_dmesg(&entity, 42);
    assert!(
        presence_log.contains("kernel:")
            || presence_log.contains("WARNING")
            || presence_log.contains("FATAL")
            || presence_log.contains("memory block")
            || presence_log.contains("ACPI")
            || presence_log.contains("usb")
            || presence_log.contains("sd")
            || presence_log.contains("EXT4")
            || presence_log.contains("audit")
            || presence_log.contains("net_ratelimit")
    );
}
