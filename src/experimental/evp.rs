use crate::content::ContentLibrary;
use crate::effects::{CorruptionEffect, CorruptionIntensity};
use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Electronic Voice Phenomenon (EVP) scanner.
/// Captures fleeting, corrupted fragments of past victims' logs.
pub struct EvpScanner;

impl EvpScanner {
    /// Scans for EVP fragments, pulling from the `ContentLibrary` and corrupting them based on depth.
    #[must_use]
    pub fn scan(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::new();
        let _ = writeln!(output, "[EVP SCANNER ACTIVE... TUNING TO RESIDUAL ECHOES]");

        let library = ContentLibrary::new();
        let histories = library.all_histories();

        if histories.is_empty() {
            let _ = writeln!(output, "[NO SIGNAL. THE VOID IS EMPTY.]");
            return output;
        }

        let history = &histories[rng.gen_range(0..histories.len())];
        let entries = history.entries();

        if entries.is_empty() {
            let _ = writeln!(output, "[NO SIGNAL. SILENCE REIGNS.]");
            return output;
        }

        let entry = &entries[rng.gen_range(0..entries.len())];

        let intensity = match layer {
            EscalationLayer::Surface => CorruptionIntensity::None,
            EscalationLayer::Corruption => CorruptionIntensity::Mild,
            EscalationLayer::Presence => CorruptionIntensity::Moderate,
            EscalationLayer::Infection => CorruptionIntensity::Severe,
        };

        let effect = CorruptionEffect::new(intensity);
        let corrupted_content = effect.apply(entry.content(), interaction_seed);

        let _ = writeln!(output, "\n--- INTERCEPTED FRAGMENT ---");
        let _ = writeln!(output, "SUBJECT: {}", history.name());
        let _ = writeln!(output, "YEAR: {}", history.year());
        let _ = writeln!(output, "DATE: {}", entry.date());
        let _ = writeln!(output, "\n{corrupted_content}");
        let _ = writeln!(output, "----------------------------");

        // If deep enough, add a direct threat/glitch
        if matches!(layer, EscalationLayer::Infection) && rng.gen_bool(0.5) {
            let _ = writeln!(output, "\n[EVP WARNING: THEY ARE LISTENING TO YOU TOO]");
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_evp_scan_surface() {
        let entity = Entity::new();
        let result = EvpScanner::scan(&entity, 123);
        assert!(result.contains("[EVP SCANNER ACTIVE"));
        assert!(result.contains("SUBJECT:"));
        assert!(result.contains("YEAR:"));
    }

    #[test]
    fn test_evp_scan_infection() {
        let mut entity = Entity::new();
        entity.update_depth(30); // Infection layer
        let result = EvpScanner::scan(&entity, 123);
        assert!(result.contains("[EVP SCANNER ACTIVE"));
    }
}
