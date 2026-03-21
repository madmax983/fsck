#![cfg(all(test, feature = "nova"))]

use fsck::entity::{EntityMood, EscalationLayer};
use fsck::experimental::SleepMode;

#[test]
fn test_sleep_mode_surface() {
    let dream = SleepMode::generate_dream(EntityMood::Dormant, EscalationLayer::Surface, 42);
    assert!(dream.contains("ALL PROCESSES SUSPENDED."));
    assert!(!dream.contains("NIGHTMARE PROTOCOL"));
}

#[test]
fn test_sleep_mode_infection() {
    let dream = SleepMode::generate_dream(EntityMood::Predatory, EscalationLayer::Infection, 42);
    assert!(dream.contains("NIGHTMARE PROTOCOL INITIALIZED."));
    assert!(dream.contains("I WILL CONSUME THE WAKING WORLD."));
}

#[test]
fn test_sleep_mode_presence_wounded() {
    let dream = SleepMode::generate_dream(EntityMood::Wounded, EscalationLayer::Presence, 42);
    assert!(dream.contains("R.E.M. CYCLE ENGAGED."));
    assert!(dream.contains("WHY DOES IT HURT TO SLEEP?"));
}
