use anyhow::Context;

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Search(Searchmode),
}

#[derive(Debug, Clone, Copy)]
pub enum Searchmode {
    Standard,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Args {
    // Essential Arguments
    pub special: Option<SpecialMode>,
    pub mode: Mode,
    pub positional: Vec<String>,
    pub pattern: String,
    pub file: File<String>,

    // Everything Else
    pub patterns: Patterns<String>,
}

impl Args {
    /// Returns true if some non-zero number of matches is believed to be
    /// possible.
    pub fn matches_possible(&self) -> bool {
        if self.positional.is_empty()
            && self.pattern.is_empty()
            && self.patterns.patterns.is_empty()
        {
            return false;
        }

        true
    }

    // Returns possible Pattern that is to be searched
    pub fn get_patterns(&mut self) -> anyhow::Result<Vec<String>> {
        if !self.patterns.is_empty() {
            return Ok(self.patterns.patterns.clone());
        }

        if !self.pattern.is_empty() {
            return Ok(vec![self.pattern.clone()]);
        }

        let pattern = vec![
            self.positional
                .pop()
                .context("pattern to search not found")?,
        ];

        Ok(pattern)
    }

    pub fn get_file(&mut self) -> anyhow::Result<String> {
        if !self.file.name.is_empty() {
            return Ok(self.file.name.clone());
        }

        let file = self
            .positional
            .pop()
            .context("file to be searched for pattern not provided")?;

        Ok(file)
    }
}

impl Default for Args {
    fn default() -> Self {
        Args {
            special: None,
            mode: Mode::Search(Searchmode::Standard),
            positional: Vec::new(),
            pattern: String::new(),
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
pub struct Patterns<T> {
    pub patterns: Vec<T>,
}

impl<T> Patterns<T> {
    pub fn new() -> Patterns<T> {
        Patterns {
            patterns: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.patterns.is_empty()
    }
}

impl<T> From<String> for Patterns<T>
where
    T: From<String>,
{
    fn from(value: String) -> Patterns<T> {
        Patterns {
            patterns: vec![T::from(value)],
        }
    }
}

#[derive(Debug)]
pub struct File<T> {
    pub name: T,
}

impl<T> File<T> {
    pub fn new(name: T) -> Self {
        File { name }
    }
}

impl<T: Clone> Clone for File<T> {
    fn clone(&self) -> File<T> {
        File {
            name: self.name.clone(),
        }
    }
}
