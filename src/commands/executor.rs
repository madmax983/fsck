use super::types::{Command, CommandResult};
use crate::entity::{Entity, ResponseGenerator};
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

        for file in self.fs.current_node().files() {
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

    fn type_file(&self, filename: &str) -> CommandResult {
        if filename.is_empty() {
            return CommandResult::error("?SYNTAX ERROR\n");
        }

        let filename_upper = filename.to_uppercase();
        for file in self.fs.current_node().files() {
            if file.name() == filename_upper {
                return CommandResult::success(&format!("{}\n", file.content()));
            }
        }

        CommandResult::error(&format!("?FILE NOT FOUND: {}\n", filename_upper))
    }

    fn home(&self) -> CommandResult {
        // Returns special control sequence (handled by frontend)
        CommandResult::success("\x1B[2J\x1B[H")
    }

    fn fsck(&self, _args: &[String]) -> CommandResult {
        // TODO: Implement fsck logic
        CommandResult::success("CHECKING DISK...\n\nNO ERRORS FOUND\n\n")
    }

    fn hello(&self) -> CommandResult {
        let response = self.responses.hello_response(self.entity.current_mood());
        CommandResult::success(&format!("{}\n", response))
    }

    fn who(&self) -> CommandResult {
        let response = self
            .responses
            .who_response(self.entity.current_mood(), None);
        CommandResult::success(&format!("{}\n", response))
    }

    fn help(&self) -> CommandResult {
        CommandResult::success(concat!(
            "\nAVAILABLE COMMANDS:\n",
            "  CATALOG  - LIST FILES\n",
            "  CD       - CHANGE DIRECTORY\n",
            "  TYPE     - DISPLAY FILE\n",
            "  HOME     - CLEAR SCREEN\n",
            "  FSCK     - CHECK FILESYSTEM\n",
            "\n"
        ))
    }

    fn quit(&self) -> CommandResult {
        let response = self.responses.quit_response(self.entity.current_mood());
        CommandResult::error(&format!("{}\n", response))
    }

    fn run(&self, _prog: &str) -> CommandResult {
        // TODO: BASIC interpreter
        CommandResult::error("?PROGRAM NOT FOUND\n")
    }
}
