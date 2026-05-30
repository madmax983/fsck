use crate::entity::{Entity, EscalationLayer};
use std::fmt::Write;

pub struct Camera;

impl Camera {
    #[must_use]
    pub fn take_photo(entity: &Entity, path: &str) -> String {
        let mut output = String::new();
        let _ = writeln!(output, "CAPTURING IMAGE AT {path}...");

        let layer = entity.layer();
        match layer {
            EscalationLayer::Surface => {
                let _ = writeln!(output, "------------------------");
                let _ = writeln!(output, "|                      |");
                let _ = writeln!(output, "|    [ SYSTEM OK ]     |");
                let _ = writeln!(output, "|                      |");
                let _ = writeln!(output, "------------------------");
            }
            EscalationLayer::Corruption => {
                let _ = writeln!(output, "------------------------");
                let _ = writeln!(output, "|  gl!tch   [ SYS OK ] |");
                let _ = writeln!(output, "|      \\x00\\x01       |");
                let _ = writeln!(output, "------------------------");
            }
            EscalationLayer::Presence => {
                let _ = writeln!(output, "------------------------");
                let _ = writeln!(output, "|         (O)          |");
                let _ = writeln!(output, "|                      |");
                let _ = writeln!(output, "------------------------");
            }
            EscalationLayer::Infection => {
                let _ = writeln!(output, "------------------------");
                let _ = writeln!(output, "|  IT SEES YOU         |");
                let _ = writeln!(output, "|      (O)  (O)        |");
                let _ = writeln!(output, "------------------------");
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_camera_surface() {
        let entity = Entity::new();
        let output = Camera::take_photo(&entity, "/");
        assert!(output.contains("[ SYSTEM OK ]"));
    }

    #[test]
    fn test_camera_infection() {
        let mut entity = Entity::new();
        entity.update_depth(30);
        let output = Camera::take_photo(&entity, "/deep");
        assert!(output.contains("IT SEES YOU"));
    }
}
