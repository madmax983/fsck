import os
import glob
import re

for filepath in glob.glob("tests/*.rs"):
    with open(filepath, "r") as f:
        content = f.read()

    content = re.sub(
        r"FilesystemGenerator::generate\(([^,]+), ([^)]+)\)",
        r"FilesystemGenerator::generate(\1, \2, None)",
        content
    )

    content = re.sub(
        r"FilesystemGenerator::generate_with_content\(([^,]+), ([^)]+)\)",
        r"FilesystemGenerator::generate_with_content(\1, \2, None)",
        content
    )

    with open(filepath, "w") as f:
        f.write(content)
