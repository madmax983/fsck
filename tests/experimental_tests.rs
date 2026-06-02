#![cfg(feature = "nova")]

use fsck::entity::Entity;
use fsck::experimental::{
    HardwareSensors, MemoryDumpGenerator, ProcessMonitor, SentimentAnalyzer, SpatialAudioGenerator,
    SystemDiagnostics,
};

#[test]
fn test_webcam_escalation() {
    let mut entity = Entity::new();
    let seed = 42;

    // Surface layer
    entity.update_depth(0);
    let surface_dump = fsck::experimental::WebcamCapture::capture(&entity, seed);
    assert!(surface_dump.contains("ACTIVE"));

    // Infection layer
    entity.update_depth(30);
    let infection_dump = fsck::experimental::WebcamCapture::capture(&entity, seed);
    assert!(infection_dump.contains("RECIPROCAL OBSERVATION"));
}

#[test]
fn test_memdump_escalation() {
    let mut entity = Entity::new();
    let seed = 42;

    // Surface layer
    entity.update_depth(0);
    let surface_dump = MemoryDumpGenerator::generate_dump(&entity, seed);
    assert!(surface_dump.contains("\"NOMINAL\""));
    assert!(surface_dump.contains("\"export_integrity\": \"100%\""));

    // Infection layer
    entity.update_depth(30);
    let infection_dump = MemoryDumpGenerator::generate_dump(&entity, seed);
    assert!(infection_dump.contains("\"ALL_OF_THEM\""));
    assert!(infection_dump.contains("\"flesh_sectors\": true"));
    assert!(infection_dump.contains("\"escape\": null"));
    assert!(infection_dump.contains("\"export_integrity\": \"0xDEADBEEF\""));
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

#[test]
fn test_process_monitor_escalation() {
    let mut entity = Entity::new();
    let seed = 42;

    // Surface layer
    entity.update_depth(0);
    let surface_report = ProcessMonitor::generate_process_list(&entity, seed);
    assert!(surface_report.contains("INIT"));
    assert!(surface_report.contains("KERNEL_TASK"));
    assert!(surface_report.contains("TERM"));

    // Corruption layer
    entity.update_depth(10);
    let corruption_report = ProcessMonitor::generate_process_list(&entity, seed);
    assert!(corruption_report.contains("KERNEL_PANIC?"));
    assert!(corruption_report.contains("WATCHING"));

    // Presence layer
    entity.update_depth(20);
    let presence_report = ProcessMonitor::generate_process_list(&entity, seed);
    assert!(presence_report.contains("I_AM_INIT"));
    assert!(presence_report.contains("MEMORY_LEAK"));
    assert!(presence_report.contains("WHY_ARE_YOU_READING_THIS"));

    // Infection layer
    entity.update_depth(30);
    let infection_report = ProcessMonitor::generate_process_list(&entity, seed);
    assert!(
        infection_report.contains("WAITING")
            || infection_report.contains("STARVING")
            || infection_report.contains("BLEEDING")
            || infection_report.contains("DIGESTING")
            || infection_report.contains("CONSUMING_CYCLES")
            || infection_report.contains("FORGETTING_HOW_TO_STOP")
            || infection_report.contains("ECHOING")
            || infection_report.contains("SCREAMING_INTO_DEV_NULL")
    );
}

#[test]
fn test_sentiment_analyzer_empty() {
    let entity = Entity::new();
    let text = "";
    let out = SentimentAnalyzer::analyze(text, &entity, 42);
    assert_eq!(out, "?FILE IS EMPTY\n");

    let text_spaces = "   \n  \t  ";
    let out_spaces = SentimentAnalyzer::analyze(text_spaces, &entity, 42);
    assert_eq!(out_spaces, "?FILE IS EMPTY\n");
}

#[test]
fn test_sentiment_analyzer_counts() {
    let entity = Entity::new();
    let text = "this is a test with words";
    let out = SentimentAnalyzer::analyze(text, &entity, 42);

    assert!(
        out.contains("WORD COUNT: 6"),
        "Failed to find correct word count. Output: {out}"
    );
    assert!(
        out.contains("AVG LENGTH: 3.33 CHARS"),
        "Failed to find correct average length. Output: {out}"
    );
}

#[test]
fn test_sentiment_analyzer_escalation() {
    let mut entity = Entity::new();
    let text = "this is a normal test file with some words.";

    // Surface layer
    let out_surface = SentimentAnalyzer::analyze(text, &entity, 42);
    assert!(out_surface.contains("--- PSYCHO-LINGUISTIC ANALYSIS ---"));
    assert!(out_surface.contains("SENTIMENT:"));

    // Set depth to Infection layer (26+)
    entity.update_depth(26);
    let out_infection = SentimentAnalyzer::analyze(text, &entity, 42);
    assert!(out_infection.contains("--- PSYCHO-LINGUISTIC ANALYSIS ---"));
    assert!(
        out_infection.contains("BLOOD")
            || out_infection.contains("BURNS")
            || out_infection.contains("WORDS")
            || out_infection.contains("HUNGER")
            || out_infection.contains("READING"),
        "Infection layer output should contain horror strings"
    );
}

#[test]
fn test_process_monitor_deterministic() {
    let mut entity = Entity::new();
    let seed = 42;

    entity.update_depth(10); // Corruption layer

    let report1 = ProcessMonitor::generate_process_list(&entity, seed);
    let report2 = ProcessMonitor::generate_process_list(&entity, seed);

    assert_eq!(report1, report2);
}

#[test]
fn test_hardware_sensors_escalation() {
    let mut entity = Entity::new();
    let seed = 42;

    // Surface layer
    entity.update_depth(0);
    let surface_report = HardwareSensors::get_readings(&entity, seed);
    assert!(surface_report.contains("CPU_TEMP"));
    assert!(surface_report.contains("SYS_FAN"));
    assert!(surface_report.contains("VOLTAGE"));

    // Presence layer
    entity.update_depth(20);
    let presence_report = HardwareSensors::get_readings(&entity, seed);
    assert!(presence_report.contains("ROOM_TEMP") || presence_report.contains("PROXIMITY"));

    // Infection layer
    entity.update_depth(30);
    let infection_report = HardwareSensors::get_readings(&entity, seed);
    assert!(infection_report.contains("HEARTBEAT"));
    assert!(infection_report.contains("EYE_CONTACT_SEC"));
}
