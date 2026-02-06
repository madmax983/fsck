use fsck::entity::{Entity, EntityMood, EscalationLayer};

#[test]
fn test_entity_starts_dormant() {
    let entity = Entity::new();
    assert_eq!(entity.layer(), EscalationLayer::Surface);
}

#[test]
fn test_depth_affects_layer() {
    let mut entity = Entity::new();
    entity.update_depth(15);
    assert_eq!(entity.layer(), EscalationLayer::Corruption);
}

#[test]
fn test_interaction_count_tracked() {
    let mut entity = Entity::new();
    entity.record_interaction();
    entity.record_interaction();
    assert_eq!(entity.interaction_count(), 2);
}

#[test]
fn test_mood_shifts_with_depth() {
    let mut entity = Entity::new();
    entity.update_depth(5);
    assert!(matches!(entity.current_mood(), EntityMood::Curious | EntityMood::Dormant));

    entity.update_depth(40);
    // At presence layer, mood should be more intense
    assert!(!matches!(entity.current_mood(), EntityMood::Dormant));
}
