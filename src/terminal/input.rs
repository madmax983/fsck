/// Parsed command from user input
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedInput {
    pub command: String,
    pub args: Vec<String>,
}

/// Parser for Apple `IIe` style command input
pub struct InputParser;

impl InputParser {
    pub fn parse(input: &str) -> ParsedInput {
        let normalized = input.trim().to_uppercase();
        let mut parts = normalized.split_whitespace();

        let command = parts.next().unwrap_or("").to_string();
        let args: Vec<String> = parts.map(String::from).collect();

        ParsedInput { command, args }
    }
}
