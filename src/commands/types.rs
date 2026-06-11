use std::borrow::Cow;

/// Result of executing a command
#[derive(Debug, Clone)]
pub struct CommandResult {
    output: Cow<'static, str>,
    is_error: bool,
}

impl CommandResult {
    #[must_use]
    pub fn success(output: impl Into<Cow<'static, str>>) -> Self {
        Self {
            output: output.into(),
            is_error: false,
        }
    }

    #[must_use]
    pub fn error(message: impl Into<Cow<'static, str>>) -> Self {
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
        self.output.into_owned()
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
        if command.eq_ignore_ascii_case("CATALOG")
            || command.eq_ignore_ascii_case("DIR")
            || command.eq_ignore_ascii_case("LS")
        {
            Self::Catalog
        } else if command.eq_ignore_ascii_case("CD") || command.eq_ignore_ascii_case("CHDIR") {
            let path = args.next().map(str::to_uppercase).unwrap_or_default();
            Self::ChangeDir(path)
        } else if command.eq_ignore_ascii_case("TYPE") || command.eq_ignore_ascii_case("CAT") {
            let file = args.next().map(str::to_uppercase).unwrap_or_default();
            Self::Type(file)
        } else if command.eq_ignore_ascii_case("RUN") {
            let prog = args.next().map(str::to_uppercase).unwrap_or_default();
            Self::Run(prog)
        } else if command.eq_ignore_ascii_case("HOME")
            || command.eq_ignore_ascii_case("CLS")
            || command.eq_ignore_ascii_case("CLEAR")
        {
            Self::Home
        } else if command.eq_ignore_ascii_case("FSCK") {
            Self::Fsck
        } else if command.eq_ignore_ascii_case("HELLO") || command.eq_ignore_ascii_case("HI") {
            Self::Hello
        } else if command.eq_ignore_ascii_case("WHO") || command.eq_ignore_ascii_case("WHOAMI") {
            Self::Who
        } else if command.eq_ignore_ascii_case("HELP") || command.eq_ignore_ascii_case("?") {
            Self::Help
        } else if command.eq_ignore_ascii_case("QUIT")
            || command.eq_ignore_ascii_case("EXIT")
            || command.eq_ignore_ascii_case("BYE")
        {
            Self::Quit
        } else if command.is_empty() {
            Self::Unknown(String::new())
        } else {
            Self::Unknown(command.to_uppercase())
        }
    }
}
