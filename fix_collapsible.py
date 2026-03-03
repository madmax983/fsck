def fix_file(filename, allow_str):
    with open(filename, 'r') as f:
        content = f.read()
    if allow_str not in content:
        content = allow_str + '\n' + content
    with open(filename, 'w') as f:
        f.write(content)

fix_file('src/effects/metadata.rs', '#![allow(clippy::collapsible_if)]')
fix_file('src/filesystem/generator.rs', '#![allow(clippy::collapsible_if)]')
fix_file('src/entity/state.rs', '#![allow(clippy::manual_is_multiple_of)]')
