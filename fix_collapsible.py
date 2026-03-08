import re

with open("src/lib.rs", "r") as f:
    content = f.read()

content = content.replace(
    """        let mut prev_history = None;
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
        }""",
    """        let mut prev_history = None;
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
                    let entry = VictimEntry::new("2024-??-??", &format!("THEY TYPED: {joined_cmds}"));
                    history.add_entry(entry);
                    prev_history = Some(history);
                }
            }
        }"""
)

# Add allow for collapsible_if in lib.rs to avoid nested if warnings (unstable let chains)
content = content.replace(
    "#[wasm_bindgen]",
    "#[allow(clippy::collapsible_if)]\n#[wasm_bindgen]",
    1
)

with open("src/lib.rs", "w") as f:
    f.write(content)
