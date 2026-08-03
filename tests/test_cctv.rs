#![cfg(feature = "nova")]
use fsck::entity::Entity;
use fsck::experimental::cctv::CctvNetwork;

#[test]
fn test_cctv_list_cameras() {
    let entity = Entity::new();
    let output = CctvNetwork::view("", &entity, 42);
    assert!(output.contains("AVAILABLE CAMERAS"));
    assert!(output.contains("CAM 1: FRONT ENTRANCE"));
}

#[test]
fn test_cctv_invalid_camera() {
    let entity = Entity::new();
    let output = CctvNetwork::view("5", &entity, 42);
    assert!(output.contains("NO SIGNAL FROM CAM 5"));

    let output2 = CctvNetwork::view("XYZ", &entity, 42);
    assert!(output2.contains("INVALID CAMERA"));
}

#[test]
fn test_cctv_surface_view() {
    let entity = Entity::new(); // Surface layer
    let output = CctvNetwork::view("1", &entity, 42);
    assert!(output.contains("RAIN FALLING OUTSIDE"));
}

#[test]
fn test_cctv_infection_view() {
    let mut entity = Entity::new();
    entity.update_depth(30); // Infection layer
    let output = CctvNetwork::view("2", &entity, 42);
    assert!(
        output.contains("YOU ARE SITTING AT A TERMINAL")
            || output.contains("THE WALLS ARE LISTENING")
    );
}
