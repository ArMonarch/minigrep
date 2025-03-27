use std::{
    io::{Read, Write},
    process::ExitCode,
};

mod args;
mod docs;
mod flags;
mod parse;

fn main() -> ExitCode {
    let args = flags::parse();

    match run(args) {
        Ok(exit_code) => exit_code,
        Err(err) => {
            eprintln!("error: {:#}", err);
            ExitCode::from(1)
        }
    }
}

/// The main entry point for minigerp
fn run(args: parse::ParseResult<args::Args>) -> anyhow::Result<ExitCode> {
    use args::Mode;
    use parse::ParseResult;

    let mut args = match args {
        ParseResult::Err(err) => return Err(err),
        ParseResult::Special(mode) => return special(mode),
        ParseResult::Ok(args) => args,
    };

    let matched = match args.mode {
        Mode::Search(_) if !args.matches_possible() => false,
        Mode::Search(mode) => search(&mut args, mode)?,
    };

    Ok(if matched {
        ExitCode::from(0)
    } else {
        anyhow::bail!("pattern not matched in the file")
    })
}

fn search(args: &mut args::Args, _mode: args::Searchmode) -> anyhow::Result<bool> {
    let file = args.get_file()?;
    let patterns = args.get_patterns()?;

    // print the file name
    println!("{}", file);

    let mut file = std::fs::File::open(file)?;

    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;

    for pattern in &patterns {
        for (index, line) in file_content.lines().enumerate() {
            if line.contains(pattern) {
                println!("{}: {}", index + 1, line);
            }
        }
    }

    Ok(true)
}

/// Implements minigrep's "special" modes.
pub fn special(special_mode: args::SpecialMode) -> anyhow::Result<ExitCode> {
    use args::SpecialMode;

    let exit_code = ExitCode::from(0);

    let output = match special_mode {
        SpecialMode::HelpShort => docs::generate_help_short(),
        SpecialMode::HelpLong => docs::generate_help_long(),
        SpecialMode::VersionShort => docs::generate_version_short(),
        SpecialMode::VersionLong => docs::generate_version_long(),
    };

    writeln!(std::io::stdout(), "{}", output)?;

    Ok(exit_code)
}
