use std::collections::HashMap;
use std::fmt;

#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Debug, Clone)]
pub enum Cats {
    Pattern,
    Interpretation,
    Miscellaneous,
    Output,
    Context,
}

impl fmt::Display for Cats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Cats::Miscellaneous => "Miscellaneous",
            Cats::Pattern => "Pattern",
            Cats::Interpretation => "Interpretation",
            Cats::Output => "Output",
            Cats::Context => "Context",
        };
        write!(f, "{name}")
    }
}

/// Represents a command-line flag.
///
/// A `Command` defines one possible flag your program accepts,
/// including its primary name, aliases, whether it expects a value,
/// and a short description for help messages.
#[derive(Debug)]
pub struct Command {
    /// The primary name of the flag (e.g., `"--help"`).
    pub flag: &'static str,

    /// Alternative names (aliases) for the flag (e.g., `["-h"]`).
    pub aliase: &'static str,

    /// Indicates whether this flag expects a value.
    ///
    /// If `true`, the flag should be followed by an argument
    /// (e.g., `"--file config.toml"`). If `false`, it is a standalone flag.
    pub takes_value: bool,

    /// A short description of the flag, for help messages.
    pub description: &'static str,

    /// Category command belongs to.
    pub cat: Cats,
}

/// Represents the parsed value of a flag.
#[derive(Debug, PartialEq)]
pub enum FlagValue {
    /// A boolean flag (present or not).
    Bool(bool),

    /// A flag that accepts a string value.
    Str(Option<String>),
}

/// Errors that can occur while parsing CLI arguments.
#[derive(Debug, PartialEq)]
pub enum CliError {
    UnknownFlag(String),
    MissingValue(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::UnknownFlag(flag) => write!(f, "unknown flag: {flag}"),
            CliError::MissingValue(flag) => write!(f, "missing value for flag {flag}"),
        }
    }
}

/// A map of parsed command-line flags to their values.
pub type ParsedArgs = HashMap<String, Option<Vec<String>>>;
impl Command {
    /// Prints all the commands to stdout
    pub fn print_help(commands: &[Command], bin: &str) {
        let mut final_output = String::new();
        let mut grouped: HashMap<Cats, Vec<&Command>> = HashMap::new();

        for cmd in commands {
            grouped.entry(cmd.cat.clone()).or_default().push(cmd);
        }

        let mut categories: Vec<_> = grouped.keys().collect();
        categories.sort();

        final_output.push_str(&format!("Usage: {bin} [OPTIONS] PATTERNS [FILES]\n\n"));

        for cat in categories {
            if let Some(cmds) = grouped.get(cat) {
                final_output.push_str(&format!("{cat}:\n"));
                for c in cmds {
                    final_output.push_str(&format!(
                        "  {:<3} {:<12} {}\n",
                        c.aliase, c.flag, c.description
                    ));
                }
            }
        }

        println!("{final_output}");
    }

    /// Parse the given command-line arguments according to the provided commands.
    pub fn parse_args(commands: &[Command], args: &[String]) -> Result<ParsedArgs, CliError> {
        let mut parsed: ParsedArgs = HashMap::new();
        let mut iter = args.iter().peekable();

        while let Some(arg) = iter.next() {
            if arg.starts_with('-') {
                let command = commands.iter().find(|c| c.flag == arg || c.aliase == arg);

                match command {
                    Some(cmd) => Self::handle_command(cmd, &mut iter, &mut parsed)?,
                    None => return Err(CliError::UnknownFlag(arg.clone())),
                }
            }
        }

        Ok(parsed)
    }

    /// Handle a recognized flag by either inserting a boolean or extracting its value.
    fn handle_command<'a, I>(
        cmd: &Command,
        iter: &mut std::iter::Peekable<I>,
        parsed: &mut ParsedArgs,
    ) -> Result<(), CliError>
    where
        I: Iterator<Item = &'a String>,
    {
        if cmd.takes_value {
            let val = Self::extract_value(cmd, iter)?;
            parsed.entry(cmd.flag.to_string()).or_default().get_or_insert_default().push(val);
        } else {
            parsed.entry(cmd.flag.to_string()).or_insert(None);
        }

        Ok(())
    }

    /// Extracts the value for a flag that requires one.
    fn extract_value<'a, I>(
        cmd: &Command,
        iter: &mut std::iter::Peekable<I>,
    ) -> Result<String, CliError>
    where
        I: Iterator<Item = &'a String>,
    {
        match iter.peek() {
            Some(next) if !next.starts_with('-') => Ok(iter.next().unwrap().to_string()),
            _ => Err(CliError::MissingValue(cmd.flag.to_string())),
        }
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//
//     const CMDS: [Command; 3] = [
//         Command {
//             flag: "--file",
//             aliase: "-f",
//             takes_value: true,
//             description: "File",
//             cat: Cats::Interpretation,
//         },
//         Command {
//             flag: "--help",
//             aliase: "-h",
//             takes_value: false,
//             description: "Show help",
//             cat: Cats::Miscellaneous,
//         },
//         Command {
//             flag: "--pattern",
//             aliase: "-p",
//             takes_value: true,
//             description: "Search pattern",
//             cat: Cats::Pattern,
//         },
//     ];
//
//     fn s(x: &str) -> String {
//         x.to_string()
//     }
//
//     #[test]
//     fn test_parse_single_flag_with_value() {
//         let args = vec![s("-f"), s("cats.txt")];
//         let parsed = Command::parse_args(&CMDS, &args).unwrap();
//
//         assert_eq!(
//             parsed.get("--file"),
//             Some(&FlagValue::Str(Some("cats.txt".into())))
//         );
//     }
//
//     #[test]
//     fn test_parse_multiple_flags() {
//         let args = vec![s("-f"), s("dogs.txt"), s("--pattern"), s("orange car")];
//         let parsed = Command::parse_args(&CMDS, &args).unwrap();
//
//         assert_eq!(
//             parsed.get("--file"),
//             Some(&FlagValue::Str(Some("dogs.txt".into())))
//         );
//         assert_eq!(
//             parsed.get("--pattern"),
//             Some(&FlagValue::Str(Some("orange car".into())))
//         );
//     }
//
//     #[test]
//     fn test_parse_boolean_flag() {
//         let args = vec![s("-h")];
//         let parsed = Command::parse_args(&CMDS, &args).unwrap();
//
//         assert_eq!(parsed.get("--help"), Some(&FlagValue::Bool(true)));
//     }
//
//     #[test]
//     fn test_missing_value() {
//         let args = vec![s("-f")];
//         let res = Command::parse_args(&CMDS, &args);
//         assert_eq!(res, Err(CliError::MissingValue("--file".into())));
//     }
//
//     #[test]
//     fn test_unknown_flag() {
//         let args = vec![s("--unknown")];
//         let res = Command::parse_args(&CMDS, &args);
//         assert_eq!(res, Err(CliError::UnknownFlag("--unknown".into())));
//     }
// }
