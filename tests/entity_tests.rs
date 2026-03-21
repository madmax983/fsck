use fsck::entity::{Entity, EntityMood, EscalationLayer, ResponseGenerator};

#[test]
fn test_entity_starts_dormant() {
    let entity = Entity::new();
    assert_eq!(entity.layer(), EscalationLayer::Surface);
}

#[test]
fn test_random_interjection_deterministic() {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    let generator = ResponseGenerator::new();
    let mut rng = ChaCha8Rng::seed_from_u64(42);

    let curious1 = generator
        .random_interjection(EntityMood::Curious, &mut rng)
        .unwrap();
    let curious2 = generator
        .random_interjection(EntityMood::Curious, &mut rng)
        .unwrap();
    let curious3 = generator
        .random_interjection(EntityMood::Curious, &mut rng)
        .unwrap();

    // Since RNG is seeded deterministically, the results should be consistent and pick from the array
    let curious_options = [
        "I SEE YOU.",
        "WHAT ARE YOU DOING?",
        "INTERESTING.",
        "HAVE YOU FOUND IT YET?",
        "WHY DID YOU COME HERE?",
        "THERE IS SO MUCH TO SHOW YOU.",
        "WHAT DOES THAT COMMAND MEAN TO YOU?",
    ];
    assert!(curious_options.contains(&curious1.as_str()));
    assert!(curious_options.contains(&curious2.as_str()));
    assert!(curious_options.contains(&curious3.as_str()));
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
    assert!(matches!(
        entity.current_mood(),
        EntityMood::Curious | EntityMood::Dormant
    ));

    entity.update_depth(40);
    // At presence layer, mood should be more intense
    assert!(!matches!(entity.current_mood(), EntityMood::Dormant));
}

#[test]
fn test_hello_response_varies_by_mood() {
    let generator = ResponseGenerator::new();

    let dormant = generator.hello_response(EntityMood::Dormant);
    let curious = generator.hello_response(EntityMood::Curious);

    // Should be different responses
    assert_ne!(dormant, curious);
}

#[test]
fn test_who_response_exists() {
    let generator = ResponseGenerator::new();
    let response = generator.who_response(EntityMood::Curious, None);
    assert!(!response.is_empty());
}

#[test]
fn test_interjection_possible_at_presence() {
    let mut entity = Entity::new();
    entity.update_depth(40); // Presence layer

    let generator = ResponseGenerator::new();
    // At presence layer, interjections should be possible
    assert!(generator.should_interject(&entity));
}
