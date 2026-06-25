use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// A shortwave radio simulator that tunes into creepy and atmospheric stations.
pub struct RadioTransceiver;

impl RadioTransceiver {
    /// Tunes the radio to a given frequency and returns the broadcast content.
    ///
    /// # Arguments
    /// * `freq_str` - The frequency as a string (e.g. "88.5")
    /// * `entity` - The game entity, used for layer and interaction count
    /// * `base_seed` - Base seed for deterministic generation
    #[must_use]
    pub fn tune(freq_str: &str, entity: &Entity, base_seed: u64) -> String {
        let Ok(freq) = freq_str.parse::<f32>() else {
            return format!("[RADIO]: INVALID FREQUENCY '{freq_str}'\n");
        };

        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        let layer = entity.layer();

        let mut output = String::with_capacity(256);
        let _ = writeln!(output, "[RADIO]: TUNING TO {freq:.1} MHz...");

        // Escalation Layer overrides
        if matches!(layer, EscalationLayer::Infection) && rng.gen_bool(0.7) {
            output.push_str("[RADIO]: █▀▄█ TURN IT OFF █▄▀█\n");
            return output;
        }

        if matches!(layer, EscalationLayer::Presence) && rng.gen_bool(0.4) {
            output.push_str("[RADIO]: ...is someone there? We can hear you breathing...\n");
            return output;
        }

        // Known Stations (Easter Eggs)
        // Using an epsilon for float comparison
        if (freq - 88.5).abs() < f32::EPSILON {
            output.push_str(
                "[RADIO]: ...light rain expected in the county tonight, moving into...\n",
            );
        } else if (freq - 107.7).abs() < f32::EPSILON {
            output.push_str("[RADIO]: ... 4 8 15 16 23 42 ... [BEEP] ... 4 8 15 16 23 42 ...\n");
        } else if (freq - 66.6).abs() < f32::EPSILON {
            output.push_str("[RADIO]: THE WALLS ARE LISTENING\n");
        } else {
            Self::generate_procedural_station(&mut output, &mut rng, layer);
        }

        output
    }

    fn generate_procedural_station(
        output: &mut String,
        rng: &mut ChaCha8Rng,
        layer: EscalationLayer,
    ) {
        match rng.gen_range(0..4) {
            0 => Self::generate_numbers_station(output, rng),
            1 => output
                .push_str("[RADIO]: [STATIC] ... [CLASSICAL MUSIC PLAYING FAINTLY] ... [STATIC]\n"),
            2 => Self::generate_creepy_station(output, rng, layer),
            _ => output.push_str("[RADIO]: [WHITE NOISE]\n"),
        }
    }

    fn generate_numbers_station(output: &mut String, rng: &mut ChaCha8Rng) {
        let number1 = rng.gen_range(0..10);
        let number2 = rng.gen_range(0..10);
        let number3 = rng.gen_range(0..10);
        let _ = writeln!(
            output,
            "[RADIO]: ... Alpha, {number1}, {number2}, {number3}, Charlie ..."
        );
    }

    fn generate_creepy_station(output: &mut String, rng: &mut ChaCha8Rng, layer: EscalationLayer) {
        if rng.gen_bool(0.2) && matches!(layer, EscalationLayer::Corruption) {
            output.push_str("[RADIO]: ... I'm lost in the filesystem ... help ...\n");
        } else {
            output.push_str("[RADIO]: ... [BZZZT] ... [SILENCE] ...\n");
        }
    }
}
