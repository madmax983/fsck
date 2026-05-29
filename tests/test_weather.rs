#![cfg(feature = "nova")]

use fsck::entity::Entity;
use fsck::experimental::WeatherService;

#[test]
fn test_weather_escalation() {
    let mut entity = Entity::new();
    let seed = 42;

    // Surface layer
    entity.update_depth(0);
    let surface_weather = WeatherService::get_forecast(&entity, seed);
    assert!(surface_weather.contains("CONDITIONS") || surface_weather.contains("TEMPERATURE"));

    // Infection layer
    entity.update_depth(30);
    let infection_weather = WeatherService::get_forecast(&entity, seed);
    assert!(
        infection_weather.contains("BLOOD")
            || infection_weather.contains("TEETH")
            || infection_weather.contains("FALLING")
            || infection_weather.contains("FLESH")
    );
}
