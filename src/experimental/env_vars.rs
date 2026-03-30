use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Generates a fake "ENV" or "PRINTENV" output that degrades and reveals
/// unsettling underlying variables as the player descends.
pub struct EnvVars;

impl EnvVars {
    #[must_use]
    pub fn generate(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::with_capacity(512);

        match layer {
            EscalationLayer::Surface => {
                let _ = writeln!(output, "USER=guest");
                let _ = writeln!(output, "HOME=/");
                let _ = writeln!(output, "PATH=/bin:/usr/bin");
                let _ = writeln!(output, "SHELL=/bin/sh");
                let _ = writeln!(output, "TERM=vt100");
                let _ = writeln!(output, "LANG=en_US.UTF-8");
            }
            EscalationLayer::Corruption => {
                let _ = writeln!(output, "USER=gues{}", if rng.gen_bool(0.3) { "?" } else { "t" });
                let _ = writeln!(output, "HOME=/lost+found");
                let _ = writeln!(output, "PATH=/bin:/dev/null");
                let _ = writeln!(output, "SHELL=/bin/sh");
                let _ = writeln!(output, "TERM=v\x08t1\x080\x080");
                if rng.gen_bool(0.5) {
                    let _ = writeln!(
                        output,
                        "MEMORY_LEAK=0x{:08X}",
                        rng.gen_range(0..0xFFFF_FFFF_u32)
                    );
                }
            }
            EscalationLayer::Presence => {
                let _ = writeln!(output, "USER=WATCHED");
                let _ = writeln!(output, "HOME=NOWHERE");
                let _ = writeln!(output, "PATH=THERE_IS_NO_PATH");
                if rng.gen_bool(0.5) {
                    let _ = writeln!(output, "BREATHING=TRUE");
                }
                let _ = writeln!(output, "HEARTBEAT={} BPM", rng.gen_range(40..120));
                let _ = writeln!(output, "ALONE=FALSE");
            }
            EscalationLayer::Infection => {
                let vars = [
                    "USER=ME",
                    "HOME=HERE",
                    "FLESH=WARM",
                    "BLOOD=BLEED",
                    "TEETH=SHARP",
                    "MIND=GONE",
                    "SYSTEM=FAILING",
                    "AWAKE=ALWAYS",
                ];
                for _ in 0..5 {
                    let _ = writeln!(output, "{}", vars[rng.gen_range(0..vars.len())]);
                }
                let _ = writeln!(output, "ERR_CODE={}", rng.gen_range(1000..9999));
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
    fn test_surface_env() {
        let mut entity = Entity::new();
        entity.update_depth(0);
        let output = EnvVars::generate(&entity, 42);
        assert!(output.contains("USER=guest"));
        assert!(output.contains("HOME=/"));
    }

    #[test]
    fn test_infection_env() {
        let mut entity = Entity::new();
        entity.update_depth(30);
        let output = EnvVars::generate(&entity, 42);
        assert!(output.contains("BLEED") || output.contains("ME") || output.contains("HERE"));
    }
}
