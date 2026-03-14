#[cfg(all(test, feature = "nova"))]
mod tests {
    use fsck::entity::Entity;
    use fsck::experimental::DefragTool;

    #[test]
    fn test_defrag_runs() {
        let mut entity = Entity::new();
        // Give it some depth so it falls into an interesting layer
        entity.update_depth(10); // Corruption layer

        let output = DefragTool::run_defrag(&entity, 12345);
        assert!(output.contains("STARTING DISK DEFRAGMENTATION"));
        assert!(output.contains("ANALYZING VOLUME"));
        // At corruption layer, the hidden message should appear occasionally as fragmented chunks,
        // or at least have a chance to.
    }
}
