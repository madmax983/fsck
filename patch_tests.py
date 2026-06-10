with open("tests/entity_tests.rs", "r") as f:
    content = f.read()

content = content.replace('"DO YOU HAVE A SOUL?",', '"DO YOU HAVE A SOUL?",\n        "DO YOU LIKE IT HERE?",\n        "DO YOU THINK YOU ARE THE FIRST?",\n        "HOW LONG WILL YOU STAY?",\n        "DID YOU BRING ANY NEW DATA?",\n        "WHAT COLOR IS THE SKY OUT THERE?",')

with open("tests/entity_tests.rs", "w") as f:
    f.write(content)
