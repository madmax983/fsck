#![cfg(feature = "nova")]

use fsck::entity::Entity;
// We assume it will be exported from experimental
use fsck::experimental::CctvSystem;

#[test]
fn test_cctv_escalation() {
    let mut entity = Entity::new();
    let seed = 42;

    // Surface layer
    entity.update_depth(0);
    let surface_feed = CctvSystem::get_feed(&entity, seed);
    assert!(surface_feed.contains("CAM 01"));
    assert!(surface_feed.contains("ONLINE") || surface_feed.contains("OFFLINE") || surface_feed.contains("STATIC"));

    // Presence layer
    entity.update_depth(20);
    let presence_feed = CctvSystem::get_feed(&entity, seed);
    assert!(presence_feed.contains("SOMEONE IS THERE") || presence_feed.contains("BREATHING") || presence_feed.contains("WATCHING") || presence_feed.contains("LOOKING BACK") || presence_feed.contains("OBSERVATION"));

    // Infection layer
    entity.update_depth(30);
    let infection_feed = CctvSystem::get_feed(&entity, seed);
    assert!(infection_feed.contains("FLESH") || infection_feed.contains("EYES") || infection_feed.contains("TEETH") || infection_feed.contains("BLEEDING"));
}
