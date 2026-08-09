use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

pub struct TerminalMirror;

impl TerminalMirror {
    #[must_use]
    pub fn reflect(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let last_cmd = entity.commands_seen.last().map_or("", String::as_str);

        match entity.layer() {
            EscalationLayer::Surface => {
                format!("YOU SEE YOURSELF.\nYOUR LAST ACTION WAS: {last_cmd}\n")
            }
            EscalationLayer::Corruption => {
                let reversed: String = last_cmd.chars().rev().collect();
                format!("THE REFLECTION IS DISTORTED.\n:SAW NOITCA TSAL RUOY\n{reversed}\n")
            }
            EscalationLayer::Presence => {
                "THE MIRROR SHOWS SOMEONE STANDING BEHIND YOU.\nTHEY ARE SMILING.\n".to_string()
            }
            EscalationLayer::Infection => {
                if rng.gen_bool(0.5) {
                    "IT IS LOOKING AT YOU THROUGH THE GLASS.\nBREAK IT.\n".to_string()
                } else {
                    "YOU HAVE NO REFLECTION HERE.\n".to_string()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_surface_mirror() {
        let mut entity = Entity::new();
        entity.record_command("LOOK");
        let result = TerminalMirror::reflect(&entity, 42);
        assert!(result.contains("YOUR LAST ACTION WAS: LOOK"));
    }

    #[test]
    fn test_infection_mirror() {
        let mut entity = Entity::new();
        entity.add_depth(30);
        let result = TerminalMirror::reflect(&entity, 42);
        assert!(result.contains("IT IS LOOKING") || result.contains("NO REFLECTION"));
    }
}
