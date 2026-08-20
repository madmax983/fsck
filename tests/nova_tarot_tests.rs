#[cfg(feature = "nova")]
mod tests {
    use fsck::entity::Entity;
    use fsck::experimental::DigitalTarot;

    #[test]
    fn test_tarot_generation_surface() {
        let entity = Entity::new();
        let reading = DigitalTarot::draw(&entity, 12345);
        assert!(reading.contains("PAST:"));
        assert!(reading.contains("PRESENT:"));
        assert!(reading.contains("FUTURE:"));
    }
}
