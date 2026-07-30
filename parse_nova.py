import re

with open("src/commands/executor.rs", "r") as f:
    content = f.read()

# find handle_nova_commands function using regex to extract its body
match = re.search(r'fn handle_nova_commands\(&self, cmd: &str\) -> Option<CommandResult> \{(.*?)\n    \}', content, re.DOTALL)
if match:
    body = match.group(1)
    # find all else if blocks
    blocks = re.findall(r'else if (.*?) \{\n(.*?)        \}', body, re.DOTALL)
    for condition, block in blocks:
        print("Condition:", condition.strip())
        print("Block:", block.strip())
        print("-" * 20)
