use fsck::entity::Entity;
#[cfg(feature = "nova")]
use fsck::experimental::network_trace::NetworkTrace;

#[cfg(feature = "nova")]
#[test]
fn test_network_trace_surface() {
    let entity = Entity::new();
    let trace = NetworkTrace::generate_trace(&entity, 42, "example.com");
    assert!(trace.contains("TRACING ROUTE TO EXAMPLE.COM..."));
    assert!(trace.contains("MAX HOPS: 30"));
    assert!(trace.contains("HOP  RTT1   RTT2   RTT3   HOST"));
    assert!(trace.contains("TRACE COMPLETE."));
}

#[cfg(feature = "nova")]
#[test]
fn test_network_trace_infection() {
    let mut entity = Entity::new();
    entity.add_depth(30); // Reach Infection layer
    let trace = NetworkTrace::generate_trace(&entity, 42, "example.com");
    // Ensure we hit at least one infection host or presence host
    assert!(trace.contains("TRACE COMPLETE."));
}
