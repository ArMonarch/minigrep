use std::ffi::OsString;
use std::process::ExitCode;

use anyhow::Context;

use crate::args;
use crate::flags::{FlagInfo, FlagInfoKind, FlagLookUp, FlagMap, FlagName, FlagValue};

#[derive(Debug)]
pub enum ParseResult<T> {
    Special(args::SpecialMode),
    Ok(T),
    Err(anyhow::Error),
}

#[derive(Debug)]
pub struct Parser {
    map: FlagMap,
    infos: Vec<FlagInfo>,
}

impl Parser {
    pub fn new() -> &'static Parser {
        use crate::flags::FLAGS;
        use std::sync::OnceLock;

        static P: OnceLock<Parser> = OnceLock::new();

        P.get_or_init(|| {
            let mut infos = Vec::new();

            for &flag in FLAGS.iter() {
                // flag with name_long
                infos.push(FlagInfo {
                    flag,
                    name: FlagName::String(flag.name_long()),
                    _flag_kind: FlagInfoKind::Standard,
                });

                // flag with name_short
                if let Some(ch) = flag.name_short() {
                    infos.push(FlagInfo {
                        flag,
                        name: FlagName::from(ch),
                        _flag_kind: FlagInfoKind::Standard,
                    });
                }
            }

            let map = FlagMap::new(&infos);

            Parser { map, infos }
        })
    }

    pub fn parse<I, O>(&self, rawargs: I, args: &mut args::Args) -> anyhow::Result<ExitCode>
    where
        I: IntoIterator<Item = O>,
        O: Into<OsString>,
    {
        let mut p = lexopt::Parser::from_args(rawargs);

        while let Some(arg) = p.next().context("invalid CLI argument")? {
            let lookup = match arg {
                lexopt::Arg::Value(value) => {
                    match value.into_string() {
                        Ok(value) => args.positional.push(value),
                        Err(_) => anyhow::bail!("failed to convert OsString to String"),
                    };
                    continue;
                }
                lexopt::Arg::Short(ch) if ch == 'h' => {
                    // Special case -h/--help since behavior is different
                    // based on whether short or long flag is given.
                    args.special = Some(args::SpecialMode::HelpShort);
                    continue;
                }
                lexopt::Arg::Short(ch) if ch == 'v' => {
                    // Special case -v/--version since behavior is different
                    // based on whether short or long flag is given.
                    args.special = Some(args::SpecialMode::VersionShort);
                    continue;
                }
                lexopt::Arg::Short(ch) => self.find_short(ch),
                lexopt::Arg::Long(name) if name == "help" => {
                    // Special case -h/--help since behavior is different
                    // based on whether short or long flag is given.
                    args.special = Some(args::SpecialMode::HelpLong);
                    continue;
                }
                lexopt::Arg::Long(name) if name == "version" => {
                    // Special case -v/--version since behavior is different
                    // based on whether short or long flag is given.
                    args.special = Some(args::SpecialMode::VersionLong);
                    continue;
                }
                lexopt::Arg::Long(name) => self.find_long(name),
            };

            let mat = match lookup {
                FlagLookUp::Match(mat) => mat,
                FlagLookUp::UnrecognizedShort(ch) => anyhow::bail!("Unrecognized flag -{ch}"),
                FlagLookUp::UnrecognizedLong(str) => anyhow::bail!("Unrecognized flag --{str}"),
            };

            // TODO: handel for multivalued flag
            let value: FlagValue<bool, String> = if mat.flag.is_switch() {
                FlagValue::Switch(true)
            } else {
                FlagValue::try_from(
                    p.value()
                        .with_context(|| format!("missing value for flag -{:?}", mat.name))?,
                )
                .with_context(|| format!(""))?
            };

            mat.flag
                .update(value, args)
                .with_context(|| format!("error parsing flag {:?}", mat))?;
        }

        Ok(ExitCode::from(0))
    }

    fn find_short(&self, ch: char) -> FlagLookUp<'_> {
        if !ch.is_ascii() {
            return FlagLookUp::UnrecognizedShort(ch);
        }

        let byte = u8::try_from(ch).unwrap();

        let Some(index) = self.map.find(&[byte]) else {
            return FlagLookUp::UnrecognizedShort(ch);
        };

        FlagLookUp::Match(&self.infos[index])
    }

    fn find_long(&self, str: &str) -> FlagLookUp<'_> {
        let Some(index) = self.map.find(str.as_bytes()) else {
            return FlagLookUp::UnrecognizedLong(str.to_string());
        };

        FlagLookUp::Match(&self.infos[index])
    }
}
