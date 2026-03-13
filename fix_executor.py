import re

with open('src/commands/executor.rs', 'r') as f:
    content = f.read()

# Replace CommandResult::success(&format!(...)) with CommandResult::success(format!(...))
content = re.sub(r'CommandResult::success\(&format!\((.*?)\)\)', r'CommandResult::success(format!(\1))', content, flags=re.DOTALL)
content = re.sub(r'CommandResult::error\(&format!\((.*?)\)\)', r'CommandResult::error(format!(\1))', content, flags=re.DOTALL)

# Replace CommandResult::success(&output) with CommandResult::success(output)
content = content.replace('CommandResult::success(&output)', 'CommandResult::success(output)')

# Replace CommandResult::success(&help_text) with CommandResult::success(help_text)
content = content.replace('CommandResult::success(&help_text)', 'CommandResult::success(help_text)')

# Replace CommandResult::success(&meta_response) with CommandResult::success(meta_response)
content = content.replace('CommandResult::success(&meta_response)', 'CommandResult::success(meta_response)')

# Replace CommandResult::success(&report) with CommandResult::success(report)
content = content.replace('CommandResult::success(&report)', 'CommandResult::success(report)')

# Replace CommandResult::error(&e) with CommandResult::error(e)
content = content.replace('CommandResult::error(&e)', 'CommandResult::error(e)')

with open('src/commands/executor.rs', 'w') as f:
    f.write(content)
