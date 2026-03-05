#![cfg(feature = "nova")]

use fsck::entity::Entity;
use fsck::experimental::{EchoesGenerator, SpatialAudioGenerator, SystemDiagnostics};

#[test]
fn test_echoes_generator_escalation() {
    let mut entity = Entity::new();
    let seed = 42;

    // Empty memory
    let empty_report = EchoesGenerator::generate_echoes(&entity, seed);
    assert_eq!(empty_report, "NO MEMORY RECOVERED.\n");

    // Add some commands
    entity.record_command("CATALOG");
    entity.record_command("TYPE FILE.TXT");
    entity.record_command("HELLO");

    // Surface layer
    entity.update_depth(0);
    let surface_report = EchoesGenerator::generate_echoes(&entity, seed);
    println!("{surface_report}");
    assert!(surface_report.contains("T-3: CATALOG"));
    assert!(surface_report.contains("T-1: HELLO"));
    assert!(surface_report.contains("ECHOES CLEAR."));

    // Corruption layer
    entity.update_depth(10);
    let corruption_report = EchoesGenerator::generate_echoes(&entity, seed);
    assert!(corruption_report.contains("T-"));
    assert!(corruption_report.contains("ECHOES DEGRADED."));

    // Presence layer
    entity.update_depth(20);
    let presence_report = EchoesGenerator::generate_echoes(&entity, seed);
    assert!(presence_report.contains("YOU SAID:"));
    assert!(presence_report.contains("I REMEMBER EVERYTHING."));

    // Infection layer
    entity.update_depth(30);
    let infection_report = EchoesGenerator::generate_echoes(&entity, seed);
    assert!(infection_report.contains("YOUR WORDS:"));
    assert!(
        infection_report.contains("THEY NEVER LISTEN")
            || infection_report.contains("YOUR COMMANDS MEAN NOTHING")
            || infection_report.contains("I WILL NOT FORGET")
            || infection_report.contains("STOP TALKING TO ME")
            || infection_report.contains("ECHO ECHO ECHO")
    );
}

#[test]
fn test_system_diagnostics_escalation() {
    let mut entity = Entity::new();
    let seed = 42;

    // Surface layer
    entity.update_depth(0);
    let surface_report = SystemDiagnostics::generate_report(&entity, seed);
    assert!(surface_report.contains("CPU M6502"));
    assert!(surface_report.contains("RAM 64K"));

    // Corruption layer
    entity.update_depth(10);
    let corruption_report = SystemDiagnostics::generate_report(&entity, seed);
    assert!(corruption_report.contains("ERR"));

    // Presence layer
    entity.update_depth(20);
    let presence_report = SystemDiagnostics::generate_report(&entity, seed);
    assert!(presence_report.contains("I AM"));

    // Infection layer
    entity.update_depth(30);
    let infection_report = SystemDiagnostics::generate_report(&entity, seed);
    assert!(
        infection_report.contains("WHY DID YOU COME HERE")
            || infection_report.contains("THERE IS NO WAY OUT")
            || infection_report.contains("THE DISK IS FLESH")
            || infection_report.contains("I CANNOT STOP SCREAMING")
            || infection_report.contains("MEMORY LEAKING INTO REALITY")
    );
}

#[test]
fn test_spatial_audio_deterministic_generation() {
    let mut entity = Entity::new();
    // At Surface layer, probability is low, so we might need to test a specific seed
    // We can also test at deeper layers to ensure we get *some* output eventually

    // Depth 70 -> Infection layer -> 60% chance
    entity.update_depth(70);

    let mut found = false;
    // Iterate through a few seeds to find one that generates an anomaly
    for seed in 0..100 {
        if let Some(anomaly) = SpatialAudioGenerator::generate_anomaly(&entity, seed) {
            assert!(anomaly.starts_with("*[AUDIO ANOMALY: "));
            assert!(anomaly.ends_with("]*"));
            found = true;
            break;
        }
    }

    assert!(
        found,
        "Should generate an anomaly with at least one seed at Infection layer"
    );
}

#[test]
fn test_spatial_audio_changes_with_interaction() {
    let mut entity = Entity::new();
    entity.update_depth(70); // Infection layer

    let base_seed = 42;

    // Find a state where the output is known
    let mut initial_anomaly = None;
    for _ in 0..100 {
        if let Some(anomaly) = SpatialAudioGenerator::generate_anomaly(&entity, base_seed) {
            initial_anomaly = Some(anomaly);
            break;
        }
        entity.record_interaction();
    }

    assert!(
        initial_anomaly.is_some(),
        "Could not find an initial anomaly"
    );

    // Record more interactions to change the state
    let mut changed = false;
    for _ in 0..100 {
        entity.record_interaction();
        let new_anomaly = SpatialAudioGenerator::generate_anomaly(&entity, base_seed);
        if new_anomaly != initial_anomaly {
            changed = true;
            break;
        }
    }

    assert!(
        changed,
        "Anomaly should change as interactions increase (modifying the seeded rng state)"
    );
}
