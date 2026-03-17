/// Parsed command from user input
#[derive(Debug, Clone)]
pub struct ParsedInput<'a> {
    pub command: &'a str,
    /// ⚡ Bolt Optimization: Uses `SplitWhitespace` iterator instead of allocating a `Vec<&str>`
    /// for command arguments, eliminating a heap allocation on every parsed user input.
    pub args: core::str::SplitWhitespace<'a>,
}

/// Parser for Apple `IIe` style command input
pub struct InputParser;

impl InputParser {
    #[must_use]
    pub fn parse(input: &str) -> ParsedInput<'_> {
        let mut parts = input.split_whitespace();

        let command = parts.next().unwrap_or("");

        ParsedInput {
            command,
            args: parts,
        }
    }
}
