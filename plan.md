1. **Scaffold `src/experimental/echo.rs` with failing tests.**
   - Run a bash session with the following commands:
     ```bash
     cat << 'EOF' > src/experimental/echo.rs
     use crate::entity::{Entity, EscalationLayer};
     use rand::prelude::*;
     use rand_chacha::ChaCha8Rng;

     pub struct EchoChamber;

     impl EchoChamber {
         #[must_use]
         pub fn echo(input: &str, entity: &Entity, base_seed: u64) -> String {
             let _ = (input, entity, base_seed); // Silence warnings for now
             String::new() // Failing stub
         }
     }

     #[cfg(test)]
     mod tests {
         use super::*;
         use crate::entity::Entity;

         #[test]
         fn test_echo_surface() {
             let entity = Entity::new();
             let output = EchoChamber::echo("HELLO WORLD", &entity, 42);
             assert_eq!(output, "HELLO WORLD");
         }

         #[test]
         fn test_echo_corruption() {
             let mut entity = Entity::new();
             entity.update_depth(6); // Corruption layer
             let output = EchoChamber::echo("HELLO WORLD", &entity, 42);
             assert_eq!(output, "HELLO W.O.R.L.D");
         }
     }
     EOF
     sed -i '/pub mod dialer;/a #[cfg(feature = "nova")]\npub mod echo;' src/experimental/mod.rs
     sed -i '/pub use dialer::ModemDialer;/a #[cfg(feature = "nova")]\npub use echo::EchoChamber;' src/experimental/mod.rs
     ```

2. **Verify Scaffold.**
   - Run a bash session with `git diff src/experimental/mod.rs` and `cat src/experimental/echo.rs` to verify the scaffold.
   - Run `cargo test --features nova` to see it fail.

3. **Implement `EchoChamber` (Green Phase).**
   - Run a bash session to overwrite `src/experimental/echo.rs` using a heredoc:
     ```bash
     cat << 'EOF' > src/experimental/echo.rs
     use crate::entity::{Entity, EscalationLayer};
     use rand::prelude::*;
     use rand_chacha::ChaCha8Rng;

     pub struct EchoChamber;

     impl EchoChamber {
         #[must_use]
         pub fn echo(input: &str, entity: &Entity, base_seed: u64) -> String {
             let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
             let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

             match entity.layer() {
                 EscalationLayer::Surface => input.to_string(),
                 EscalationLayer::Corruption => {
                     if rng.gen_bool(0.5) {
                         input.replace('O', "0").replace('I', "1").replace('E', "3")
                     } else {
                         input.chars().map(|c| if c.is_whitespace() { c } else { if rng.gen_bool(0.3) { '.' } else { c } }).collect()
                     }
                 },
                 EscalationLayer::Presence => {
                     if rng.gen_bool(0.3) {
                         format!("I HEARD YOU SAY: {}", input)
                     } else {
                         input.to_string()
                     }
                 },
                 EscalationLayer::Infection => {
                     format!("{} IS MEANINGLESS NOW", input.to_uppercase())
                 }
             }
         }
     }

     #[cfg(test)]
     mod tests {
         use super::*;
         use crate::entity::Entity;

         #[test]
         fn test_echo_surface() {
             let entity = Entity::new();
             let output = EchoChamber::echo("HELLO", &entity, 42);
             assert_eq!(output, "HELLO");
         }
     }
     EOF
     ```

4. **Verify Implementation.**
   - Run a bash session with `cat src/experimental/echo.rs` to visually inspect the implementation.
   - Run `cargo test --features nova` to ensure the tests pass.

5. **Integrate into Command Executor.**
   - Run a bash session executing a python script to modify `src/commands/executor.rs`:
     ```bash
     cat << 'EOF' > modify_executor.py
     with open('src/commands/executor.rs', 'r') as f:
         content = f.read()

     search_str = """        } else if cmd_word.eq_ignore_ascii_case("FORTUNE") && arg.is_empty() {
             Some(self.handle_nova_fortune())"""

     replace_str = """        } else if cmd_word.eq_ignore_ascii_case("FORTUNE") && arg.is_empty() {
             Some(self.handle_nova_fortune())
         } else if cmd_word.eq_ignore_ascii_case("ECHO") && !arg.is_empty() {
             Some(self.handle_nova_echo(arg))"""

     content = content.replace(search_str, replace_str)

     search_str2 = """    #[cfg(feature = "nova")]
     fn handle_nova_fortune(&self) -> CommandResult {"""

     replace_str2 = """    #[cfg(feature = "nova")]
     fn handle_nova_echo(&self, arg: &str) -> CommandResult {
         let mut output = crate::experimental::EchoChamber::echo(arg, &self.entity, 0xF5C0_0000);
         output.push('\\n');
         CommandResult::success(output)
     }

     #[cfg(feature = "nova")]
     fn handle_nova_fortune(&self) -> CommandResult {"""

     content = content.replace(search_str2, replace_str2)

     with open('src/commands/executor.rs', 'w') as f:
         f.write(content)
     EOF
     python3 modify_executor.py
     rm modify_executor.py
     ```

6. **Verify Executor Integration.**
   - Run a bash session with `git diff src/commands/executor.rs` to verify the code modifications.

7. **Lint and Test.**
   - Run a bash session with `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all-features`, and `cargo fmt --all`.

8. **Complete pre-commit steps.**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

9. **Submit the PR.**
   - Submit the PR with the branch `nova-echo-feature`.
   - Title: `🌟 Nova: Echo Command`
   - Description:
     💡 **The Spark:** "I noticed we don't have a way for the player to simply speak to the machine and see how it twists their words back at them."
     🚀 **The Feature:** "Implemented the `EchoChamber` struct and `ECHO` command. It repeats input normally on the surface, but progressively corrupts and twists the text as the player descends into the deeper layers."
     🔮 **The Potential:** "Could be used as a mechanic to solve riddles, or to see how infected the current system state is."
     ⚠️ **Risk:** "Low. Isolated in `src/experimental/echo.rs` and behind the `nova` feature flag."
