#![cfg(all(test, feature = "nova"))]

use fsck::entity::Entity;
use fsck::experimental::VoiceSynthesizer;

#[test]
fn test_surface_layer_echo() {
    let entity = Entity::new();
    // Entity starts at depth 0, so it's at EscalationLayer::Surface
    let input = "hello world";
    let output = VoiceSynthesizer::synthesize(input, &entity, 42);

    assert_eq!(output, "SPEAK: hello world");
}

#[test]
fn test_infection_layer_distortion() {
    let mut entity = Entity::new();
    entity.update_depth(30); // Depth 30 = EscalationLayer::Infection

    let input = "please let me out";
    let output = VoiceSynthesizer::synthesize(input, &entity, 42);

    // Output should be distorted, not matching the input
    assert_ne!(output, format!("SPEAK: {input}"));
    assert!(!output.is_empty());
}
