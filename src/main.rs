use std::process::ExitCode;

mod args;
mod flags;
mod parse;

fn main() -> ExitCode {
    let args = flags::parse();

    match run(args) {
        Ok(exit_code) => exit_code,
        Err(err) => {
            eprintln!("{:#}", err);
            ExitCode::from(2)
        }
    }
}

/// The main entry point for minigerp
fn run(args: parse::ParseResult<args::Args>) -> anyhow::Result<ExitCode> {
    use parse::ParseResult;

    let _args = match args {
        ParseResult::Err(err) => return Err(err),
        ParseResult::Special(mode) => return special(mode),
        ParseResult::Ok(args) => args,
    };

    return Ok(ExitCode::SUCCESS);
}

fn special(_special_mode: args::SpecialMode) -> anyhow::Result<ExitCode> {
    Ok(ExitCode::from(0))
}
