use fsck::effects::{CorruptionEffect, CorruptionIntensity, InterferenceEffect, InterferenceType};

#[test]
fn test_no_corruption_at_surface() {
    let effect = CorruptionEffect::new(CorruptionIntensity::None);
    let input = "HELLO WORLD";
    let output = effect.apply(input, 12345);
    assert_eq!(input, output);
}

#[test]
fn test_mild_corruption_preserves_length() {
    let effect = CorruptionEffect::new(CorruptionIntensity::Mild);
    let input = "HELLO WORLD";
    let output = effect.apply(input, 12345);
    assert_eq!(input.len(), output.len());
}

#[test]
fn test_severe_corruption_changes_text() {
    let effect = CorruptionEffect::new(CorruptionIntensity::Severe);
    let input = "HELLO WORLD";
    let output = effect.apply(input, 12345);
    assert_ne!(input, output);
}

#[test]
fn test_same_seed_produces_same_corruption() {
    let effect = CorruptionEffect::new(CorruptionIntensity::Moderate);
    let input = "HELLO WORLD";
    let out1 = effect.apply(input, 999);
    let out2 = effect.apply(input, 999);
    assert_eq!(out1, out2);
}

#[test]
fn test_intensity_from_depth() {
    assert!(matches!(
        CorruptionIntensity::from_depth(5),
        CorruptionIntensity::None
    ));
    assert!(matches!(
        CorruptionIntensity::from_depth(15),
        CorruptionIntensity::Mild
    ));
    assert!(matches!(
        CorruptionIntensity::from_depth(45),
        CorruptionIntensity::Severe
    ));
}

#[test]
fn test_echo_duplication() {
    let effect = InterferenceEffect::new(InterferenceType::Echo);
    let output = effect.apply("HELLO");
    assert!(output.contains("HELLO"));
    assert!(output.len() > "HELLO".len()); // Should have duplication
}

#[test]
fn test_cursor_jump() {
    let effect = InterferenceEffect::new(InterferenceType::CursorJump);
    let sequence = effect.generate_sequence();
    // Should contain ANSI escape codes for cursor movement
    assert!(sequence.contains("\x1B"));
}

#[test]
fn test_line_noise() {
    let effect = InterferenceEffect::new(InterferenceType::LineNoise);
    let output = effect.apply("NORMAL TEXT");
    // Should add visual noise
    assert!(output.len() >= "NORMAL TEXT".len());
}
