use fsck::terminal::OutputBuffer;

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
