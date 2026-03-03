use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

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

impl CommandExecutor {
    pub fn new(fs: FilesystemGraph, entity: Entity) -> Self {
        Self {
            fs,
            entity,
            responses: ResponseGenerator::new(),
        }
    }

    pub fn execute(&mut self, command: Command) -> CommandResult {
        // Record the interaction
        self.entity.record_command(&format!("{:?}", command));

        match command {
            Command::Catalog => self.catalog(),
            Command::ChangeDir(path) => self.change_dir(&path),
            Command::Type(file) => self.type_file(&file),
            Command::Home => self.home(),
            Command::Fsck(args) => self.fsck(&args),
            Command::Hello => self.hello(),
            Command::Who => self.who(),
            Command::Help => self.help(),
            Command::Quit => self.quit(),
            Command::Run(prog) => self.run(&prog),
            Command::Unknown(cmd) => {
                if cmd.is_empty() {
                    CommandResult::success("")
                } else {
                    CommandResult::error(&format!("?SYNTAX ERROR: {}\n", cmd))
                }
            }
        }
    }

    pub fn current_path(&self) -> String {
        self.fs.current_path()
    }

    pub fn entity(&self) -> &Entity {
        &self.entity
    }

    fn catalog(&self) -> CommandResult {
        let mut output = String::new();
        output.push_str("\nDISK VOLUME 254\n\n");

        for dir in self.fs.list_directories() {
            output.push_str(&format!(" *{:<15} DIR\n", dir));
        }

        for file in self.fs.current_node().visible_files() {
            output.push_str(&format!("  {:<15} TXT\n", file.name()));
        }

        output.push('\n');

        // Maybe add an interjection
        if self.responses.should_interject(&self.entity) {
            if let Some(interjection) = self
                .responses
                .random_interjection(self.entity.current_mood())
            {
                output.push_str(&format!("\n{}\n", interjection));
            }
        }

        #[cfg(feature = "nova")]
        {
            if let Some(audio_hint) = crate::experimental::SpatialAudioGenerator::generate_anomaly(
                &self.entity,
                0xF5C0_0000,
            ) {
                output.push_str(&format!("\n{}\n", audio_hint));
            }
        }

        CommandResult::success(&output)
    }

    fn change_dir(&mut self, path: &str) -> CommandResult {
        if path.is_empty() {
            return CommandResult::error("?SYNTAX ERROR\n");
        }

        match self.fs.change_dir(path) {
            Ok(()) => {
                // Update entity with new depth
                self.entity.update_depth(self.fs.current_depth());
                CommandResult::success("")
            }
            Err(e) => CommandResult::error(&format!("?{}\n", e.to_string().to_uppercase())),
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

                return CommandResult::success(&format!("{}\n", content));
            }
        }

        CommandResult::error(&format!("?FILE NOT FOUND: {}\n", filename_upper))
    }

    fn home(&self) -> CommandResult {
        // Returns special control sequence (handled by frontend)
        CommandResult::success("\x1B[2J\x1B[H")
    }

    fn fsck(&mut self, _args: &[String]) -> CommandResult {
        let layer = self.entity.layer();
        let fsck_count = self.entity.fsck_count();
        let mood = self.entity.current_mood();

        // 1. Record the fsck use (applies depth pressure every 3rd)
        self.entity.increment_fsck();

        // 2. Generate deterministic scan output
        let scan_seed = 0xF5C0_0000u64.wrapping_add(u64::from(fsck_count));
        let scan_output = Self::generate_fsck_scan(layer, fsck_count, scan_seed);

        // 3. Reveal hidden content in current directory
        let revealed = self.fs.reveal_hidden_in_current();

        // 4. Build recovery report
        let recovery = if revealed.is_empty() {
            "NO ERRORS FOUND\n".to_string()
        } else {
            let mut report = format!("{} SECTOR(S) RECOVERED:\n", revealed.len());
            for name in &revealed {
                report.push_str(&format!("  RECOVERED: {}\n", name));
            }
            report
        };

        // 5. Entity resistance
        let entity_text = self
            .responses
            .fsck_response(mood, layer, self.entity.fsck_count());

        // 6. At Presence+: add paradox to current dir (fixes come back worse)
        if matches!(
            layer,
            EscalationLayer::Presence | EscalationLayer::Infection
        ) && !revealed.is_empty()
        {
            self.fs.add_paradox_to_self();
        }

        // 7. At Infection: extra depth pressure and corrupt the output
        if matches!(layer, EscalationLayer::Infection) {
            self.entity.add_depth(3);
        }

        // 8. Assemble final output
        let mut output = scan_output;
        output.push_str(&recovery);

        if let Some(entity_response) = entity_text {
            output.push('\n');
            output.push_str(&entity_response);
            output.push('\n');
        }

        output.push('\n');

        // At Infection: corrupt the entire output
        if matches!(layer, EscalationLayer::Infection) {
            let corruption = CorruptionEffect::new(CorruptionIntensity::Moderate);
            output = corruption.apply(&output, scan_seed);
        }

        CommandResult::success(&output)
    }

    /// Generate sector scan output appropriate to the current layer
    fn generate_fsck_scan(layer: EscalationLayer, fsck_count: u32, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut output = String::new();

        output.push_str("CHECKING DISK...\n\n");

        match layer {
            EscalationLayer::Surface => {
                // Clean, normal disk check
                let total_sectors = 560;
                output.push_str(&format!("READING {} SECTORS\n", total_sectors));
                output.push_str("SECTOR 0000-022F: OK\n");
                output.push_str("VTOC: OK\n");
                output.push_str("CATALOG: OK\n\n");
            }
            EscalationLayer::Corruption => {
                // Errors appear, numbers don't add up
                let total_sectors = 560 + rng.gen_range(0..100);
                let bad_sectors = rng.gen_range(1..=3);
                output.push_str(&format!("READING {} SECTORS\n", total_sectors));
                output.push_str("SECTOR 0000-00FF: OK\n");
                output.push_str(&format!("SECTOR 0100-01FF: {} ERROR(S)\n", bad_sectors));
                output.push_str("SECTOR 0200-022F: OK\n");
                if fsck_count > 1 {
                    output.push_str("SECTOR 0100-01FF: SCAN LOOP DETECTED\n");
                }
                output.push_str("VTOC: MISMATCH\n\n");
            }
            EscalationLayer::Presence => {
                // Entity interjects mid-scan
                let total_sectors = rng.gen_range(400..700);
                output.push_str(&format!("READING {} SECTORS\n", total_sectors));
                output.push_str("SECTOR 0000-00FF: OK\n");
                output.push_str("SECTOR 0100-01FF: ACCESS DENIED\n");
                output.push_str("SECTOR 0200-02FF: CONFLICTING RESULTS\n");
                output.push_str("SECTOR 0300-03FF: SECTOR RESISTS READ\n");
                output.push_str(&format!(
                    "VTOC: {} ENTRIES (EXPECTED 256)\n\n",
                    rng.gen_range(1..=1024)
                ));
            }
            EscalationLayer::Infection => {
                // Heavily corrupted scan
                let total_sectors = rng.gen_range(0..=99999);
                output.push_str(&format!("READING {} SECTORS\n", total_sectors));
                output.push_str("SECTOR 0000-????: ?????\n");
                output.push_str("SECTOR ????-????: CANNOT\n");
                output.push_str("VTOC: VTOC: VTOC: VTOC:\n\n");
            }
        }

        output
    }

    fn hello(&self) -> CommandResult {
        let response = self.responses.hello_response(self.entity.current_mood());
        CommandResult::success(&format!("{}\n", response))
    }

    fn who(&self) -> CommandResult {
        // Meta-horror response at deep levels
        if let Some(meta_response) = self.responses.who_meta_response(&self.entity) {
            return CommandResult::success(&meta_response);
        }

        let response = self
            .responses
            .who_response(self.entity.current_mood(), None);
        CommandResult::success(&format!("{}\n", response))
    }

    fn help(&self) -> CommandResult {
        let layer = self.entity.layer();

        // Deep layers get meta-horror response
        if let Some(meta_response) = self.responses.help_meta_response(&self.entity) {
            return CommandResult::success(&format!("{}\n", meta_response));
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
        CommandResult::error(&format!("{}\n", response))
    }

    fn run(&self, _prog: &str) -> CommandResult {
        // TODO: BASIC interpreter
        CommandResult::error("?PROGRAM NOT FOUND\n")
    }
}
