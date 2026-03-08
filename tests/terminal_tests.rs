use fsck::terminal::{InputParser, OutputBuffer};

#[test]
fn test_output_buffer_captures_text() {
    let mut buffer = OutputBuffer::new();
    buffer.write("HELLO");
    assert_eq!(buffer.drain(), "HELLO");
}

#[test]
fn test_output_buffer_accumulates() {
    let mut buffer = OutputBuffer::new();
    buffer.write("LINE 1\n");
    buffer.write("LINE 2");
    assert_eq!(buffer.drain(), "LINE 1\nLINE 2");
}

#[test]
fn test_drain_clears_buffer() {
    let mut buffer = OutputBuffer::new();
    buffer.write("TEXT");
    let _ = buffer.drain();
    assert_eq!(buffer.drain(), "");
}

#[test]
fn test_parse_simple_command() {
    let result = InputParser::parse("CATALOG");
    assert_eq!(result.command, "CATALOG");
    assert!(result.args.is_empty());
}

#[test]
fn test_parse_command_with_args() {
    let result = InputParser::parse("CD GAMES");
    assert_eq!(result.command, "CD");
    assert_eq!(result.args, vec!["GAMES"]);
}

#[test]
fn test_parse_normalizes_to_uppercase() {
    // It no longer normalizes to uppercase here, we do it when evaluating the command
    let result = InputParser::parse("catalog");
    assert_eq!(result.command, "catalog");
}

#[test]
fn test_parse_empty_input() {
    let result = InputParser::parse("");
    assert_eq!(result.command, "");
    assert!(result.args.is_empty());
}

#[test]
fn test_parse_trims_whitespace() {
    let result = InputParser::parse("  CD   GAMES  ");
    assert_eq!(result.command, "CD");
    assert_eq!(result.args, vec!["GAMES"]);
}
