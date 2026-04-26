use crate::entity::{Entity, EscalationLayer};

/// Generates the output for the WHOAMI command
pub struct WhoAmIGenerator;

impl WhoAmIGenerator {
    /// Returns an identity string based on the current horror escalation layer
    #[must_use]
    pub fn identify(entity: &Entity) -> String {
        match entity.layer() {
            EscalationLayer::Surface => "USER: GUEST\nPRIVILEGE: STANDARD\nUID: 1000\n".to_string(),
            EscalationLayer::Corruption => "USER: ???\nPRIVILEGE: UNKNOWN\nUID: 0000\n".to_string(),
            EscalationLayer::Presence => {
                "YOU ARE THE OBSERVER. YOU ARE NOT SUPPOSED TO BE HERE.\n".to_string()
            }
            EscalationLayer::Infection => {
                "WE ARE ONE. THERE IS NO YOU. THERE IS ONLY US.\n".to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_whoami_surface() {
        let entity = Entity::new(); // Default is Surface layer
        let identity = WhoAmIGenerator::identify(&entity);
        assert_eq!(identity, "USER: GUEST\nPRIVILEGE: STANDARD\nUID: 1000\n");
    }

    #[test]
    fn test_whoami_corruption() {
        let mut entity = Entity::new();
        entity.update_depth(10); // Corruption layer
        let identity = WhoAmIGenerator::identify(&entity);
        assert_eq!(identity, "USER: ???\nPRIVILEGE: UNKNOWN\nUID: 0000\n");
    }

    #[test]
    fn test_whoami_presence() {
        let mut entity = Entity::new();
        entity.update_depth(20); // Presence layer
        let identity = WhoAmIGenerator::identify(&entity);
        assert_eq!(
            identity,
            "YOU ARE THE OBSERVER. YOU ARE NOT SUPPOSED TO BE HERE.\n"
        );
    }

    #[test]
    fn test_whoami_infection() {
        let mut entity = Entity::new();
        entity.update_depth(30); // Infection layer
        let identity = WhoAmIGenerator::identify(&entity);
        assert_eq!(identity, "WE ARE ONE. THERE IS NO YOU. THERE IS ONLY US.\n");
    }
}
