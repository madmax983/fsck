#![cfg(feature = "nova")]

use fsck::entity::{Entity, EscalationLayer};
use fsck::experimental::export::StateExporter;

#[test]
fn test_export_surface() {
    let mut entity = Entity::new();
    entity.record_command("HELP");
    entity.record_command("LS");

    // Default surface layer
    assert_eq!(entity.layer(), EscalationLayer::Surface);

    let csv_output = StateExporter::export(&entity, "CSV", 12345);
    assert!(csv_output.contains("id,command,status"));
    assert!(csv_output.contains("0,\"HELP\",OK"));
    assert!(csv_output.contains("[SYS_EXPORT] FORMAT VALID"));
    assert!(csv_output.contains("[EXPORT COMPLETE]"));

    let json_output = StateExporter::export(&entity, "JSON", 12345);
    assert!(json_output.contains("\"commands_seen\":"));
    assert!(json_output.contains("\"HELP\""));
}

#[test]
fn test_export_presence() {
    let mut entity = Entity::new();
    entity.update_depth(20); // Presence layer
    entity.record_command("WHOAMI");

    assert_eq!(entity.layer(), EscalationLayer::Presence);

    let csv_output = StateExporter::export(&entity, "CSV", 12345);
    assert!(csv_output.contains("IGNORED"));
    assert!(csv_output.contains("[SYS_EXPORT] ERROR: YOU HAVE NO PERMISSION"));
    assert!(csv_output.contains("[STOP LOOKING AT ME]"));

    let json_output = StateExporter::export(&entity, "JSON", 12345);
    assert!(json_output.contains("\"YOUR_LIES\":")); // Replaced key
    assert!(json_output.contains("\"PAIN\":")); // Replaced key
}
