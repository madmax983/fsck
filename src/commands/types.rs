/// Result of executing a command
#[derive(Debug, Clone)]
pub struct CommandResult {
    output: String,
    is_error: bool,
}

impl CommandResult {
    pub fn success(output: &str) -> Self {
        Self {
            output: output.to_string(),
            is_error: false,
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            output: message.to_string(),
            is_error: true,
        }
    }

    pub fn output(&self) -> &str {
        &self.output
    }

    pub fn is_error(&self) -> bool {
        self.is_error
    }
}

/// Commands recognized by the system
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    /// List directory contents (CATALOG, DIR, LS)
    Catalog,
    /// Change directory (CD, CHDIR)
    ChangeDir(String),
    /// Display file contents (TYPE, CAT)
    Type(String),
    /// Run a BASIC program
    Run(String),
    /// Clear screen
    Home,
    /// The namesake - filesystem check
    Fsck(Vec<String>),
    /// Say hello to the machine
    Hello,
    /// Ask who/what is here
    Who,
    /// Request help
    Help,
    /// View command history
    History,
    /// Attempt to quit
    Quit,
    /// Unknown command
    Unknown(String),
}

impl Command {
    pub fn from_input(command: &str, args: &[String]) -> Self {
        match command {
            "CATALOG" | "DIR" | "LS" => Command::Catalog,
            "CD" | "CHDIR" => {
                let path = args.first().cloned().unwrap_or_default();
                Command::ChangeDir(path)
            }
            "TYPE" | "CAT" => {
                let file = args.first().cloned().unwrap_or_default();
                Command::Type(file)
            }
            "RUN" => {
                let prog = args.first().cloned().unwrap_or_default();
                Command::Run(prog)
            }
            "HOME" | "CLS" | "CLEAR" => Command::Home,
            "FSCK" => Command::Fsck(args.to_vec()),
            "HELLO" | "HI" => Command::Hello,
            "WHO" | "WHOAMI" => Command::Who,
            "HELP" | "?" => Command::Help,
            "HISTORY" | "REMEMBRANCE" => Command::History,
            "QUIT" | "EXIT" | "BYE" => Command::Quit,
            "" => Command::Unknown(String::new()),
            other => Command::Unknown(other.to_string()),
        }
    }
}
