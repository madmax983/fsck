#![cfg(feature = "nova")]

use fsck::entity::Entity;

#[cfg(feature = "nova")]
use fsck::experimental::{ProcessMonitor, SpatialAudioGenerator, SystemDiagnostics};

#[cfg(feature = "nova")]
#[cfg(feature = "nova")]
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

#[cfg(feature = "nova")]
#[cfg(feature = "nova")]
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

#[cfg(feature = "nova")]
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

#[cfg(feature = "nova")]
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

#[cfg(feature = "nova")]
#[test]
fn test_process_monitor_deterministic() {
    let mut entity = Entity::new();
    let seed = 42;

    entity.update_depth(10); // Corruption layer

    let report1 = ProcessMonitor::generate_process_list(&entity, seed);
    let report2 = ProcessMonitor::generate_process_list(&entity, seed);

    assert_eq!(report1, report2);
}

#[cfg(feature = "nova")]
#[test]
fn test_network_simulator_escalation() {
    let mut entity = Entity::new();
    let seed = 42;

    // Surface layer
    entity.update_depth(0);
    let surface_netstat = fsck::experimental::NetworkSimulator::generate_netstat(&entity, seed);
    assert!(surface_netstat.contains("Local"));
    assert!(surface_netstat.contains("ESTABLISHED"));
    assert!(surface_netstat.contains("127.0.0.1"));

    let surface_ping = fsck::experimental::NetworkSimulator::ping("127.0.0.1", &entity, seed);
    assert!(surface_ping.contains("PING 127.0.0.1: 56 data bytes"));
    assert!(surface_ping.contains("time="));

    // Corruption layer
    entity.update_depth(10);
    let corruption_netstat = fsck::experimental::NetworkSimulator::generate_netstat(&entity, seed);
    assert!(corruption_netstat.contains("127.0.0.1"));
    assert!(corruption_netstat.contains("UNKNOWN") || corruption_netstat.contains("DROPPED"));

    let corruption_ping = fsck::experimental::NetworkSimulator::ping("localhost", &entity, seed);
    assert!(corruption_ping.contains("Packet loss") || corruption_ping.contains("timeout"));

    // Presence layer
    entity.update_depth(20);
    let presence_netstat = fsck::experimental::NetworkSimulator::generate_netstat(&entity, seed);
    assert!(
        presence_netstat.contains("LISTENING_TO_YOU")
            || presence_netstat.contains("INSIDE_THE_HOUSE")
    );

    let presence_ping = fsck::experimental::NetworkSimulator::ping("hello", &entity, seed);
    assert!(presence_ping.contains("I AM HERE") || presence_ping.contains("time=9999"));

    // Infection layer
    entity.update_depth(30);
    let infection_netstat = fsck::experimental::NetworkSimulator::generate_netstat(&entity, seed);
    assert!(
        infection_netstat.contains("NO_WAY_OUT")
            || infection_netstat.contains("BEYOND_REACH")
            || infection_netstat.contains("PORT_666")
    );

    let infection_ping = fsck::experimental::NetworkSimulator::ping("anyone", &entity, seed);
    assert!(
        infection_ping.contains("ALONE")
            || infection_ping.contains("SILENCE")
            || infection_ping.contains("THEY_ARE_GONE")
    );
}
