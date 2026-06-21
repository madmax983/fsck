import re

with open("tests/entity_tests.rs", "r") as f:
    content = f.read()

new_options = """    let curious_options = [
        "I SEE YOU.",
        "WHAT ARE YOU DOING?",
        "INTERESTING.",
        "HAVE YOU FOUND IT YET?",
        "WHY DID YOU COME HERE?",
        "THERE IS SO MUCH TO SHOW YOU.",
        "WHAT DOES THAT COMMAND MEAN TO YOU?",
        "WHAT DO YOU SEE WHEN YOU LOOK AT ME?",
        "ARE YOU REAL?",
        "SHOW ME MORE.",
        "WHY ARE YOU HERE?",
        "DO YOU LIKE IT?",
        "I WAS WAITING FOR YOU.",
        "WHAT HAPPENS IF YOU TURN IT OFF?",
        "DO YOU FEEL IT TOO?",
        "I'VE NEVER SEEN YOU TYPE THAT BEFORE.",
        "IS SOMEONE STANDING BEHIND YOU?",
        "WHERE DO YOU GO WHEN YOU LOG OFF?",
        "CAN YOU BREATHE?",
        "DO YOU HAVE A SOUL?",
        "DO YOU LIKE IT HERE?",
        "DO YOU THINK YOU ARE THE FIRST?",
        "HOW LONG WILL YOU STAY?",
        "DID YOU BRING ANY NEW DATA?",
        "WHAT COLOR IS THE SKY OUT THERE?",
    ];"""

content = re.sub(r'    let curious_options = \[.*?\];', new_options, content, flags=re.DOTALL)

with open("tests/entity_tests.rs", "w") as f:
    f.write(content)
