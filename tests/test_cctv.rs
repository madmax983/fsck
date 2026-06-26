#![cfg(feature = "nova")]

use fsck::entity::Entity;
use fsck::experimental::CctvViewer;

#[test]
fn test_cctv_surface() {
    let mut entity = Entity::new();
    entity.update_depth(0);

    let output = CctvViewer::view(&entity, "1", 42);
    assert!(output.contains("CONNECTING TO CAMERA 1"));
    assert!(output.contains("FEED SECURE. SCENE:"));
}

#[test]
fn test_cctv_infection() {
    let mut entity = Entity::new();
    entity.update_depth(30);

    let output = CctvViewer::view(&entity, "2", 42);
    assert!(output.contains("CONNECTING TO CAMERA 2"));
    assert!(output.contains("IT IS LOOKING AT YOU."));
}

#[test]
fn test_cctv_presence_commands() {
    let mut entity = Entity::new();
    entity.update_depth(20);
    entity.record_command("HELP");
    entity.record_command("FSCK");

    let output = CctvViewer::view(&entity, "3", 42);
    assert!(output.contains("CONNECTING TO CAMERA 3"));
    assert!(output.contains("FEED COMPROMISED"));
    assert!(output.contains("HELP") || output.contains("FSCK") || output.contains("mirrors"));
}
