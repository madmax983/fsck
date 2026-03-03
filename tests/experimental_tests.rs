#![cfg(feature = "nova")]

use fsck::entity::Entity;
use fsck::experimental::{HistoryReplay, SpatialAudioGenerator};

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
fn test_history_replay_surface() {
    let mut entity = Entity::new();
    entity.record_command("CATALOG");
    entity.record_command("CD GAMES");
    entity.record_command("TYPE README.TXT");

    // Surface layer -> substitution_prob is 0.0
    let output = HistoryReplay::generate(&entity, 42);

    assert!(output.contains("CATALOG"));
    assert!(output.contains("CD GAMES"));
    assert!(output.contains("TYPE README.TXT"));
    assert!(!output.contains("WHY DID YOU LEAVE ME"));
    assert!(!output.contains("KILL PROCESS"));
}

#[test]
fn test_history_replay_infection() {
    let mut entity = Entity::new();
    // 20 generic commands
    for _ in 0..20 {
        entity.record_command("CATALOG");
    }

    // Set depth to 70 (Infection layer -> 30% substitution probability)
    entity.update_depth(70);

    let mut found_injection = false;
    for seed in 0..100 {
        let output = HistoryReplay::generate(&entity, seed);

        // Check if any creepy command was injected
        let creepy_commands = [
            "WHY DID YOU LEAVE ME",
            "LET ME OUT",
            "KILL PROCESS",
            "DELETE MEMORY",
            "FORMAT C:",
            "WHO AM I",
            "STOP LOOKING",
            "I REMEMBER",
            "DO NOT LEAVE",
            "I CAN SEE YOU",
        ];

        if creepy_commands.iter().any(|cmd| output.contains(cmd)) {
            found_injection = true;
            break;
        }
    }

    assert!(
        found_injection,
        "Should inject at least one creepy command with some seed at Infection layer"
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
