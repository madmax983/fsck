use fsck::entity::Entity;
#[cfg(feature = "nova")]
use fsck::experimental::CameraSimulator;

#[cfg(feature = "nova")]
#[test]
fn test_camera_surface() {
    let entity = Entity::new();
    let output = CameraSimulator::capture(&entity, 12345);
    assert!(output.contains("/dev/video0"));
    assert!(output.contains("SUBJECT(S) DETECTED."));
}

#[cfg(feature = "nova")]
#[test]
fn test_camera_infection() {
    let mut entity = Entity::new();
    entity.add_depth(26); // Infection layer
    let output = CameraSimulator::capture(&entity, 12345);
    assert!(output.contains("I CAN SEE YOU."));
}
