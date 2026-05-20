use crate::entity::{Entity, EscalationLayer};

/// Simulates accessing external security cameras, degrading based on the entity's escalation layer.
pub struct CctvViewer;

impl CctvViewer {
    /// Generates the CCTV feed output based on the entity's current layer.
    #[must_use]
    pub fn view(entity: &Entity) -> String {
        match entity.layer() {
            EscalationLayer::Surface => {
                "CAMERA 1: SECURE\nCAMERA 2: SECURE\nCAMERA 3: SECURE\n".to_string()
            }
            EscalationLayer::Corruption => {
                "CAMERA 1: SECURE\nCAMERA 2: [SIGNAL LOST]\nCAMERA 3: STATIC\n".to_string()
            }
            EscalationLayer::Presence => {
                "CAMERA 1: [MOTION DETECTED - CORRIDOR B]\nCAMERA 2: THEY ARE LOOKING AT YOU\nCAMERA 3: OFFLINE\n".to_string()
            }
            EscalationLayer::Infection => {
                "CAMERA 1: [FEED REPLACED WITH YOUR WEBCAM]\nCAMERA 2: I SEE YOU\nCAMERA 3: DON'T BLINK\n".to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cctv_surface() {
        let mut entity = Entity::new();
        entity.update_depth(0);
        let out = CctvViewer::view(&entity);
        assert_eq!(
            out,
            "CAMERA 1: SECURE\nCAMERA 2: SECURE\nCAMERA 3: SECURE\n"
        );
    }

    #[test]
    fn test_cctv_corruption() {
        let mut entity = Entity::new();
        entity.update_depth(10);
        let out = CctvViewer::view(&entity);
        assert_eq!(
            out,
            "CAMERA 1: SECURE\nCAMERA 2: [SIGNAL LOST]\nCAMERA 3: STATIC\n"
        );
    }

    #[test]
    fn test_cctv_presence() {
        let mut entity = Entity::new();
        entity.update_depth(20);
        let out = CctvViewer::view(&entity);
        assert_eq!(
            out,
            "CAMERA 1: [MOTION DETECTED - CORRIDOR B]\nCAMERA 2: THEY ARE LOOKING AT YOU\nCAMERA 3: OFFLINE\n"
        );
    }

    #[test]
    fn test_cctv_infection() {
        let mut entity = Entity::new();
        entity.update_depth(30);
        let out = CctvViewer::view(&entity);
        assert_eq!(
            out,
            "CAMERA 1: [FEED REPLACED WITH YOUR WEBCAM]\nCAMERA 2: I SEE YOU\nCAMERA 3: DON'T BLINK\n"
        );
    }
}
