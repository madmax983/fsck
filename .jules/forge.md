**[Generate Fsck Scan]**
**Learning:** Hardcoded strings in `generate_surface_scan` and other `generate_*_scan` functions make them less flexible and repetitive, returning string outputs in an unstructured format.
**Action:** Extract the scan generation logic to its own module or helper functions to improve readability and enforce idiomatic Rust patterns without changing runtime behavior.

**[Refactor FsckScanGenerator strings]**
**Learning:** Large, multi-line string templates using `format!` can be difficult to read and modify incrementally (e.g., conditionally appending text like `loop_warning`).
**Action:** Replace monolithic `format!` macros with `String::with_capacity` and sequential `writeln!` calls to construct output efficiently and explicitly.

**[Refactor handle_nova_commands]**
**Learning:** A giant match statement where each arm executes significant logic acts as a "God Object" and "Pyramid of Doom", drastically reducing readability and violating DRY/single-responsibility principles. Also, helper functions unconditionally returning `Some(CommandResult)` caused `clippy::unnecessary_wraps` lints.
**Action:** Extract match arms into separate, strongly typed helper functions. Ensure those helper functions directly return the inner type (e.g., `CommandResult`) instead of an unnecessary `Option`, and wrap it in `Some(...)` only at the call site within the match statement.
**[Iterator Chains vs Manual Loops]**
**Learning:** Deeply nested `for` loops used for linear searches (e.g., iterating to find a file by name inside a collection) act as a "Pyramid of Doom" and obscure the main logic of the function.
**Action:** Replace manual `for` loops used for searching with idiomatic `Iterator::find(...)` pipelines, coupled with Guard Clauses (early returns using `let Some(...) = ... else { return; }`) to flatten functions and dramatically improve readability.
**[Extracted God Function logic in FilesystemGenerator::add_files]**
**Learning:** `add_files` was a God Function because it contained deep nesting and complex `match` statements for generating multiple different file types inline.
**Action:** Extract specific file generation types into smaller, private helper functions (e.g., `generate_trapdoor_file`, `generate_dynamic_file`) to flatten nesting and improve overall readability of the core generation loops.
**[Refactor God-function Conversion & String Allocations]**
**Learning:** Hardcoded, monolithic depth-to-Era conversion mappings (`match` blocks) inside generator functions make business logic bloated. Also, using `.fold(String::new(), ...)` to construct strings dynamically generates unnecessary heap allocations when the content could be appended directly.
**Action:** Extract generic type conversions into `From<T> / Into<T>` trait implementations to encapsulate mapping logic cleanly out of main execution loops. Replace intermediate string generation in closures by using simple `for` loops writing directly to the target mutable string buffer using `writeln!`.
**[Extracted God Function logic in SystemDiagnostics::generate_report]**
**Learning:** `generate_report` was a God Function because it contained deep nesting and complex `match` statements for generating multiple different report types inline.
**Action:** Extract specific report generation types into smaller, private helper functions (e.g., `generate_surface_report`, `generate_corruption_report`) to flatten nesting and improve overall readability of the core generation loops.
**[Extracted God Function logic in SearchTool::format_match]**
**Learning:** `format_match` inside `src/experimental/grep.rs` was a God Function because it contained deep nesting and complex `match` statements for generating multiple different match formatting logic inline.
**Action:** Extract specific match formatting types into smaller, private helper functions (e.g., `format_surface_match`, `format_corruption_match`) to flatten nesting and improve overall readability of the core generation loops.
**[Extract God Function in UserProfiler::generate_profile]**
**Learning:**  inside  was a God Function because it mixed data gathering (command counting) and complex nested formatting logic (tendency and psychological assessment matching).
**Action:** Extract specific formatting and counting logic into smaller, private helper functions (e.g., , ) to flatten nesting and improve overall readability of the core generation loops.
**[Extract God Function in UserProfiler::generate_profile]**
**Learning:** `generate_profile` inside `src/experimental/profile.rs` was a God Function because it mixed data gathering (command counting) and complex nested formatting logic (tendency and psychological assessment matching).
**Action:** Extract specific formatting and counting logic into smaller, private helper functions (e.g., `generate_tendency_report`, `generate_psychological_assessment`) to flatten nesting and improve overall readability of the core generation loops.
**[Extracted God Function logic in HardwareSensors::get_readings and ProcessMonitor::generate_process_list]**
**Learning:** Functions that generate output strings based on escalating depth layers (like `EscalationLayer::Surface` through `Infection`) using inline inline generation loops inside large `match` statements act as "Pyramid of Doom" God Functions.
**Action:** Extract the specific string-generation blocks for each distinct layer into small, private helper functions (e.g., `generate_surface_readings`) to flatten nesting, isolate logic, and improve overall readability of the core generation loops.
**[Extract God Function logic in EnvVarsGenerator::generate_env]**
**Learning:** `generate_env` inside `src/experimental/env.rs` was a God Function because it contained deep nesting and complex inline string generation logic inside a large `match layer` statement for multiple different environment generation layers.
**Action:** Extract specific formatting logic into smaller, private helper functions (e.g., `generate_surface_env`, `generate_corruption_env`) to flatten nesting, isolate logic, and improve overall readability of the core generation loops.
**[Extracted God Function logic in HexDumpGenerator and LifeSimulator]**
**Learning:** `generate_dump` and `simulate_life` were God Functions because they contained deep nesting and complex inline string generation/character selection logic based on escalation layers inside loops.
**Action:** Extract specific formatting and selection blocks for each distinct layer into small, private helper functions (e.g., `format_hex_bytes`, `format_ascii_decoding`, `get_cell_character`) to flatten nesting, isolate logic, and improve overall readability of the core loops.
**[Refactor Fortune and Memdump God Functions]**
**Learning:** God Functions in  and  used massive  statements based on  to build strings sequentially, violating single-responsibility and creating a pyramid of doom.
**Action:** Extract each layer's generation logic into specific helper functions (e.g. , ) to flatten the structure and encapsulate the logic.
**[Refactor Fortune and Memdump God Functions]**
**Learning:** God Functions in `fortune.rs` and `memdump.rs` used massive `match` statements based on `EscalationLayer` to build strings sequentially, violating single-responsibility and creating a pyramid of doom.
**Action:** Extract each layer's generation logic into specific helper functions (e.g. `generate_surface_fortune`, `generate_corruption_dump`) to flatten the structure and encapsulate the logic.
**[Refactor PingTool God Function]**\n**Learning:** The `run_ping` function inside `src/experimental/ping.rs` acted as a God Function with a massive inline `match` statement on `EscalationLayer`, creating a Pyramid of Doom.\n**Action:** Extracted each match arm into dedicated, smaller helper functions (e.g., `generate_surface_ping`) to flatten nesting, encapsulate logic, and improve readability.

**[Extract God Function in StatTool and UndeleteTool]**
**Learning:** `StatTool::generate_stat`, `UndeleteTool::run_undelete`, and `UndeleteTool::generate_fragment` were God Functions because they contained deep nesting and complex inline string generation logic inside a large `match layer` statement for multiple different generation layers.
**Action:** Extract specific formatting logic into smaller, private helper functions (e.g., `generate_surface_stat`, `generate_corruption_undelete`) to flatten nesting, isolate logic, and improve overall readability of the core generation loops.
**[Extract God Function formatting logic in various experimental components]
**Learning:** Functions like `analyze`, `dial`, `scan_entropy`, `generate_dream`, and `simulate_update` were God Functions because they contained deep nesting and complex inline string generation logic inside a large `match layer` statement for multiple different generation layers.
**Action:** Extract specific formatting logic into smaller, private helper functions (e.g., `generate_surface_analysis`, `generate_corruption_dial`) to flatten nesting, isolate logic, and improve overall readability of the core generation functions.
**[Extract God Function in EmotionalBleed::inject_emotion]
**Learning:** `inject_emotion` inside `src/experimental/emotional_bleed.rs` was a God Function because it mixed complex inline string generation logic inside a large `match mood` statement for multiple different generation layers.
**Action:** Extract specific formatting logic into smaller, private helper functions (e.g., `inject_helpful_bleed`, `inject_wounded_bleed`) to flatten nesting, isolate logic, and improve overall readability of the core generation loops.

**[Extract Executor Dynamic Logic]**
**Learning:** `inject_dynamic_file_content` contained deep inline loops and logic to build output strings based on filenames inside a match block, making it difficult to read.
**Action:** Extracted the inner loops of the match block into smaller, strongly-typed helper methods (`inject_observe_txt`, `inject_machine_log`, `inject_history_txt`) to flatten the function.

**[Refactor Sequential Check Logic]**
**Learning:** `from_previous_session` contained four sequential, near-identical `if` statements performing string formatting. This violates DRY and acts as an anti-pattern.
**Action:** Replaced repetitive sequential `if` conditions with a concise `for` loop iterating over an array of parameter tuples (the string pattern to search for, and the response to append) to significantly shorten the code.

**[Extract Grep Search Logic]**
**Learning:** `SearchTool::search` handled iterating over files, checking matching logic, and formatting the output logic entirely within a single loop, leading to a God Object iteration pattern.
**Action:** Extracted the content extraction, matching logic, and result aggregation into a specific helper method (`check_and_format_match`) invoked by the outer iteration.
**[Refactor Basic Matcher & String Matching Options]**
**Learning:** `src/commands/executor.rs`'s `handle_nova_commands` and `src/commands/types.rs`'s `from_input` were God Functions implementing "Pyramid of Doom" architectures using many `if / else if` string-matching statements and multiple heap allocations instead of `match` guards.
**Action:** Flatten these using a `match` expression with guards, like `match (cmd_word, arg) { (c, a) if c.eq_ignore_ascii_case("CMD") && a.is_empty() => { ... } }`, avoiding any need for string heap allocations, improving logic isolation, and maintaining zero-behavior change.
