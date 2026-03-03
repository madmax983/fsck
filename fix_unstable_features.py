import re
import os

def fix_file(filename):
    with open(filename, 'r') as f:
        content = f.read()

    # Revert let_chains which requires nightly
    content = re.sub(r'if let Some\(([^)]+)\)\s*=\s*(.+?)\s*&&\s*(.+?)\.is_ok\(\)\s*\{', r'if let Some(\1) = \2 {\n            if \3.is_ok() {', content)

    # Also for normal if with let_chains
    content = re.sub(r'if (.+?)\s*&&\s*let Some\((.+?)\) = (.+?)\s*\{', r'if \1 {\n            if let Some(\2) = \3 {', content)

    # Revert is_multiple_of which might not be in stable
    content = content.replace('.is_multiple_of(3)', ' % 3 == 0')
    content = content.replace('.is_multiple_of(2)', ' % 2 == 0')

    with open(filename, 'w') as f:
        f.write(content)

for root, _, files in os.walk('src'):
    for file in files:
        if file.endswith('.rs'):
            fix_file(os.path.join(root, file))

for root, _, files in os.walk('tests'):
    for file in files:
        if file.endswith('.rs'):
            fix_file(os.path.join(root, file))
