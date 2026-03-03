filename = 'src/effects/metadata.rs'
with open(filename, 'r') as f:
    content = f.read()

content = content.replace('if let Some(pos) = chars.iter().position(char::is_ascii_digit) {\n            if let Some(digit) = char::from_digit(rng.r#gen_range(0..10), 10) {\n                    chars[pos] = digit;\n                }\n                chars.into_iter().collect()\n            }', 'if let Some(pos) = chars.iter().position(char::is_ascii_digit) {\n                    if let Some(digit) = char::from_digit(rng.r#gen_range(0..10), 10) {\n                        chars[pos] = digit;\n                    }\n                }\n                chars.into_iter().collect()\n            }')

with open(filename, 'w') as f:
    f.write(content)
