import re

with open("src/commands/executor.rs", "r") as f:
    content = f.read()

old_code = """        if matches!(layer, EscalationLayer::Infection) {
            self.entity.add_depth(3);
            let corruption = CorruptionEffect::new(CorruptionIntensity::Moderate);
            output = corruption.apply(&output, scan_seed);
        }

        CommandResult::success(&output)"""

new_code = """        if matches!(layer, EscalationLayer::Infection) {
            self.entity.add_depth(3);
            let corruption = CorruptionEffect::new(CorruptionIntensity::Moderate);
            output = corruption.apply(&output, scan_seed).into_owned();
        }

        CommandResult::success(&output)"""

if old_code in content:
    content = content.replace(old_code, new_code)
    print("Replaced executor.rs 1 successfully")
else:
    print("Old code not found in executor.rs")

old_code = """        if matches!(layer, EscalationLayer::Infection) {
            let corruption = CorruptionEffect::new(CorruptionIntensity::Moderate);
            output = corruption.apply(
                &output,
                0xF5C0_0000u64.wrapping_add(u64::from(self.entity.interaction_count())),
            );
        }

        CommandResult::success(output)"""

new_code = """        if matches!(layer, EscalationLayer::Infection) {
            let corruption = CorruptionEffect::new(CorruptionIntensity::Moderate);
            output = corruption.apply(
                &output,
                0xF5C0_0000u64.wrapping_add(u64::from(self.entity.interaction_count())),
            ).into_owned();
        }

        CommandResult::success(output)"""

if old_code in content:
    content = content.replace(old_code, new_code)
    print("Replaced executor.rs 2 successfully")
else:
    print("Old code not found in executor.rs 2")

with open("src/commands/executor.rs", "w") as f:
    f.write(content)
