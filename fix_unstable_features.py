import re

with open("src/lib.rs", "r") as f:
    content = f.read()

# Replace the nested ifs with match/allow to satisfy clippy without using unstable let_chains
replacement = """        let mut prev_history = None;
        #[allow(clippy::collapsible_if)]
        if let Ok(Some(json)) = GameStorage::load(StorageKey::PlayerHistory) {
            #[allow(clippy::collapsible_if)]
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

content = re.sub(r"        let mut prev_history = None;\n        if let Ok\(Some\(json\)\) = GameStorage::load\(StorageKey::PlayerHistory\) \{\n            if let Ok\(commands\) = serde_json::from_str::<Vec<String>>\(&json\) \{\n                if !commands.is_empty\(\) \{\n                    let mut history = VictimHistory::new\(Era::Current, \"THE LAST ONE\", 2024\);\n                    let last_cmds = if commands.len\(\) > 10 \{\n                        &commands\[commands.len\(\) - 10..\]\n                    \} else \{\n                        &commands\[..\]\n                    \};\n                    let joined_cmds = last_cmds.join\(\", \"\);\n                    let entry = VictimEntry::new\(\"2024-\?\?-\?\?\", &format\!\(\"THEY TYPED: \{joined_cmds\}\"\)\);\n                    history.add_entry\(entry\);\n                    prev_history = Some\(history\);\n                \}\n            \}\n        \}", replacement, content)

with open("src/lib.rs", "w") as f:
    f.write(content)
