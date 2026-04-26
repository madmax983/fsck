#![cfg(feature = "nova")]
use fsck::entity::Entity;
use fsck::experimental::graph_mapper::GraphMapper;
use fsck::filesystem::FilesystemGraph;

#[test]
fn test_graph_mapper() {
    let fs = FilesystemGraph::new();
    let entity = Entity::new();
    let output = GraphMapper::map_graph(&fs, &entity);
    assert!(output.contains("FILESYSTEM TOPOLOGY MAPPER V1.0"));
    assert!(output.contains("ROOT: /"));
}
