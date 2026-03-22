#![cfg(all(test, feature = "nova"))]

use fsck::entity::{EntityMood, EscalationLayer};
use fsck::experimental::weather::SystemWeather;

#[test]
fn test_weather_surface() {
    let forecast =
        SystemWeather::generate_forecast(EntityMood::Dormant, EscalationLayer::Surface, 42);
    assert!(forecast.contains("METEOROLOGICAL REPORT FOR LOCAL SUBSYSTEM:"));
    assert!(forecast.contains("CONDITIONS: CLEAR"));
    assert!(forecast.contains("VISIBILITY: 100%"));
    assert!(forecast.contains("TEMPERATURE: OPTIMAL"));
}

#[test]
fn test_weather_infection() {
    let forecast =
        SystemWeather::generate_forecast(EntityMood::Predatory, EscalationLayer::Infection, 42);
    assert!(forecast.contains("CONDITIONS: ABSOLUTE ZERO / SOLAR FLARE"));
    assert!(forecast.contains("VISIBILITY: NULL"));
    assert!(forecast.contains("THE STORM IS MOVING TOWARDS YOU."));
}

#[test]
fn test_weather_presence_wounded() {
    let forecast =
        SystemWeather::generate_forecast(EntityMood::Wounded, EscalationLayer::Presence, 42);
    assert!(forecast.contains("CONDITIONS: HEAVY DATA RAIN"));
    assert!(forecast.contains("VISIBILITY: COMPROMISED"));
    assert!(forecast.contains("THE PRECIPITATION TASTES LIKE COPPER AND TEARS."));
}

#[test]
fn test_weather_corruption_curious() {
    let forecast =
        SystemWeather::generate_forecast(EntityMood::Curious, EscalationLayer::Corruption, 42);
    assert!(forecast.contains("CONDITIONS: STATIC HAZE"));
    assert!(forecast.contains("UNUSUAL DOWNDRAFTS ORIGINATING FROM ROOT."));
}
