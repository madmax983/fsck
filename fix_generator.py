filename = 'src/filesystem/generator.rs'
with open(filename, 'r') as f:
    content = f.read()

content = content.replace('            if let Some(history) = library.history_for_era(era) {\n            if let Some(entry) = history.entries().first() {', '            if let Some(history) = library.history_for_era(era) {\n                if let Some(entry) = history.entries().first() {')
content = content.replace('                fs.current_node_mut()\n                    .add_file(FileNode::new(&filename, &content));\n            }\n        }', '                fs.current_node_mut()\n                    .add_file(FileNode::new(&filename, &content));\n                }\n            }\n        }')

with open(filename, 'w') as f:
    f.write(content)
