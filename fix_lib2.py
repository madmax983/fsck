import re

with open("src/lib.rs", "r") as f:
    content = f.read()

content = content.replace(
    "use commands::{Command, CommandExecutor};",
    "use commands::{Command, CommandExecutor};\nuse content::{Era, VictimEntry, VictimHistory};"
)

content = content.replace(
    "let fs = FilesystemGenerator::generate(seed, 5);",
    """let mut prev_history = None;
        if let Ok(Some(json)) = GameStorage::load(StorageKey::PlayerHistory) {
            if let Ok(commands) = serde_json::from_str::<Vec<String>>(&json) {
                if !commands.is_empty() {
                    let mut history = VictimHistory::new(Era::Current, "THE LAST ONE", 2024);
                    let last_cmds = if commands.len() > 10 {
                        &commands[commands.len() - 10..]
                    } else {
                        &commands[..]
                    };
                    let joined_cmds = last_cmds.join(", ");
                    let entry = VictimEntry::new("2024-??-??", &format!("THEY TYPED: {}", joined_cmds));
                    history.add_entry(entry);
                    prev_history = Some(history);
                }
            }
        }

        let fs = FilesystemGenerator::generate(seed, 5, prev_history);"""
)

with open("src/lib.rs", "w") as f:
    f.write(content)
