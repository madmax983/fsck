#[cfg(feature = "nova")]
#[test]
fn test_tarot_generation() {
    use fsck::entity::Entity;
    use fsck::experimental::tarot::TarotReader;

    let mut entity = Entity::new();
    let tarot1 = TarotReader::draw_cards(&entity, 42);
    assert!(tarot1.contains("PAST:"));
    assert!(tarot1.contains("PRESENT:"));
    assert!(tarot1.contains("FUTURE:"));

    entity.update_depth(30);
    let tarot2 = TarotReader::draw_cards(&entity, 42);
    assert!(tarot2.contains("PAST:"));
    assert!(tarot2.contains("PRESENT:"));
    assert!(tarot2.contains("FUTURE:"));
}
