/// Buffer for accumulating terminal output before sending to JS
#[derive(Default)]
pub struct OutputBuffer {
    content: String,
}

impl OutputBuffer {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn write(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn drain(&mut self) -> String {
        std::mem::take(&mut self.content)
    }
}
