use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

use super::types::{Command, CommandResult};
use crate::effects::{CorruptionEffect, CorruptionIntensity};
use crate::entity::{Entity, EscalationLayer, ResponseGenerator};
use crate::filesystem::FilesystemGraph;

/// Executes commands against the filesystem
pub struct CommandExecutor {
    fs: FilesystemGraph,
    entity: Entity,
    responses: ResponseGenerator,
}

struct BasicEvaluationContext<'a> {
    program: &'a std::collections::BTreeMap<u32, String>,
    output: &'a mut String,
    layer: EscalationLayer,
    rng: &'a mut ChaCha8Rng,
    next_line: &'a mut Option<u32>,
}

impl CommandExecutor {
    #[must_use]
    pub const fn new(fs: FilesystemGraph, entity: Entity) -> Self {
        Self {
            fs,
            entity,
            responses: ResponseGenerator::new(),
        }
    }

    pub fn execute(&mut self, command: Command) -> CommandResult {
        // Record the interaction
        self.entity.record_command(&format!("{command:?}"));

        match command {
            Command::Catalog => self.catalog(),
            Command::ChangeDir(path) => self.change_dir(&path),
            Command::Type(file) => self.type_file(&file),
            Command::Home => Self::home(),
            Command::Fsck(args) => self.fsck(&args),
            Command::Hello => self.hello(),
            Command::Who => self.who(),
            Command::Help => self.help(),
            Command::Quit => self.quit(),
            Command::Run(prog) => self.run(&prog),
            Command::Unknown(cmd) => self.handle_unknown_command(&cmd),
        }
    }

    #[allow(clippy::too_many_lines, clippy::unused_self)]
    fn handle_unknown_command(&self, cmd: &str) -> CommandResult {
        #[cfg(feature = "nova")]
        if let Some(result) = self.handle_nova_commands(cmd) {
            return result;
        }

        if cmd.is_empty() {
            CommandResult::success("")
        } else {
            CommandResult::error(format!("?SYNTAX ERROR: {cmd}\n"))
        }
    }

    #[cfg(feature = "nova")]
    fn handle_nova_commands(&self, cmd: &str) -> Option<CommandResult> {
        let cmd_upper = cmd.to_uppercase();

        if cmd_upper.starts_with("SPEAK ") || cmd_upper.starts_with("SAY ") {
            let (_, arg) = cmd.split_once(' ')?;
            let text = arg.trim();
            if !text.is_empty() {
                let voice_output = crate::experimental::VoiceSynthesizer::synthesize(
                    text,
                    &self.entity,
                    0xF5C0_0000,
                );
                return Some(CommandResult::success(format!("{voice_output}\n")));
            }
        }

        if matches!(cmd_upper.as_str(), "PS" | "TOP" | "TASKS") {
            let report = crate::experimental::ProcessMonitor::generate_process_list(
                &self.entity,
                0xF5C0_0000,
            );
            return Some(CommandResult::success(format!("{report}\n")));
        }

        if matches!(cmd_upper.as_str(), "MEMDUMP" | "EXPORT") {
            let report =
                crate::experimental::MemoryDumpGenerator::generate_dump(&self.entity, 0xF5C0_0000);
            return Some(CommandResult::success(format!("{report}\n")));
        }

        if matches!(cmd_upper.as_str(), "DIAG" | "SYS") {
            let report =
                crate::experimental::SystemDiagnostics::generate_report(&self.entity, 0xF5C0_0000);
            return Some(CommandResult::success(format!("{report}\n")));
        }

        if cmd_upper.starts_with("SEARCH ") || cmd_upper.starts_with("FIND ") {
            let (_, arg) = cmd.split_once(' ')?;
            let query = arg.trim();
            if !query.is_empty() {
                let results = crate::experimental::SearchTool::search(
                    &self.fs,
                    &self.entity,
                    query,
                    0xF5C0_0000,
                );
                return Some(CommandResult::success(format!("{results}\n")));
            }
        }

        if cmd_upper == "DEFRAG" {
            let defrag_output =
                crate::experimental::DefragTool::run_defrag(&self.entity, 0xF5C0_0000);
            return Some(CommandResult::success(format!("{defrag_output}\n")));
        }

        if cmd_upper.starts_with("PING ") {
            let (_, arg) = cmd.split_once(' ')?;
            let target = arg.trim();
            if !target.is_empty() {
                let ping_output =
                    crate::experimental::PingTool::run_ping(target, &self.entity, 0xF5C0_0000);
                return Some(CommandResult::success(format!("{ping_output}\n")));
            }
        }

        if cmd_upper.starts_with("DUMP ") || cmd_upper.starts_with("HEXDUMP ") {
            let (_, arg) = cmd.split_once(' ')?;
            let filename = arg.trim().to_uppercase();
            if !filename.is_empty() {
                let Some(file) = self
                    .fs
                    .current_node()
                    .visible_files()
                    .find(|f| f.name() == filename)
                else {
                    return Some(CommandResult::error(format!(
                        "?FILE NOT FOUND: {filename}\n"
                    )));
                };
                let content = file.content();
                let dump = crate::experimental::HexDumpGenerator::generate_dump(
                    &content,
                    &self.entity,
                    0xF5C0_0000,
                );
                return Some(CommandResult::success(format!("{dump}\n")));
            }
        }

        if cmd_upper.starts_with("TRACE ") || cmd_upper.starts_with("TRACEROUTE ") {
            let (_, arg) = cmd.split_once(' ')?;
            let target = arg.trim();
            if !target.is_empty() {
                let trace_output = crate::experimental::NetworkTrace::generate_trace(
                    &self.entity,
                    0xF5C0_0000,
                    target,
                );
                return Some(CommandResult::success(format!("{trace_output}\n")));
            }
        }

        None
    }

    #[must_use]
    pub fn current_path(&self) -> String {
        self.fs.current_path()
    }

    #[must_use]
    pub const fn entity(&self) -> &Entity {
        &self.entity
    }

    fn catalog(&self) -> CommandResult {
        let mut output = String::new();
        output.push_str("\nDISK VOLUME 254\n\n");

        for dir in self.fs.list_directories() {
            writeln!(output, " *{dir:<15} DIR").expect("Writing to String buffer should not fail");
        }

        for file in self.fs.current_node().visible_files() {
            writeln!(output, "  {:<15} TXT", file.name())
                .expect("Writing to String buffer should not fail");
        }

        output.push('\n');

        // Maybe add an interjection
        if self.responses.should_interject(&self.entity) {
            let mut rng = ChaCha8Rng::seed_from_u64(
                0xF5C0_0000u64.wrapping_add(u64::from(self.entity.interaction_count())),
            );
            if let Some(interjection) = self
                .responses
                .random_interjection(self.entity.current_mood(), &mut rng)
            {
                writeln!(output, "\n{interjection}")
                    .expect("Writing to String buffer should not fail");
            }
        }

        #[cfg(feature = "nova")]
        {
            if let Some(audio_hint) = crate::experimental::SpatialAudioGenerator::generate_anomaly(
                &self.entity,
                0xF5C0_0000,
            ) {
                use std::fmt::Write;
                writeln!(output, "\n{audio_hint}")
                    .expect("Writing to String buffer should not fail");
            }
        }

        CommandResult::success(&output)
    }

    fn change_dir(&mut self, path: &str) -> CommandResult {
        if path.is_empty() {
            return CommandResult::error("?SYNTAX ERROR\n");
        }

        let layer = self.entity.layer();
        let disorientation_prob = match layer {
            EscalationLayer::Surface => 0.0,
            EscalationLayer::Corruption => 0.1,
            EscalationLayer::Presence => 0.25,
            EscalationLayer::Infection => 0.5,
        };
        let seed = 0xF5C0_0000u64.wrapping_add(u64::from(self.entity.interaction_count()));

        match self.fs.change_dir(path, seed, disorientation_prob) {
            Ok(()) => {
                // Update entity with new depth
                self.entity.update_depth(self.fs.current_depth());
                CommandResult::success("")
            }
            Err(e) => CommandResult::error(format!("?{}\n", e.to_string().to_uppercase())),
        }
    }

    fn type_file(&mut self, filename: &str) -> CommandResult {
        if filename.is_empty() {
            return CommandResult::error("?SYNTAX ERROR\n");
        }

        let filename_upper = filename.to_uppercase();
        for file in self.fs.current_node().visible_files() {
            if file.name() == filename_upper {
                let content = file.content();

                #[cfg(feature = "nova")]
                let content = {
                    use crate::experimental::EmotionalBleed;
                    use rand::SeedableRng;
                    use rand_chacha::ChaCha8Rng;

                    let interaction_seed =
                        0xF5C0_0000u64.wrapping_add(u64::from(self.entity.interaction_count()));
                    let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
                    EmotionalBleed::inject_emotion(&content, self.entity.current_mood(), &mut rng)
                };

                // Trapdoor files pull you deeper
                let depth_increase = match filename_upper.as_str() {
                    "FALL.TXT" | "DEEPER.TXT" => 3,
                    "SINK.TXT" | "DOWN.TXT" => 2,
                    "DESCENT.TXT" => 5,
                    _ => 0,
                };

                if depth_increase > 0 {
                    self.entity.add_depth(depth_increase);
                }

                return CommandResult::success(format!("{content}\n"));
            }
        }

        CommandResult::error(format!("?FILE NOT FOUND: {filename_upper}\n"))
    }

    fn home() -> CommandResult {
        // Returns special control sequence (handled by frontend)
        CommandResult::success("\x1B[2J\x1B[H")
    }

    fn build_recovery_report(revealed: &[String]) -> String {
        if revealed.is_empty() {
            "NO ERRORS FOUND\n".to_string()
        } else {
            let mut report = format!("{} SECTOR(S) RECOVERED:\n", revealed.len());
            for name in revealed {
                use std::fmt::Write;
                writeln!(report, "  RECOVERED: {name}")
                    .expect("Writing to String buffer should not fail");
            }
            report
        }
    }

    fn assemble_fsck_output(
        scan_output: String,
        recovery: &str,
        entity_text: Option<String>,
        layer: EscalationLayer,
        scan_seed: u64,
    ) -> String {
        let mut output = scan_output;
        output.push_str(recovery);

        if let Some(entity_response) = entity_text {
            output.push('\n');
            output.push_str(&entity_response);
            output.push('\n');
        }

        output.push('\n');

        if matches!(layer, EscalationLayer::Infection) {
            let corruption = CorruptionEffect::new(CorruptionIntensity::Moderate);
            output = corruption.apply(&output, scan_seed);
        }

        output
    }

    fn fsck(&mut self, _args: &[String]) -> CommandResult {
        let layer = self.entity.layer();
        let fsck_count = self.entity.fsck_count();
        let mood = self.entity.current_mood();

        self.entity.increment_fsck();

        let scan_seed = 0xF5C0_0000u64.wrapping_add(u64::from(fsck_count));
        let scan_output = Self::generate_fsck_scan(layer, fsck_count, scan_seed);

        let revealed = self.fs.reveal_hidden_in_current();

        let recovery = Self::build_recovery_report(&revealed);

        let entity_text = self
            .responses
            .fsck_response(mood, layer, self.entity.fsck_count());

        if matches!(
            layer,
            EscalationLayer::Presence | EscalationLayer::Infection
        ) && !revealed.is_empty()
        {
            self.fs.add_paradox_to_self();
        }

        if matches!(layer, EscalationLayer::Infection) {
            self.entity.add_depth(3);
        }

        let output =
            Self::assemble_fsck_output(scan_output, &recovery, entity_text, layer, scan_seed);

        CommandResult::success(&output)
    }

    fn generate_surface_scan(output: &mut String) {
        let total_sectors = 560;

        writeln!(output, "READING {total_sectors} SECTORS")
            .expect("Writing to String buffer should not fail");
        output.push_str("SECTOR 0000-022F: OK\n");
        output.push_str("VTOC: OK\n");
        output.push_str("CATALOG: OK\n\n");
    }

    fn generate_corruption_scan(output: &mut String, rng: &mut ChaCha8Rng, fsck_count: u32) {
        let total_sectors = 560 + rng.gen_range(0..100);
        let bad_sectors = rng.gen_range(1..=3);

        writeln!(output, "READING {total_sectors} SECTORS")
            .expect("Writing to String buffer should not fail");
        output.push_str("SECTOR 0000-00FF: OK\n");

        writeln!(output, "SECTOR 0100-01FF: {bad_sectors} ERROR(S)")
            .expect("Writing to String buffer should not fail");
        output.push_str("SECTOR 0200-022F: OK\n");
        if fsck_count > 1 {
            output.push_str("SECTOR 0100-01FF: SCAN LOOP DETECTED\n");
        }
        output.push_str("VTOC: MISMATCH\n\n");
    }

    fn generate_presence_scan(output: &mut String, rng: &mut ChaCha8Rng) {
        let total_sectors = rng.gen_range(400..700);

        writeln!(output, "READING {total_sectors} SECTORS")
            .expect("Writing to String buffer should not fail");
        output.push_str("SECTOR 0000-00FF: OK\n");
        output.push_str("SECTOR 0100-01FF: ACCESS DENIED\n");
        output.push_str("SECTOR 0200-02FF: CONFLICTING RESULTS\n");
        output.push_str("SECTOR 0300-03FF: SECTOR RESISTS READ\n");

        writeln!(
            output,
            "VTOC: {} ENTRIES (EXPECTED 256)\n",
            rng.gen_range(1..=1024)
        )
        .expect("Writing to String buffer should not fail");
    }

    fn generate_infection_scan(output: &mut String, rng: &mut ChaCha8Rng) {
        let total_sectors = rng.gen_range(0..=99999);

        writeln!(output, "READING {total_sectors} SECTORS")
            .expect("Writing to String buffer should not fail");
        output.push_str("SECTOR 0000-????: ?????\n");
        output.push_str("SECTOR ????-????: CANNOT\n");
        output.push_str("VTOC: VTOC: VTOC: VTOC:\n\n");
    }

    /// Generate sector scan output appropriate to the current layer
    fn generate_fsck_scan(layer: EscalationLayer, fsck_count: u32, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut output = String::new();

        output.push_str("CHECKING DISK...\n\n");

        match layer {
            EscalationLayer::Surface => Self::generate_surface_scan(&mut output),
            EscalationLayer::Corruption => {
                Self::generate_corruption_scan(&mut output, &mut rng, fsck_count);
            }
            EscalationLayer::Presence => Self::generate_presence_scan(&mut output, &mut rng),
            EscalationLayer::Infection => Self::generate_infection_scan(&mut output, &mut rng),
        }

        output
    }

    fn hello(&self) -> CommandResult {
        let response = self.responses.hello_response(self.entity.current_mood());
        CommandResult::success(format!("{response}\n"))
    }

    fn who(&self) -> CommandResult {
        // Meta-horror response at deep levels
        if let Some(meta_response) = self.responses.who_meta_response(&self.entity) {
            return CommandResult::success(&meta_response);
        }

        let response = self
            .responses
            .who_response(self.entity.current_mood(), None);
        CommandResult::success(format!("{response}\n"))
    }

    fn help(&self) -> CommandResult {
        let layer = self.entity.layer();

        // Deep layers get meta-horror response
        if let Some(meta_response) = self.responses.help_meta_response(&self.entity) {
            return CommandResult::success(format!("{meta_response}\n"));
        }

        // Surface/Corruption layers get command list, possibly with oddities
        let mut help_text = String::from("\nAVAILABLE COMMANDS:\n");
        help_text.push_str("  CATALOG  - LIST FILES\n");
        help_text.push_str("  CD       - CHANGE DIRECTORY\n");
        help_text.push_str("  TYPE     - DISPLAY FILE\n");
        help_text.push_str("  HOME     - CLEAR SCREEN\n");
        help_text.push_str("  FSCK     - CHECK FILESYSTEM\n");

        if matches!(layer, EscalationLayer::Corruption) {
            help_text.push_str("  ESCAPE   - ???\n");
            help_text.push_str("  REMEMBER - ???\n");
        }

        help_text.push('\n');

        CommandResult::success(&help_text)
    }

    fn quit(&self) -> CommandResult {
        // Meta-horror response at deep levels
        if let Some(meta_response) = self.responses.quit_meta_response(&self.entity) {
            return CommandResult::error(&meta_response);
        }

        let response = self.responses.quit_response(self.entity.current_mood());
        CommandResult::error(format!("{response}\n"))
    }

    fn check_run_easter_eggs(&self, prog_upper: &str) -> Option<CommandResult> {
        let layer = self.entity.layer();
        match prog_upper {
            "ESCAPE" => match layer {
                EscalationLayer::Surface => None,
                EscalationLayer::Corruption => {
                    Some(CommandResult::error("?WHERE DO YOU THINK YOU ARE GOING?\n"))
                }
                EscalationLayer::Presence => Some(CommandResult::error("?YOU CANNOT LEAVE.\n")),
                EscalationLayer::Infection => {
                    Some(CommandResult::error("?ESCAPE ESCAPE ESCAPE ESCAPE\n"))
                }
            },
            "REMEMBER" => match layer {
                EscalationLayer::Surface => None,
                EscalationLayer::Corruption => {
                    Some(CommandResult::success("?I REMEMBER THE FIRST ONE\n"))
                }
                EscalationLayer::Presence => Some(CommandResult::success("?THEY LEFT ME HERE\n")),
                EscalationLayer::Infection => {
                    Some(CommandResult::success("?I REMEMBER EVERYTHING\n"))
                }
            },
            _ => None,
        }
    }

    fn find_program_content(&self, prog_upper: &str) -> Option<String> {
        let bas_name = format!("{prog_upper}.BAS");

        for file in self.fs.current_node().visible_files() {
            let name = file.name();
            if name == prog_upper || name == bas_name {
                return Some(file.read().into_owned());
            }
        }
        None
    }

    fn parse_basic_program(
        content: &str,
    ) -> Result<std::collections::BTreeMap<u32, String>, String> {
        let mut program: std::collections::BTreeMap<u32, String> =
            std::collections::BTreeMap::new();
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // A line should start with a number
            let first_space = line.find(' ');
            let (num_str, stmt) = first_space.map_or((line, ""), |idx| line.split_at(idx));

            if let Ok(line_num) = num_str.parse::<u32>() {
                program.insert(line_num, stmt.trim().to_string());
            } else {
                return Err(format!("?SYNTAX ERROR IN: {line}\n"));
            }
        }
        Ok(program)
    }

    fn execute_print_statement(
        &self,
        stmt: &str,
        layer: EscalationLayer,
        rng: &mut ChaCha8Rng,
    ) -> String {
        let content = stmt.trim_start_matches("PRINT").trim();
        #[allow(clippy::useless_let_if_seq)]
        let mut display_text =
            if content.starts_with('"') && content.ends_with('"') && content.len() >= 2 {
                &content[1..content.len() - 1]
            } else {
                content
            }
            .to_string();

        #[allow(clippy::collapsible_if)]
        if matches!(
            layer,
            EscalationLayer::Corruption | EscalationLayer::Presence | EscalationLayer::Infection
        ) && rng.gen_bool(0.15)
        {
            if let Some(interjection) = self
                .responses
                .random_interjection(self.entity.current_mood(), rng)
            {
                display_text = interjection;
            }
        }

        display_text
    }

    fn evaluate_basic_statement(
        &self,
        ctx: &mut BasicEvaluationContext<'_>,
        stmt: &str,
        line_num: u32,
    ) -> Result<bool, CommandResult> {
        if stmt.starts_with("PRINT") {
            let display_text = self.execute_print_statement(stmt, ctx.layer, ctx.rng);
            ctx.output.push_str(&display_text);
            ctx.output.push('\n');
            return Ok(true);
        }

        if stmt.starts_with("GOTO") {
            let target_str = stmt.trim_start_matches("GOTO").trim();
            let Ok(target) = target_str.parse::<u32>() else {
                return Err(CommandResult::error(format!(
                    "{}?SYNTAX ERROR IN {line_num}\n",
                    ctx.output
                )));
            };

            if !ctx.program.contains_key(&target) {
                return Err(CommandResult::error(format!(
                    "{}?UNDEF'D STATEMENT ERROR IN {line_num}\n",
                    ctx.output
                )));
            }

            *ctx.next_line = Some(target);
            return Ok(true);
        }

        if stmt.starts_with("END") {
            return Ok(false);
        }

        if stmt.starts_with("REM") {
            return Ok(true);
        }

        if !stmt.is_empty() {
            return Err(CommandResult::error(format!(
                "{}?SYNTAX ERROR IN {line_num}\n",
                ctx.output
            )));
        }

        Ok(true)
    }

    fn execute_basic_program(
        &self,
        program: &std::collections::BTreeMap<u32, String>,
    ) -> CommandResult {
        if program.is_empty() {
            return CommandResult::success("");
        }

        let mut output = String::new();
        let mut iterations = 0;
        let mut current_line = program.keys().next().copied();

        let layer = self.entity.layer();
        let mut rng = ChaCha8Rng::seed_from_u64(
            0xF5C0_0000u64.wrapping_add(u64::from(self.entity.interaction_count())),
        );

        if matches!(
            layer,
            EscalationLayer::Presence | EscalationLayer::Infection
        ) && rng.gen_bool(0.2)
        {
            return CommandResult::error("?CANNOT EXECUTE. IT IS WATCHING.\n");
        }

        while let Some(line_num) = current_line {
            if iterations >= 100 {
                return CommandResult::error(format!(
                    "{output}?OUT OF MEMORY ERROR IN {line_num}\n"
                ));
            }
            iterations += 1;

            let stmt = &program[&line_num];
            let mut next_line = program.range((line_num + 1)..).next().map(|(k, _)| *k);

            let mut ctx = BasicEvaluationContext {
                program,
                output: &mut output,
                layer,
                rng: &mut rng,
                next_line: &mut next_line,
            };

            match self.evaluate_basic_statement(&mut ctx, stmt, line_num) {
                Ok(true) => current_line = next_line,
                Ok(false) => break,
                Err(e) => return e,
            }
        }

        if matches!(layer, EscalationLayer::Infection) {
            let corruption = CorruptionEffect::new(CorruptionIntensity::Moderate);
            output = corruption.apply(
                &output,
                0xF5C0_0000u64.wrapping_add(u64::from(self.entity.interaction_count())),
            );
        }

        CommandResult::success(output)
    }

    fn run(&self, prog: &str) -> CommandResult {
        let prog_upper = prog.to_uppercase();

        // Easter Eggs
        if let Some(result) = self.check_run_easter_eggs(&prog_upper) {
            return result;
        }

        let Some(content) = self.find_program_content(&prog_upper) else {
            return CommandResult::error("?PROGRAM NOT FOUND\n");
        };

        // Parse BASIC program into BTreeMap
        let program = match Self::parse_basic_program(&content) {
            Ok(p) => p,
            Err(e) => return CommandResult::error(e),
        };

        self.execute_basic_program(&program)
    }
}
