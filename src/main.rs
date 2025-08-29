mod cmds;

use crate::cmds::COMMANDS;
use cli::Command;
use grep::PatternMatcher;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    match Command::parse_args(COMMANDS, &args[1..]) {
        Ok(pargs) => {
            if pargs.contains_key("--help") {
                Command::print_help(COMMANDS, &args[0]);
                return Ok(());
            }

            if pargs.contains_key("--version") {
                const VERSION: &str = env!("CARGO_PKG_VERSION");
                println!("{VERSION}");
                return Ok(());
            }

            let mut files = pargs
                .get("--file")
                .and_then(|opt| opt.as_ref().cloned())
                .unwrap_or_default();

            let mut patterns = pargs
                .get("--pattern")
                .and_then(|opt| opt.as_ref().cloned())
                .unwrap_or_default();

            // fallback to positional args
            if patterns.is_empty() && !args[1..].is_empty() {
                patterns.push(args[1].clone());
            }

            if files.is_empty() && args.len() > 2 {
                files.extend_from_slice(&args[2..]);
            }

            let matcher = PatternMatcher::new(&patterns, &files)
                .case_sensitive(!pargs.contains_key("--ignore-case"))
                .invert_match(pargs.contains_key("--invert-match"));

            let matches = matcher.find_matches()?;
            for m in &matches {
                println!("{}", m.line);
            }
        }
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }

    Ok(())
}
