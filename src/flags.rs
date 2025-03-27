use std::ffi::OsString;
use std::fmt::Debug;

use crate::args;

/// Represents flag name, either &str OR u8
#[derive(Debug)]
pub enum FlagName<I, O> {
    Char(I),
    String(O),
}

impl<I, O> From<char> for FlagName<I, O>
where
    I: From<char>,
{
    fn from(value: char) -> FlagName<I, O> {
        FlagName::<I, O>::Char(I::from(value))
    }
}

impl<I, O> From<u8> for FlagName<I, O>
where
    I: From<u8>,
{
    fn from(value: u8) -> FlagName<I, O> {
        FlagName::<I, O>::Char(I::from(value))
    }
}

/// The kind of flag that is being matched.
#[derive(Debug)]
pub enum FlagInfoKind {
    /// A standard flag, e.g., --passthru.
    Standard,
    /// A negation of a standard flag, e.g., --no-multiline.
    _Negated,
}

/// The info about a flag associated with a flag's ID in the flag map.
#[derive(Debug)]
pub struct FlagInfo {
    /// The flag object and its associated metadata.
    pub flag: &'static dyn Flag,
    /// The actual name that is stored in the Aho-Corasick automaton. When this
    /// is a byte, it corresponds to a short single character ASCII flag. The
    /// actual pattern that's in the Aho-Corasick automaton is just the single
    /// byte.
    pub name: FlagName<u8, &'static str>,
    /// The type of flag that is stored for the corresponding Aho-Corasick
    /// pattern.
    pub _flag_kind: FlagInfoKind,
}

/// Represents a value parsed from the command line.
///
/// This doesn't include the corresponding flag, but values come in one of
/// two forms: a switch (on or off) or an arbitrary value.
pub enum FlagValue<I, O> {
    Switch(I),
    Value(O),
}

impl<I, O> TryFrom<OsString> for FlagValue<I, O>
where
    O: From<String>,
{
    type Error = anyhow::Error;

    fn try_from(value: OsString) -> Result<Self, Self::Error> {
        let str = match value.into_string() {
            Ok(str) => str,
            Err(os_str) => anyhow::bail!(
                "failed convertion form OsString to String for value: {:?}",
                os_str
            ),
        };

        Ok(FlagValue::<I, O>::Value(O::from(str)))
    }
}

impl<I, O> FlagValue<I, O> {
    /// Return the yes or no value of this switch.
    ///
    /// If this flag value is not a switch, then this panics.
    ///
    /// This is useful when writing the implementation of `Flag::update`.
    /// namely, callers usually know whether a switch or a value is expected.
    /// If a flag is something different, then it indicates a bug, and thus a
    /// panic is acceptable.
    fn _unwrap_switch(self) -> I {
        match self {
            FlagValue::Switch(val) => val,
            FlagValue::Value(_) => unreachable!("got flag value but expected switch"),
        }
    }

    /// Return the user provided value of this flag.
    ///
    /// If this flag is a switch, then this panics.
    ///
    /// This is useful when writing the implementation of `Flag::update`.
    /// namely, callers usually know whether a switch or a value is expected.
    /// If a flag is something different, then it indicates a bug, and thus a
    /// panic is acceptable.
    fn unwrap_value(self) -> O {
        match self {
            FlagValue::Value(val) => val,
            FlagValue::Switch(_) => unreachable!("got switch but expected flag value"),
        }
    }
}

#[derive(Debug)]
pub struct FlagMap {
    pub map: std::collections::HashMap<Vec<u8>, usize>,
}

impl FlagMap {
    /// Create a new map of flags for the given flag information.
    ///
    /// The index of each flag info corresponds to its ID.
    pub fn new(infos: &[FlagInfo]) -> FlagMap {
        let mut map = std::collections::HashMap::new();

        for (i, flag_info) in infos.iter().enumerate() {
            match flag_info.name {
                FlagName::Char(byte) => {
                    assert_eq!(
                        None,
                        map.insert(vec![byte], i),
                        "to be inserted value already in HashMap, which is never possible"
                    );
                }
                FlagName::String(str) => {
                    assert_eq!(
                        None,
                        map.insert(str.as_bytes().to_vec(), i),
                        "to be inserted value already in HashMap, which is never possible"
                    );
                }
            }
        }

        FlagMap { map }
    }

    /// Look for a match of `name` in the given Aho-Corasick automaton.
    ///
    /// This only returns a match if the one found has a length equivalent to
    /// the length of the name given.
    pub fn find(&self, bytes: &[u8]) -> Option<usize> {
        self.map.get(bytes).copied()
    }
}

/// The result of looking up a flag name.
#[derive(Debug)]
pub enum FlagLookUp<'a> {
    /// Lookup found a match and the metadata for the flag is attached.
    Match(&'a FlagInfo),
    /// The given short name is unrecognized.
    UnrecognizedShort(char),
    /// The given long name is unrecognized.
    UnrecognizedLong(String),
}

pub trait Flag: Debug + Send + Sync + 'static {
    fn is_switch(&self) -> bool;

    /// Flag short name is Optional
    fn name_short(&self) -> Option<u8> {
        None
    }

    fn name_long(&self) -> &'static str;

    fn doc_short(&self) -> &'static str;

    fn _doc_long(&self) -> &'static str;

    fn update(&self, value: FlagValue<bool, String>, args: &mut args::Args) -> anyhow::Result<()>;
}

/// A list of all flags in minigrep via implementations of `Flag`.
pub(super) const FLAGS: &[&dyn Flag] = &[&Patterns, &File];

/// -p/--pattern
#[derive(Debug)]
struct Patterns;

impl Flag for Patterns {
    fn is_switch(&self) -> bool {
        false
    }

    fn name_short(&self) -> Option<u8> {
        Some(b'p')
    }

    fn name_long(&self) -> &'static str {
        "pattern"
    }

    fn doc_short(&self) -> &'static str {
        r"Search for given patterns"
    }

    fn _doc_long(&self) -> &'static str {
        ""
    }

    fn update(&self, value: FlagValue<bool, String>, args: &mut args::Args) -> anyhow::Result<()> {
        let patterns = value.unwrap_value();

        args.patterns = args::Patterns::from(patterns);
        Ok(())
    }
}

/// -f/--file
#[derive(Debug)]
struct File;

impl Flag for File {
    fn is_switch(&self) -> bool {
        false
    }

    fn name_short(&self) -> Option<u8> {
        Some(b'f')
    }

    fn name_long(&self) -> &'static str {
        "file"
    }

    fn doc_short(&self) -> &'static str {
        r"Search for the given file for patterns"
    }

    fn _doc_long(&self) -> &'static str {
        ""
    }

    fn update(&self, value: FlagValue<bool, String>, args: &mut args::Args) -> anyhow::Result<()> {
        let file_name = value.unwrap_value();

        args.file = args::File::new(file_name);
        Ok(())
    }
}

use crate::parse::{ParseResult, Parser};
pub fn parse() -> ParseResult<args::Args> {
    let parser = Parser::new();
    let mut args = args::Args::default();

    if let Err(err) = parser.parse(std::env::args().skip(1), &mut args) {
        return ParseResult::Err(err);
    };

    // we can bail early if a special mode was enabled.
    // This is basically only for version and help output
    if let Some(special_mode) = args.special {
        return ParseResult::Special(special_mode);
    }

    ParseResult::Ok(args)
}
