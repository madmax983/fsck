with open('src/commands/types.rs', 'r') as f:
    content = f.read()

content = content.replace('pub fn success(output: &str) -> Self {', 'pub fn success(output: impl Into<String>) -> Self {')
content = content.replace('output: output.to_string(),', 'output: output.into(),')

content = content.replace('pub fn error(message: &str) -> Self {', 'pub fn error(message: impl Into<String>) -> Self {')
content = content.replace('output: message.to_string(),', 'output: message.into(),')

with open('src/commands/types.rs', 'w') as f:
    f.write(content)
