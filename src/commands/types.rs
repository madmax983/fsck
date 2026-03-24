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

    /// ⚡ Bolt Optimization: Consumes the `CommandResult` to return the owned `String`, avoiding a `.to_string()` heap allocation.
    #[must_use]
    pub fn into_output(self) -> String {
        self.output
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
    /// ⚡ Bolt Optimization: Uses an Iterator `impl Iterator<Item = &'a str>` instead of a slice `&[&str]`
    /// for command arguments, avoiding the need to collect arguments into a temporary vector first.
    #[must_use]
    pub fn from_input<'a>(command: &str, mut args: impl Iterator<Item = &'a str>) -> Self {
        match command.to_uppercase().as_str() {
            "CATALOG" | "DIR" | "LS" => Self::Catalog,
            "CD" | "CHDIR" => {
                let path = args.next().map(str::to_uppercase).unwrap_or_default();
                Self::ChangeDir(path)
            }
            "TYPE" | "CAT" => {
                let file = args.next().map(str::to_uppercase).unwrap_or_default();
                Self::Type(file)
            }
            "RUN" => {
                let prog = args.next().map(str::to_uppercase).unwrap_or_default();
                Self::Run(prog)
            }
            "HOME" | "CLS" | "CLEAR" => Self::Home,
            "FSCK" => Self::Fsck(args.map(str::to_uppercase).collect()),
            "HELLO" | "HI" => Self::Hello,
            "WHO" | "WHOAMI" => Self::Who,
            "HELP" | "?" => Self::Help,
            "QUIT" | "EXIT" | "BYE" => Self::Quit,
            "" => Self::Unknown(String::new()),
            _ => Self::Unknown(command.to_uppercase()),
        }
    }
}
