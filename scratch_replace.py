import re

with open("src/effects/corruption.rs", "r") as f:
    content = f.read()

old_code = """    pub fn apply(&self, text: &str, seed: u64) -> String {
        if matches!(self.intensity, CorruptionIntensity::None) {
            return text.to_string();
        }

        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let rate = self.intensity.corruption_rate();
        let mut result = String::with_capacity(text.len());

        for ch in text.chars() {
            if ch.is_whitespace() {
                result.push(ch);
            } else if rng.r#gen::<f32>() < rate {
                result.push(CORRUPTION_CHARS[rng.gen_range(0..CORRUPTION_CHARS.len())]);
            } else {
                result.push(ch);
            }
        }

        result
    }"""

new_code = """    pub fn apply<'a>(&self, text: &'a str, seed: u64) -> std::borrow::Cow<'a, str> {
        if matches!(self.intensity, CorruptionIntensity::None) {
            return std::borrow::Cow::Borrowed(text);
        }

        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let rate = self.intensity.corruption_rate();
        // Calculate max possible capacity. 3 is the max byte size of CORRUPTION_CHARS.
        let mut result = String::with_capacity(text.len() * 3);

        for ch in text.chars() {
            if ch.is_whitespace() {
                result.push(ch);
            } else if rng.r#gen::<f32>() < rate {
                result.push(CORRUPTION_CHARS[rng.gen_range(0..CORRUPTION_CHARS.len())]);
            } else {
                result.push(ch);
            }
        }

        std::borrow::Cow::Owned(result)
    }"""

if old_code in content:
    with open("src/effects/corruption.rs", "w") as f:
        f.write(content.replace(old_code, new_code))
    print("Replaced successfully")
else:
    print("Old code not found")
