#![cfg(feature = "nova")]

use fsck::entity::Entity;
use fsck::experimental::CompilerSimulator;

#[test]
fn test_compiler_surface_layer() {
    let mut entity = Entity::new();
    entity.update_depth(0);

    let output = CompilerSimulator::compile("main.c", &entity, 42);
    assert!(output.contains("Building C object main.c"));
    assert!(output.contains("[100%] Built target"));
    assert!(!output.contains("FLESH"));
}

#[test]
fn test_compiler_infection_layer() {
    let mut entity = Entity::new();
    entity.update_depth(30);

    let output = CompilerSimulator::compile("main.c", &entity, 42);
    assert!(output.contains("fatal error"));
    assert!(output.contains("IT HURTS"));
}
