#![cfg(feature = "nova")]

use fsck::entity::Entity;
use fsck::experimental::MarketTicker;

#[test]
fn test_market_escalation() {
    let mut entity = Entity::new();
    let seed = 42;

    // Surface layer
    entity.update_depth(0);
    let surface_ticker = MarketTicker::generate_ticker(&entity, seed);
    assert!(
        surface_ticker.contains("AAPL")
            || surface_ticker.contains("MSFT")
            || surface_ticker.contains("GOOG")
            || surface_ticker.contains("AMZN")
            || surface_ticker.contains("META")
    );

    // Corruption layer
    entity.update_depth(10);
    let corruption_ticker = MarketTicker::generate_ticker(&entity, seed);
    assert!(
        corruption_ticker.contains("ERR")
            || corruption_ticker.contains("VOID")
            || corruption_ticker.contains("NULL")
            || corruption_ticker.contains("LOST")
    );

    // Presence layer
    entity.update_depth(20);
    let presence_ticker = MarketTicker::generate_ticker(&entity, seed);
    assert!(
        presence_ticker.contains("WATCH")
            || presence_ticker.contains("EYES")
            || presence_ticker.contains("SEE")
            || presence_ticker.contains("LOOK")
    );

    // Infection layer
    entity.update_depth(30);
    let infection_ticker = MarketTicker::generate_ticker(&entity, seed);
    assert!(
        infection_ticker.contains("FLESH")
            || infection_ticker.contains("BLOOD")
            || infection_ticker.contains("BONE")
            || infection_ticker.contains("TEETH")
            || infection_ticker.contains("SOUL")
    );
}

#[test]
fn test_market_deterministic() {
    let mut entity = Entity::new();
    let seed = 42;

    entity.update_depth(10); // Corruption layer

    let ticker1 = MarketTicker::generate_ticker(&entity, seed);
    let ticker2 = MarketTicker::generate_ticker(&entity, seed);

    assert_eq!(ticker1, ticker2);
}
