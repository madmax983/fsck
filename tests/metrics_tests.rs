#![cfg(feature = "nova")]

use fsck::entity::Entity;
use fsck::experimental::PrometheusExporter;

#[test]
fn test_metrics_surface() {
    let entity = Entity::new();
    let metrics = PrometheusExporter::export(&entity, 42);
    assert!(metrics.contains("system_uptime_seconds"));
    assert!(metrics.contains("cpu_temperature_celsius"));
    assert!(metrics.contains("memory_usage_bytes"));
    assert!(!metrics.contains("flesh_growth_rate"));
}

#[test]
fn test_metrics_infection() {
    let mut entity = Entity::new();
    entity.update_depth(30); // Infection
    let metrics = PrometheusExporter::export(&entity, 42);
    assert!(metrics.contains("system_uptime_seconds"));
    assert!(metrics.contains("cpu_temperature_celsius"));
    assert!(metrics.contains("flesh_growth_rate"));
    assert!(metrics.contains("screams_total"));
    assert!(metrics.contains("reality_integrity 0.0"));
}
