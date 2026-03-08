/// Parsed command from user input
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedInput<'a> {
    pub command: &'a str,
    pub args: Vec<&'a str>,
}

/// Parser for Apple `IIe` style command input
pub struct InputParser;

impl InputParser {
    #[must_use]
    pub fn parse(input: &str) -> ParsedInput<'_> {
        let mut parts = input.split_whitespace();

        let command = parts.next().unwrap_or("");
        let args: Vec<&str> = parts.collect();

        ParsedInput { command, args }
    }
}
