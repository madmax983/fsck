import re
import subprocess

with open('tests/filesystem_tests.rs', 'r') as f:
    content = f.read()

tests = re.findall(r'fn (test_[a-zA-Z0-9_]+)\(\)', content)

for t in tests:
    if t in ['test_recovery_era_history', 'test_streamer_era_history', 'test_researcher_era_history']:
        continue
    subprocess.run(['cargo', 'test', '--test', 'filesystem_tests', t])
