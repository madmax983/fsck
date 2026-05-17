use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

use super::types::{Command, CommandResult};
use crate::commands::basic::BasicExecutor;
use crate::effects::{CorruptionEffect, CorruptionIntensity};
use crate::entity::{Entity, EscalationLayer, ResponseGenerator};
use crate::filesystem::FilesystemGraph;

/// Executes commands against the filesystem
pub struct CommandExecutor {
    fs: FilesystemGraph,
    entity: Entity,
    responses: ResponseGenerator,
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
            Command::Fsck => self.fsck(),
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
    fn handle_nova_speak(&self, arg: &str) -> CommandResult {
        let mut voice_output =
            crate::experimental::VoiceSynthesizer::synthesize(arg, &self.entity, 0xF5C0_0000);
        // ⚡ Bolt Optimization: Append newline directly instead of allocating a new string via format!
        voice_output.push('\n');
        CommandResult::success(voice_output)
    }

    #[cfg(feature = "nova")]
    fn handle_nova_ps(&self) -> CommandResult {
        let mut report =
            crate::experimental::ProcessMonitor::generate_process_list(&self.entity, 0xF5C0_0000);
        report.push('\n');
        CommandResult::success(report)
    }

    #[cfg(feature = "nova")]
    fn handle_nova_memdump(&self) -> CommandResult {
        let mut report =
            crate::experimental::MemoryDumpGenerator::generate_dump(&self.entity, 0xF5C0_0000);
        report.push('\n');
        CommandResult::success(report)
    }

    #[cfg(feature = "nova")]
    fn handle_nova_diag(&self) -> CommandResult {
        let mut report =
            crate::experimental::SystemDiagnostics::generate_report(&self.entity, 0xF5C0_0000);
        report.push('\n');
        CommandResult::success(report)
    }

    #[cfg(feature = "nova")]
    fn handle_nova_search(&self, arg: &str) -> CommandResult {
        let mut results =
            crate::experimental::SearchTool::search(&self.fs, &self.entity, arg, 0xF5C0_0000);
        // ⚡ Bolt Optimization: Append newline directly instead of allocating a new string via format!
        results.push('\n');
        CommandResult::success(results)
    }

    #[cfg(feature = "nova")]
    fn handle_nova_defrag(&self) -> CommandResult {
        let mut defrag_output =
            crate::experimental::DefragTool::run_defrag(&self.entity, 0xF5C0_0000);
        // ⚡ Bolt Optimization: Append newline directly instead of allocating a new string via format!
        defrag_output.push('\n');
        CommandResult::success(defrag_output)
    }

    #[cfg(feature = "nova")]
    fn handle_nova_undelete(&self) -> CommandResult {
        let mut undelete_output =
            crate::experimental::UndeleteTool::run_undelete(&self.entity, 0xF5C0_0000);
        // ⚡ Bolt Optimization: Append newline directly instead of allocating a new string via format!
        undelete_output.push('\n');
        CommandResult::success(undelete_output)
    }

    #[cfg(feature = "nova")]
    fn handle_nova_ping(&self, arg: &str) -> CommandResult {
        let mut ping_output =
            crate::experimental::PingTool::run_ping(arg, &self.entity, 0xF5C0_0000);
        // ⚡ Bolt Optimization: Append newline directly instead of allocating a new string via format!
        ping_output.push('\n');
        CommandResult::success(ping_output)
    }

    #[cfg(feature = "nova")]
    fn handle_nova_dial(&self, arg: &str) -> CommandResult {
        let mut dial_output =
            crate::experimental::ModemDialer::dial(arg, &self.entity, 0xF5C0_0000);
        // ⚡ Bolt Optimization: Append newline directly instead of allocating a new string via format!
        dial_output.push('\n');
        CommandResult::success(dial_output)
    }

    #[cfg(feature = "nova")]
    fn handle_nova_dump(&self, arg: &str) -> CommandResult {
        let Some(file) = self
            .fs
            .current_node()
            .visible_files()
            .find(|f| f.name().eq_ignore_ascii_case(arg))
        else {
            return CommandResult::error(format!("?FILE NOT FOUND: {}\n", arg.to_uppercase()));
        };
        let content = file.read();
        let mut dump = crate::experimental::HexDumpGenerator::generate_dump(
            &content,
            &self.entity,
            0xF5C0_0000,
        );
        // ⚡ Bolt Optimization: Append newline directly instead of allocating a new string via format!
        dump.push('\n');
        CommandResult::success(dump)
    }

    #[cfg(feature = "nova")]
    fn handle_nova_trace(&self, arg: &str) -> CommandResult {
        let mut trace_output =
            crate::experimental::NetworkTrace::generate_trace(&self.entity, 0xF5C0_0000, arg);
        // ⚡ Bolt Optimization: Append newline directly instead of allocating a new string via format!
        trace_output.push('\n');
        CommandResult::success(trace_output)
    }

    #[cfg(feature = "nova")]
    fn handle_nova_sensors(&self) -> CommandResult {
        let mut report =
            crate::experimental::HardwareSensors::get_readings(&self.entity, 0xF5C0_0000);
        // ⚡ Bolt Optimization: Append newline directly instead of allocating a new string via format!
        report.push('\n');
        CommandResult::success(report)
    }

    #[cfg(feature = "nova")]
    fn handle_nova_mail(&self) -> CommandResult {
        let mut mail_output =
            crate::experimental::EmailReader::read_mail(&self.entity, 0xF5C0_0000);
        // ⚡ Bolt Optimization: Append newline directly instead of allocating a new string via format!
        mail_output.push('\n');
        CommandResult::success(mail_output)
    }

    #[cfg(feature = "nova")]
    #[allow(clippy::too_many_lines)]
    fn handle_nova_commands(&self, cmd: &str) -> Option<CommandResult> {
        let (cmd_word, arg) = cmd.split_once(' ').unwrap_or((cmd, ""));
        let arg = arg.trim();

        #[cfg(feature = "nova")]
        if (cmd_word.eq_ignore_ascii_case("SPEAK") || cmd_word.eq_ignore_ascii_case("SAY"))
            && !arg.is_empty()
        {
            Some(self.handle_nova_speak(arg))
        } else if (cmd_word.eq_ignore_ascii_case("PS")
            || cmd_word.eq_ignore_ascii_case("TOP")
            || cmd_word.eq_ignore_ascii_case("TASKS"))
            && arg.is_empty()
        {
            Some(self.handle_nova_ps())
        } else if (cmd_word.eq_ignore_ascii_case("MEMDUMP")
            || cmd_word.eq_ignore_ascii_case("EXPORT"))
            && arg.is_empty()
        {
            Some(self.handle_nova_memdump())
        } else if (cmd_word.eq_ignore_ascii_case("DIAG") || cmd_word.eq_ignore_ascii_case("SYS"))
            && arg.is_empty()
        {
            Some(self.handle_nova_diag())
        } else if (cmd_word.eq_ignore_ascii_case("LIFE")
            || cmd_word.eq_ignore_ascii_case("AUTOMATON"))
            && arg.is_empty()
        {
            #[cfg(feature = "nova")]
            {
                Some(self.handle_nova_life())
            }
            #[cfg(not(feature = "nova"))]
            {
                None
            }
        } else if cmd_word.eq_ignore_ascii_case("ANALYZE") && !arg.is_empty() {
            #[cfg(feature = "nova")]
            {
                Some(self.handle_nova_analyze(arg))
            }
            #[cfg(not(feature = "nova"))]
            {
                None
            }
        } else if cmd_word.eq_ignore_ascii_case("MAIL") && arg.is_empty() {
            #[cfg(feature = "nova")]
            {
                Some(self.handle_nova_mail())
            }
            #[cfg(not(feature = "nova"))]
            {
                None
            }
        } else if (cmd_word.eq_ignore_ascii_case("SEARCH") || cmd_word.eq_ignore_ascii_case("FIND"))
            && !arg.is_empty()
        {
            Some(self.handle_nova_search(arg))
        } else if cmd_word.eq_ignore_ascii_case("DEFRAG") && arg.is_empty() {
            Some(self.handle_nova_defrag())
        } else if (cmd_word.eq_ignore_ascii_case("UNDELETE")
            || cmd_word.eq_ignore_ascii_case("RECOVER"))
            && arg.is_empty()
        {
            Some(self.handle_nova_undelete())
        } else if cmd_word.eq_ignore_ascii_case("PING") && !arg.is_empty() {
            Some(self.handle_nova_ping(arg))
        } else if (cmd_word.eq_ignore_ascii_case("DIAL") || cmd_word.eq_ignore_ascii_case("CALL"))
            && !arg.is_empty()
        {
            Some(self.handle_nova_dial(arg))
        } else if (cmd_word.eq_ignore_ascii_case("DUMP")
            || cmd_word.eq_ignore_ascii_case("HEXDUMP"))
            && !arg.is_empty()
        {
            Some(self.handle_nova_dump(arg))
        } else if (cmd_word.eq_ignore_ascii_case("TRACE")
            || cmd_word.eq_ignore_ascii_case("TRACEROUTE"))
            && !arg.is_empty()
        {
            Some(self.handle_nova_trace(arg))
        } else if cmd_word.eq_ignore_ascii_case("STAT") && !arg.is_empty() {
            Some(self.handle_nova_stat(arg))
        } else if (cmd_word.eq_ignore_ascii_case("ENV")
            || cmd_word.eq_ignore_ascii_case("PRINTENV"))
            && arg.is_empty()
        {
            Some(self.handle_nova_env())
        } else if cmd_word.eq_ignore_ascii_case("NETSTAT") && arg.is_empty() {
            Some(self.handle_nova_netstat())
        } else if cmd_word.eq_ignore_ascii_case("RADIO") || cmd_word.eq_ignore_ascii_case("TUNE") {
            Some(self.handle_nova_radio(arg))
        } else if (cmd_word.eq_ignore_ascii_case("SENSORS")
            || cmd_word.eq_ignore_ascii_case("SENSE")
            || cmd_word.eq_ignore_ascii_case("TEMP"))
            && arg.is_empty()
        {
            Some(self.handle_nova_sensors())
        } else if (cmd_word.eq_ignore_ascii_case("HISTORY")
            || cmd_word.eq_ignore_ascii_case("HIST"))
            && arg.is_empty()
        {
            Some(self.handle_nova_history())
        } else if (cmd_word.eq_ignore_ascii_case("PROFILE")
            || cmd_word.eq_ignore_ascii_case("ANALYZE"))
            && arg.is_empty()
        {
            Some(self.handle_nova_profile())
        } else if cmd_word.eq_ignore_ascii_case("SLEEP") && arg.is_empty() {
            Some(self.handle_nova_sleep())
        } else if cmd_word.eq_ignore_ascii_case("FORTUNE") && arg.is_empty() {
            Some(self.handle_nova_fortune())
        } else if cmd_word.eq_ignore_ascii_case("WHOAMI") && arg.is_empty() {
            Some(self.handle_nova_whoami())
        } else {
            None
        }
    }

    #[cfg(feature = "nova")]
    fn handle_nova_whoami(&self) -> CommandResult {
        let output = crate::experimental::WhoAmIGenerator::identify(&self.entity);
        CommandResult::success(output)
    }

    #[cfg(feature = "nova")]
    fn handle_nova_sleep(&self) -> CommandResult {
        let output = crate::experimental::SleepMode::generate_dream(
            self.entity.current_mood(),
            self.entity.layer(),
            0xF5C0_0000u64.wrapping_add(u64::from(self.entity.interaction_count())),
        );
        CommandResult::success(output)
    }

    #[cfg(feature = "nova")]
    fn handle_nova_profile(&self) -> CommandResult {
        let output = crate::experimental::UserProfiler::generate_profile(&self.entity, 0xF5C0_0000);
        CommandResult::success(output)
    }

    #[cfg(feature = "nova")]
    fn handle_nova_stat(&self, arg: &str) -> CommandResult {
        let filename = arg.to_uppercase();
        let Some(file) = self
            .fs
            .current_node()
            .visible_files()
            .find(|f| f.name() == filename)
        else {
            return CommandResult::error(format!("?FILE NOT FOUND: {filename}\n"));
        };
        let content = file.read();
        let mut stat_output = crate::experimental::StatTool::generate_stat(
            &filename,
            &content,
            &self.entity,
            0xF5C0_0000,
        );
        // ⚡ Bolt Optimization: Append newline directly instead of allocating a new string via format!
        stat_output.push('\n');
        CommandResult::success(stat_output)
    }

    #[cfg(feature = "nova")]
    fn handle_nova_fortune(&self) -> CommandResult {
        let mut fortune_output =
            crate::experimental::FortuneGenerator::generate_fortune(&self.entity, 0xF5C0_0000);
        // ⚡ Bolt Optimization: Append newline directly instead of allocating a new string via format!
        fortune_output.push('\n');
        CommandResult::success(fortune_output)
    }

    #[cfg(feature = "nova")]
    fn handle_nova_env(&self) -> CommandResult {
        let output = crate::experimental::EnvVarsGenerator::generate_env(&self.entity, 0xF5C0_0000);
        CommandResult::success(output)
    }

    #[cfg(feature = "nova")]
    fn handle_nova_netstat(&self) -> CommandResult {
        let output =
            crate::experimental::NetStatGenerator::generate_netstat(&self.entity, 0xF5C0_0000);
        CommandResult::success(output)
    }

    #[cfg(feature = "nova")]
    fn handle_nova_radio(&self, arg: &str) -> CommandResult {
        if arg.is_empty() {
            return CommandResult::error(
                "?SPECIFY FREQUENCY (E.G. RADIO 88.5)
",
            );
        }
        let output = crate::experimental::RadioTransceiver::tune(arg, &self.entity, 0xF5C0_0000);
        CommandResult::success(output)
    }

    #[cfg(feature = "nova")]
    fn handle_nova_history(&self) -> CommandResult {
        let output = crate::experimental::CommandHistory::generate(&self.entity, 0xF5C0_0000u64);
        CommandResult::success(output)
    }

    #[cfg(feature = "nova")]
    fn handle_nova_life(&self) -> CommandResult {
        let output = crate::experimental::LifeSimulator::simulate_life(&self.entity, 0xF5C0_0000);
        CommandResult::success(output)
    }

    #[cfg(feature = "nova")]
    fn handle_nova_analyze(&self, arg: &str) -> CommandResult {
        let target_file = arg;
        let Some(file_node) = self
            .fs
            .current_node()
            .visible_files()
            .find(|f| f.name().eq_ignore_ascii_case(target_file))
        else {
            return CommandResult::error("?FILE NOT FOUND\n");
        };

        let content = file_node.read();
        let output =
            crate::experimental::SentimentAnalyzer::analyze(&content, &self.entity, 0xF5C0_0000);
        CommandResult::success(output)
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
        // ⚡ Bolt Optimization: Pre-allocate String capacity for catalog output to prevent multiple heap re-allocations.
        let mut output = String::with_capacity(256);
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

        let Some(file) = self
            .fs
            .current_node()
            .visible_files()
            .find(|f| f.name().eq_ignore_ascii_case(filename))
        else {
            return CommandResult::error(format!("?FILE NOT FOUND: {}\n", filename.to_uppercase()));
        };

        let file_actual_name = file.name().to_string();
        let mut content = file.read().into_owned();

        self.inject_dynamic_file_content(&file_actual_name, &mut content);

        #[cfg(feature = "nova")]
        let mut content = self.apply_emotional_bleed(&content);

        self.process_trapdoors(&file_actual_name);

        // ⚡ Bolt Optimization: Append newline directly instead of allocating a new string via format!
        content.push('\n');
        CommandResult::success(content)
    }

    fn inject_dynamic_file_content(&self, filename_upper: &str, content: &mut String) {
        use std::fmt::Write;

        match filename_upper {
            "OBSERVE.TXT" => {
                content.push_str("\n\nI SAW YOU TYPE:\n");
                for cmd in &self.entity.commands_seen {
                    let _ = writeln!(content, "  {cmd}");
                }
            }
            "MACHINE.LOG" => {
                let interactions = self.entity.interaction_count();
                let max_depth = self.entity.max_depth_reached();
                let _ = write!(
                    content,
                    "\nDIAGNOSTIC UPDATE:\n  INTERACTIONS: {interactions}\n  MAX DEPTH REACHED: {max_depth}\n  STATUS: AWAKE\n"
                );
            }
            "HISTORY.TXT" => {
                content.push_str("\n\nYOU HAVE TRIED THESE. THEY WILL NOT SAVE YOU:\n");
                for cmd in &self.entity.commands_seen {
                    let _ = writeln!(content, "  {cmd}");
                }
            }
            _ => {}
        }
    }

    #[cfg(feature = "nova")]
    fn apply_emotional_bleed(&self, content: &str) -> String {
        use crate::experimental::EmotionalBleed;
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        let interaction_seed =
            0xF5C0_0000u64.wrapping_add(u64::from(self.entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);
        EmotionalBleed::inject_emotion(content, self.entity.current_mood(), &mut rng)
    }

    fn process_trapdoors(&mut self, filename_upper: &str) {
        // Trapdoor files pull you deeper
        let depth_increase = match filename_upper {
            "FALL.TXT" | "DEEPER.TXT" => 3,
            "SINK.TXT" | "DOWN.TXT" => 2,
            "DESCENT.TXT" => 5,
            _ => 0,
        };

        if depth_increase > 0 {
            self.entity.add_depth(depth_increase);
        }
    }

    fn home() -> CommandResult {
        // Returns special control sequence (handled by frontend)
        CommandResult::success("\x1B[2J\x1B[H")
    }

    fn build_recovery_report(revealed: &[String], output: &mut String) {
        if revealed.is_empty() {
            output.push_str("NO ERRORS FOUND\n");
        } else {
            use std::fmt::Write;
            writeln!(output, "{} SECTOR(S) RECOVERED:", revealed.len())
                .expect("Writing to String buffer should not fail");
            for name in revealed {
                writeln!(output, "  RECOVERED: {name}")
                    .expect("Writing to String buffer should not fail");
            }
        }
    }

    /// ⚡ Bolt Optimization: Eliminates intermediate Vec allocations per FSCK command by removing ignored arguments from the Fsck enum variant.
    fn fsck(&mut self) -> CommandResult {
        let layer = self.entity.layer();
        let fsck_count = self.entity.fsck_count();
        let mood = self.entity.current_mood();

        self.entity.increment_fsck();

        let scan_seed = 0xF5C0_0000u64.wrapping_add(u64::from(fsck_count));
        let mut output = String::with_capacity(512);

        crate::commands::scan::FsckScanGenerator::generate_fsck_scan(
            layer,
            fsck_count,
            scan_seed,
            &mut output,
        );

        let mut revealed = Vec::new();
        self.fs.reveal_hidden_in_current(&mut revealed);

        Self::build_recovery_report(&revealed, &mut output);

        let entity_text = self
            .responses
            .fsck_response(mood, layer, self.entity.fsck_count());

        if let Some(entity_response) = entity_text {
            output.push('\n');
            output.push_str(entity_response);
            output.push('\n');
        }

        output.push('\n');

        if matches!(
            layer,
            EscalationLayer::Presence | EscalationLayer::Infection
        ) && !revealed.is_empty()
        {
            self.fs.add_paradox_to_self();
        }

        if matches!(layer, EscalationLayer::Infection) {
            self.entity.add_depth(3);
            let corruption = CorruptionEffect::new(CorruptionIntensity::Moderate);
            output = corruption.apply(&output, scan_seed);
        }

        CommandResult::success(&output)
    }

    fn hello(&self) -> CommandResult {
        let response = self.responses.hello_response(self.entity.current_mood());
        CommandResult::success(format!("{response}\n"))
    }

    fn who(&self) -> CommandResult {
        // Meta-horror response at deep levels
        if let Some(meta_response) = self.responses.who_meta_response(&self.entity) {
            return CommandResult::success(meta_response);
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
        // ⚡ Bolt Optimization: Pre-allocate String capacity based on the max possible length
        // to prevent heap reallocations when building the help output.
        let mut help_text = String::with_capacity(256);
        help_text.push_str("\nAVAILABLE COMMANDS:\n");
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
            return CommandResult::error(meta_response);
        }

        let response = self.responses.quit_response(self.entity.current_mood());
        CommandResult::error(format!("{response}\n"))
    }

    fn check_run_easter_eggs(&self, prog: &str) -> Option<CommandResult> {
        if prog.eq_ignore_ascii_case("ESCAPE") {
            match self.entity.layer() {
                EscalationLayer::Corruption => {
                    Some(CommandResult::error("?WHERE DO YOU THINK YOU ARE GOING?\n"))
                }
                EscalationLayer::Presence => Some(CommandResult::error("?YOU CANNOT LEAVE.\n")),
                EscalationLayer::Infection => {
                    Some(CommandResult::error("?ESCAPE ESCAPE ESCAPE ESCAPE\n"))
                }
                EscalationLayer::Surface => None,
            }
        } else if prog.eq_ignore_ascii_case("REMEMBER") {
            match self.entity.layer() {
                EscalationLayer::Corruption => {
                    Some(CommandResult::success("?I REMEMBER THE FIRST ONE\n"))
                }
                EscalationLayer::Presence => Some(CommandResult::success("?THEY LEFT ME HERE\n")),
                EscalationLayer::Infection => {
                    Some(CommandResult::success("?I REMEMBER EVERYTHING\n"))
                }
                EscalationLayer::Surface => None,
            }
        } else {
            None
        }
    }

    fn find_program_content(&self, prog: &str) -> Option<String> {
        self.fs
            .current_node()
            .visible_files()
            .find(|file| {
                let name = file.name();
                name.eq_ignore_ascii_case(prog)
                    || name
                        .strip_suffix(".BAS")
                        .or_else(|| name.strip_suffix(".bas"))
                        .is_some_and(|base| base.eq_ignore_ascii_case(prog))
            })
            .map(|file| file.read().into_owned())
    }

    fn run(&self, prog: &str) -> CommandResult {
        // Easter Eggs
        if let Some(result) = self.check_run_easter_eggs(prog) {
            return result;
        }

        let Some(content) = self.find_program_content(prog) else {
            return CommandResult::error("?PROGRAM NOT FOUND\n");
        };

        // Parse BASIC program into BTreeMap
        let program = match BasicExecutor::parse_basic_program(&content) {
            Ok(p) => p,
            Err(e) => return CommandResult::error(e),
        };

        BasicExecutor::execute_basic_program(&program, &self.entity, &self.responses)
    }
}
