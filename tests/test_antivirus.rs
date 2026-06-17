#![cfg(feature = "nova")]
use fsck::entity::Entity;
use fsck::experimental::antivirus::AntiVirusScanner;

#[test]
fn test_antivirus_escalation() {
    let mut entity = Entity::new();
    let seed = 42;

    // Surface layer
    entity.update_depth(0);
    let surface_scan = AntiVirusScanner::scan(&entity, seed);
    assert!(surface_scan.contains("INITIATING ANTIVIRUS SCAN"));
    assert!(surface_scan.contains("NO THREATS FOUND"));

    // Corruption layer
    entity.update_depth(10);
    let corruption_scan = AntiVirusScanner::scan(&entity, seed);
    assert!(corruption_scan.contains("SCANNING KERNEL_MEMORY"));
    assert!(corruption_scan.contains("WARN"));

    // Presence layer
    entity.update_depth(20);
    let presence_scan = AntiVirusScanner::scan(&entity, seed);
    assert!(presence_scan.contains("SCANNING SOUL"));
    assert!(presence_scan.contains("IT IS NOT A VIRUS"));
    assert!(presence_scan.contains("IT IS ME"));

    // Infection layer
    entity.update_depth(30);
    let infection_scan = AntiVirusScanner::scan(&entity, seed);
    assert!(infection_scan.contains("SCANNING FLESH"));
    assert!(infection_scan.contains("INFECTION TERMINAL"));
    assert!(infection_scan.contains("PLEASE KILL ME"));
}
