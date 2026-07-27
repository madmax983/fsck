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
    Fsck,
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
    /// Also uses `.eq_ignore_ascii_case()` to avoid allocating a new `String` via `to_uppercase()` for command matching.
    #[must_use]
    pub fn from_input<'a>(command: &str, mut args: impl Iterator<Item = &'a str>) -> Self {
        if command.is_empty() {
            return Self::Unknown(String::new());
        }

        match command {
            c if c.eq_ignore_ascii_case("CATALOG")
                || c.eq_ignore_ascii_case("DIR")
                || c.eq_ignore_ascii_case("LS") =>
            {
                Self::Catalog
            }
            c if c.eq_ignore_ascii_case("CD") || c.eq_ignore_ascii_case("CHDIR") => {
                let path = args.next().map(str::to_uppercase).unwrap_or_default();
                Self::ChangeDir(path)
            }
            c if c.eq_ignore_ascii_case("TYPE") || c.eq_ignore_ascii_case("CAT") => {
                let file = args.next().map(str::to_uppercase).unwrap_or_default();
                Self::Type(file)
            }
            c if c.eq_ignore_ascii_case("RUN") => {
                let prog = args.next().map(str::to_uppercase).unwrap_or_default();
                Self::Run(prog)
            }
            c if c.eq_ignore_ascii_case("HOME")
                || c.eq_ignore_ascii_case("CLS")
                || c.eq_ignore_ascii_case("CLEAR") =>
            {
                Self::Home
            }
            c if c.eq_ignore_ascii_case("FSCK") => Self::Fsck,
            c if c.eq_ignore_ascii_case("HELLO") || c.eq_ignore_ascii_case("HI") => Self::Hello,
            c if c.eq_ignore_ascii_case("WHO") || c.eq_ignore_ascii_case("WHOAMI") => Self::Who,
            c if c.eq_ignore_ascii_case("HELP") || c.eq_ignore_ascii_case("?") => Self::Help,
            c if c.eq_ignore_ascii_case("QUIT")
                || c.eq_ignore_ascii_case("EXIT")
                || c.eq_ignore_ascii_case("BYE") =>
            {
                Self::Quit
            }
            _ => Self::Unknown(command.to_uppercase()),
        }
    }
}
