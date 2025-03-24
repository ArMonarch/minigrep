#[derive(Debug)]
pub struct Args {
    // Essential Arguments
    pub special: Option<SpecialMode>,
    pub _mode: bool,
    pub positional: Vec<String>,
    pub _pattern: String,
    pub file: File,

    // Everything Else
    pub patterns: Patterns,
}

impl Default for Args {
    fn default() -> Self {
        Args {
            special: None,
            _mode: false,
            positional: Vec::new(),
            _pattern: String::new(),
            patterns: Patterns::new(),
            file: File::new(String::new()),
        }
    }
}

/// A "special" mode that supercedes everything else.
#[derive(Debug)]
pub enum SpecialMode {
    /// Show a condensed version of "help" output. This correspondes to the '-h' flag
    HelpShort,
    /// Shows a very verbose version of the "help" output. This correspondes to the '--help' flag
    HelpLong,

    /// Show condensed version information. e.g., `minigrep x.y.z`.
    VersionShort,
    /// Show verbose version information. Includes "short" information as well as features included
    /// in the build
    VersionLong,
}

#[derive(Debug)]
pub struct Patterns {
    _patterns: Vec<String>,
}

impl Patterns {
    pub fn new() -> Self {
        Patterns {
            _patterns: Vec::new(),
        }
    }

    pub fn from(patterns: Vec<String>) -> Patterns {
        Patterns {
            _patterns: patterns,
        }
    }
}

#[derive(Debug)]
pub struct File {
    pub _name: String,
}

impl File {
    pub fn new(name: String) -> Self {
        File { _name: name }
    }
}
