import re

with open("src/persistence/state.rs", "r") as f:
    content = f.read()

content = content.replace(
    "    command_history: Vec<String>,",
    "    #[serde(default)]\n    command_history: Vec<String>,"
)

content = content.replace(
    """    pub fn record_command(&mut self, cmd: &str) {
        self.command_history.push(cmd.to_string());
    }""",
    """    pub fn record_command(&mut self, cmd: &str) {
        self.command_history.push(cmd.to_string());
        if self.command_history.len() > 100 {
            self.command_history.remove(0);
        }
    }"""
)

with open("src/persistence/state.rs", "w") as f:
    f.write(content)
