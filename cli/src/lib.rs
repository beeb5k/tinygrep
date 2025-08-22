/// Represents a command-line flag.
///
/// A `Command` defines one possible flag your program accepts,
/// including its primary name, aliases, whether it expects a value,
/// and a short description for help messages.
pub struct Command {
    /// The primary name of the flag (e.g., `"--help"`).
    pub flag: &'static str,

    /// Alternative names (aliases) for the flag (e.g., `["-h"]`).
    pub aliases: Vec<&'static str>,

    /// Indicates whether this flag expects a value.
    ///
    /// If `true`, the flag should be followed by an argument
    /// (e.g., `"--file config.toml"`). If `false`, it is a standalone flag.
    pub value: bool,

    /// A short description of the flag, for help messages.
    pub description: &'static str,
}

impl Command {
    /// Validates the provided command-line arguments against a list of known commands.
    pub fn validate(commands: &[Command], args: &[String]) {
        for arg in args {
            if arg.starts_with('-') {
                let valid = commands
                    .iter()
                    .any(|cmd| cmd.flag == *arg || cmd.aliases.contains(&arg.as_str()));

                if !valid {
                    eprintln!("invalid flag: {arg}");
                }
            }
        }
    }
}

