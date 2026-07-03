#![cfg(feature = "nova")]
use fsck::entity::{EntityMood, EscalationLayer};
use fsck::experimental::cctv::CctvViewer;

#[test]
fn test_cctv_surface_valid() {
    let out = CctvViewer::view_feed(42, 1, EscalationLayer::Surface, EntityMood::Dormant);
    assert!(
        out.contains("CAM-01"),
        "CCTV output should contain camera ID"
    );
    assert!(out.contains("Status: Normal"), "Surface should be normal");
}

#[test]
fn test_cctv_corruption_curious() {
    let out = CctvViewer::view_feed(42, 2, EscalationLayer::Corruption, EntityMood::Curious);
    assert!(out.contains("CAM-02"));
    assert!(out.contains("Status: "));
    assert!(out.contains("[Feed stabilized. A faint shape is visible in the background.]"));
}

#[test]
fn test_cctv_infection_glitching() {
    let out = CctvViewer::view_feed(100, 99, EscalationLayer::Infection, EntityMood::Glitching);
    assert!(out.contains("CAM-99"));
    assert!(out.contains("[FEED_OFFLINE. SYSTEM_OVERRIDE. I_AM_HERE.]"));
}
