#![cfg(feature = "nova")]

use fsck::entity::{EntityMood, EscalationLayer};
use fsck::experimental::BlackMirror;

#[test]
fn test_mirror_surface() {
    let result = BlackMirror::gaze(EntityMood::Dormant, EscalationLayer::Surface, 42);
    assert!(result.contains("GAZING INTO THE BLACK MIRROR..."));
    assert!(result.contains("YOU SEE ONLY YOUR OWN REFLECTION"));
}

#[test]
fn test_mirror_corruption_curious() {
    let result = BlackMirror::gaze(EntityMood::Curious, EscalationLayer::Corruption, 42);
    assert!(result.contains("YOUR REFLECTION BLINKS WHEN YOU DON'T"));
}

#[test]
fn test_mirror_presence() {
    let result = BlackMirror::gaze(EntityMood::Wounded, EscalationLayer::Presence, 42);
    assert!(result.contains("THE MIRROR DOES NOT REFLECT YOU"));
}

#[test]
fn test_mirror_infection_predatory() {
    let result = BlackMirror::gaze(EntityMood::Predatory, EscalationLayer::Infection, 42);
    assert!(result.contains("IT IS COMING THROUGH"));
}
