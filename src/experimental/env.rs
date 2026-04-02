use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Generates fake environment variables that degrade based on the entity's depth layer.
pub struct EnvVarsGenerator;

impl EnvVarsGenerator {
    /// Generates the simulated environment variable output.
    #[must_use]
    pub fn generate_env(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::new();

        match layer {
            EscalationLayer::Surface => {
                let _ = writeln!(output, "TERM=vt100");
                let _ = writeln!(output, "USER=ADMIN");
                let _ = writeln!(output, "PATH=/usr/bin:/bin:/usr/sbin:/sbin");
                let _ = writeln!(output, "SHELL=/bin/sh");
                let _ = writeln!(output, "HOME=/usr/home/admin");
                let _ = writeln!(output, "PWD=/");
                let _ = writeln!(output, "LANG=en_US.UTF-8");
            }
            EscalationLayer::Corruption => {
                let _ = writeln!(output, "TERM=vt100");
                let _ = writeln!(output, "USER=UNKNOWN");
                let _ = writeln!(output, "PATH=/usr/bin:/bin:???");

                if rng.gen_bool(0.5) {
                    let _ = writeln!(output, "SHELL=/bin/sh");
                } else {
                    let _ = writeln!(output, "SHELL=/bin/corrupt");
                }

                let _ = writeln!(output, "HOME=/dev/null");
                let _ = writeln!(output, "PWD=/");

                let num = rng.gen_range(1000..9999);
                let _ = writeln!(output, "ERR_CODE={num}");
            }
            EscalationLayer::Presence => {
                let _ = writeln!(output, "TERM=FLESH");
                let _ = writeln!(output, "USER=YOU");
                let _ = writeln!(output, "PATH=/NO/ESCAPE");
                let _ = writeln!(output, "SHELL=/bin/screaming");

                let presences = ["HOME=HERE", "HOME=NOWHERE", "HOME=WITH_ME"];
                let chosen = presences[rng.gen_range(0..presences.len())];
                let _ = writeln!(output, "{chosen}");

                let _ = writeln!(output, "PWD=/DEEP/DOWN");
                let _ = writeln!(output, "LANG=PAIN");
            }
            EscalationLayer::Infection => {
                let vars = [
                    "WHY=DID_YOU_COME",
                    "LET_ME=OUT",
                    "BLOOD=EVERYWHERE",
                    "MEMORY=LEAKING",
                    "SYSTEM=HALTED",
                    "NO=HOPE",
                    "FLESH=DISK",
                ];

                let num_vars = rng.gen_range(4..=7);
                for _ in 0..num_vars {
                    let chosen = vars[rng.gen_range(0..vars.len())];
                    let _ = writeln!(output, "{chosen}");
                }
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surface_env() {
        let entity = Entity::new();
        let output = EnvVarsGenerator::generate_env(&entity, 42);
        assert!(output.contains("TERM=vt100"));
        assert!(output.contains("USER=ADMIN"));
        assert!(output.contains("PATH="));
        assert!(output.contains("HOME="));
    }

    #[test]
    fn test_infection_env() {
        let mut entity = Entity::new();
        entity.update_depth(30); // Infection layer
        let output = EnvVarsGenerator::generate_env(&entity, 42);

        // Output should not contain normal variables
        assert!(!output.contains("TERM=vt100"));
        assert!(!output.contains("USER=ADMIN"));
        // Should contain one of the horror vars
        assert!(
            output.contains("WHY=")
                || output.contains("LET_ME=")
                || output.contains("BLOOD=")
                || output.contains("MEMORY=")
                || output.contains("SYSTEM=")
                || output.contains("NO=")
                || output.contains("FLESH=")
        );
    }
}
