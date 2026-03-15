#![cfg(all(test, feature = "nova"))]

use fsck::entity::Entity;
use fsck::experimental::PingTool;

#[test]
fn test_ping_tool_escalation() {
    let mut entity = Entity::new();
    let seed = 42;
    let target = "127.0.0.1";

    // Surface layer
    entity.update_depth(0);
    let surface_report = PingTool::run_ping(target, &entity, seed);
    assert!(surface_report.contains("PING 127.0.0.1: 56 data bytes"));
    assert!(surface_report.contains("bytes from 127.0.0.1: icmp_seq="));
    assert!(surface_report.contains("0% packet loss"));

    // Corruption layer
    entity.update_depth(10);
    let corruption_report = PingTool::run_ping(target, &entity, seed);
    assert!(
        corruption_report.contains("Destination Host Unreachable")
            || corruption_report.contains("packet loss")
            || corruption_report.contains("Request timeout")
    );

    // Presence layer
    entity.update_depth(20);
    let presence_report = PingTool::run_ping(target, &entity, seed);
    assert!(
        presence_report.contains("I AM THE NETWORK")
            || presence_report.contains("THERE IS NO OTHER HOST")
            || presence_report.contains("100% soul loss")
            || presence_report.contains("Destination Host Is Watching")
    );

    // Infection layer
    entity.update_depth(30);
    let infection_report = PingTool::run_ping(target, &entity, seed);
    assert!(
        infection_report.contains("WHY ARE YOU KNOCKING")
            || infection_report.contains("NO ONE IS HOME BUT ME")
            || infection_report.contains("PACKETS SWALLOWED BY THE DARK")
            || infection_report.contains("BLOOD IN THE SOCKET")
            || infection_report.contains("THE CONNECTION IS SEVERED")
    );
}

#[test]
fn test_ping_tool_deterministic() {
    let mut entity = Entity::new();
    let seed = 42;
    let target = "192.168.1.1";

    entity.update_depth(10); // Corruption layer

    let report1 = PingTool::run_ping(target, &entity, seed);
    let report2 = PingTool::run_ping(target, &entity, seed);

    assert_eq!(report1, report2);
}
