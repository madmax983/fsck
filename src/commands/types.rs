/// Result of executing a command
#[derive(Debug, Clone)]
pub struct CommandResult {
    output: String,
    is_error: bool,
}

impl CommandResult {
    #[must_use]
    pub fn success(output: impl Into<String>) -> Self {
        Self {
            output: output.into(),
            is_error: false,
        }
    }

    #[must_use]
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            output: message.into(),
            is_error: true,
        }
    }

    #[must_use]
    pub fn output(&self) -> &str {
        &self.output
    }

    #[must_use]
    pub const fn is_error(&self) -> bool {
        self.is_error
    }
}

/// Commands recognized by the system
#[derive(Debug, Clone, PartialEq, Eq)]
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
    /// Attempt to quit
    Quit,
    /// Unknown command
    Unknown(String),
}

impl Command {
    #[must_use]
    pub fn from_input(command: &str, args: &[&str]) -> Self {
        match command.to_uppercase().as_str() {
            "CATALOG" | "DIR" | "LS" => Self::Catalog,
            "CD" | "CHDIR" => {
                let path = args.first().map(|s| s.to_uppercase()).unwrap_or_default();
                Self::ChangeDir(path)
            }
            "TYPE" | "CAT" => {
                let file = args.first().map(|s| s.to_uppercase()).unwrap_or_default();
                Self::Type(file)
            }
            "RUN" => {
                let prog = args.first().map(|s| s.to_uppercase()).unwrap_or_default();
                Self::Run(prog)
            }
            "HOME" | "CLS" | "CLEAR" => Self::Home,
            "FSCK" => Self::Fsck(args.iter().map(|s| s.to_uppercase()).collect()),
            "HELLO" | "HI" => Self::Hello,
            "WHO" | "WHOAMI" => Self::Who,
            "HELP" | "?" => Self::Help,
            "QUIT" | "EXIT" | "BYE" => Self::Quit,
            "" => Self::Unknown(String::new()),
            _ => Self::Unknown(command.to_uppercase()),
        }
    }
}
