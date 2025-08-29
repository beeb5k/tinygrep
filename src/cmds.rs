use cli::{Cats, Command};

/// A helper macro to generate a list of [`Command`] definitions at once.
///
/// # Usage
///
/// Instead of writing out an array of `Command` structs manually,
/// you can use this macro to keep the code shorter and more consistent.
///
/// Each entry takes the following parameters in order:
/// - `flag`: `&str` — the long form of the flag (e.g. `"--help"`).
/// - `alias`: `&str` — the short alias (e.g. `"-h"`).
/// - `takes_value`: `bool` — whether the flag expects a value (e.g. `true` for `--file <path>`).
/// - `desc`: `&str` — a human-readable description of the flag.
/// - `cat`: [`Cats`] — the category of the command.
///
/// Entries are separated by semicolons (`;`).  
/// A trailing semicolon is allowed.
///
/// # Example
/// ```
/// use cli::{Cats, Command};
///
/// let commands = make_commands![
///     "--file", "-f", true, "Read patterns from FILE", Cats::Interpretation;
///     "--pattern", "-p", false, "Use the given PATTERN", Cats::Pattern;
///     "--help", "-h", false, "Display help text", Cats::Miscellaneous;
/// ];
/// ```
///
/// This expands into a `&[Command]` slice that you can use at runtime.
macro_rules! make_commands {
    ($($flag:expr, $alias:expr, $takes_value:expr, $desc:expr, $cat:expr);* $(;)?) => {
        &[
            $(Command { flag: $flag, aliase: $alias, takes_value: $takes_value, description: $desc, cat: $cat }),*
        ]
    };
}

pub const COMMANDS: &[Command] = make_commands![
    "--file", "-f", true, "PATTERNS from FILE", Cats::Interpretation;
    "--pattern", "-p", true, "PATTERNS for matching", Cats::Pattern;
    "--help", "-h", false, "display this help text and exit", Cats::Miscellaneous;
    "--invert-match", "-im", false, "select non-matching lines", Cats::Miscellaneous;
    "--version", "-v", false, "display version information and exit", Cats::Miscellaneous;
    "--ignore-case", "-ic", false, "ignore case distinctions in patterns and data", Cats::Pattern;
];
