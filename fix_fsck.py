filename = 'tests/fsck_tests.rs'
with open(filename, 'r') as f:
    content = f.read()

content = content.replace('        if let Some(d2) = dirs.first() {\n            if fs.change_dir(d2).is_ok() {\n            // At depth 2+, try revealing hidden content\n            let revealed = fs.reveal_hidden_in_current();\n            // May or may not have hidden content (25% chance at depth 2)\n            // Just verify the mechanism works without panicking\n            let _ = revealed;\n        }\n    }\n}', '        if let Some(d2) = dirs.first() {\n            if fs.change_dir(d2).is_ok() {\n                // At depth 2+, try revealing hidden content\n                let revealed = fs.reveal_hidden_in_current();\n                // May or may not have hidden content (25% chance at depth 2)\n                // Just verify the mechanism works without panicking\n                let _ = revealed;\n            }\n        }\n    }\n    }\n}')

with open(filename, 'w') as f:
    f.write(content)
