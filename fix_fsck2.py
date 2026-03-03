filename = 'tests/fsck_tests.rs'
with open(filename, 'r') as f:
    content = f.read()

content = '#![allow(clippy::collapsible_if)]\n' + content

with open(filename, 'w') as f:
    f.write(content)
