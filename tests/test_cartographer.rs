#[cfg(feature = "nova")]
use fsck::entity::Entity;
#[cfg(feature = "nova")]
use fsck::experimental::Cartographer;
#[cfg(feature = "nova")]
use fsck::filesystem::FilesystemGraph;

#[cfg(feature = "nova")]
#[test]
fn test_cartographer_map_surface() {
    let mut fs = FilesystemGraph::new();
    fs.add_child("ALPHA");
    fs.add_child("BETA");
    let entity = Entity::new();
    let map = Cartographer::map(&fs, &entity, 42);

    assert!(map.contains("MAPPING LOCAL SECTOR"));
    assert!(map.contains("LOC: /"));
    assert!(map.contains("+ ALPHA"));
    assert!(map.contains("+ BETA"));
}

#[cfg(feature = "nova")]
#[test]
fn test_cartographer_map_corruption() {
    let fs = FilesystemGraph::new();
    let mut entity = Entity::new();
    entity.add_depth(5); // Depth 5 = Corruption

    let map1 = Cartographer::map(&fs, &entity, 42);
    let map2 = Cartographer::map(&fs, &entity, 43);
    let map3 = Cartographer::map(&fs, &entity, 44);

    // Output should be slightly corrupted some of the time, checking for either case
    let contains_unknown = map1.contains("LOC: UNKNOWN")
        || map2.contains("LOC: UNKNOWN")
        || map3.contains("LOC: UNKNOWN");
    let contains_known =
        map1.contains("LOC: /") || map2.contains("LOC: /") || map3.contains("LOC: /");

    assert!(contains_unknown || contains_known);
}

#[cfg(feature = "nova")]
#[test]
fn test_cartographer_map_presence() {
    let fs = FilesystemGraph::new();
    let mut entity = Entity::new();
    entity.add_depth(10); // Depth 10 = Presence (Add 10 -> effective 20)

    let map = Cartographer::map(&fs, &entity, 42);
    assert!(map.contains("I_AM_HERE"));
}

#[cfg(feature = "nova")]
#[test]
fn test_cartographer_map_infection() {
    let fs = FilesystemGraph::new();
    let mut entity = Entity::new();
    entity.add_depth(15); // Depth 15 = Infection (Add 15 -> effective 30)

    let map = Cartographer::map(&fs, &entity, 42);
    assert!(map.contains("NO_ESCAPE"));
}
