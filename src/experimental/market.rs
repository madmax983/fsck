use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Generates a stock market ticker that descends into madness as depth increases.
pub struct MarketTicker;

impl MarketTicker {
    /// Generates a ticker tape string based on the entity's current layer.
    #[must_use]
    pub fn generate_ticker(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();
        let mut output = String::new();

        let _ = writeln!(output, "FETCHING LATEST MARKET DATA...\n");
        output.push_str("SYM    PRICE      CHANGE\n");

        match layer {
            EscalationLayer::Surface => Self::generate_surface_ticker(&mut output, &mut rng),
            EscalationLayer::Corruption => Self::generate_corruption_ticker(&mut output, &mut rng),
            EscalationLayer::Presence => Self::generate_presence_ticker(&mut output, &mut rng),
            EscalationLayer::Infection => Self::generate_infection_ticker(&mut output, &mut rng),
        }

        output
    }

    fn generate_surface_ticker(output: &mut String, rng: &mut ChaCha8Rng) {
        let symbols = ["AAPL", "MSFT", "GOOG", "AMZN", "META"];
        for sym in symbols {
            let price = rng.gen_range(100.0..500.0);
            let change = rng.gen_range(-10.0..10.0);
            let sign = if change >= 0.0 { "+" } else { "" };
            let _ = writeln!(output, "{sym:<6} ${price:>6.2}    {sign}{change:>5.2}");
        }
    }

    fn generate_corruption_ticker(output: &mut String, rng: &mut ChaCha8Rng) {
        let symbols = ["AAPL", "ERR", "VOID", "NULL", "LOST"];
        for sym in symbols {
            let price = rng.gen_range(0.0..999.9);
            let change = rng.gen_range(-99.9..99.9);
            let sign = if change >= 0.0 { "+" } else { "" };
            let _ = writeln!(output, "{sym:<6} ${price:>6.2}    {sign}{change:>5.2}");
        }
    }

    fn generate_presence_ticker(output: &mut String, rng: &mut ChaCha8Rng) {
        let symbols = ["WATCH", "EYES", "SEE", "LOOK", "HERE"];
        for sym in symbols {
            let price = rng.gen_range(0.0..99.99);
            let change = rng.gen_range(-9.99..9.99);
            let sign = if change >= 0.0 { "+" } else { "" };
            let _ = writeln!(output, "{sym:<6} ${price:>6.2}    {sign}{change:>5.2}");
        }
    }

    fn generate_infection_ticker(output: &mut String, rng: &mut ChaCha8Rng) {
        let symbols = ["FLESH", "BLOOD", "BONE", "TEETH", "SOUL"];
        for sym in symbols {
            let price = rng.gen_range(666.0..666.99);
            let change = rng.gen_range(-66.6..66.6);
            let sign = if change >= 0.0 { "+" } else { "" };
            let _ = writeln!(output, "{sym:<6} ${price:>6.2}    {sign}{change:>5.2}");
        }
    }
}
