use std::{fmt::Write, iter::zip};

macro_rules! write {
    ($($tt:tt)*) => {
        std::write!($($tt)*).unwrap()
    };
}

use super::flags::FLAGS;
use super::flags::Flag;

fn generate_version() -> String {
    let semver = option_env!("CARGO_PKG_VERSION").unwrap_or("N/A");
    return semver.to_string();
}

/// Generates a short version string of the form `minigrep x.y.z`.
pub fn generate_version_short() -> String {
    let version = generate_version();
    format!("minigrep {}", version)
}

/// Generates a longer multi-line version string.
pub fn generate_version_long() -> String {
    let (author, mail): (&'static str, &'static str) = ("Ar_Monarch", "praffulthapa11@gmail.com");
    format!("{} \n{} <{}>", generate_version_short(), author, mail)
}

const TEMPLATE_HELP_SHORT: &str = r"
minigrep !!version!!
Ar_Monarch <praffulthapa11>

minigrep searches for PATTERNS in given FILE. minigrep prints each line that matches a patten.

Project Home Page: https://github.com/ArMonarch/minigrep

USAGE: 
    minigrep [OPTIONS] PATTERN FILE
    minigrep [OPTIONS] PATTERN -f|--file FILE
    minigrep [OPTIONS] -p|--pattern PATTERN FILE
    minigrep [OPTIONS] -p|--pattern PATTERN -f|--file FILE

OPTIONS:
!!options!!
";

pub fn generate_help_short() -> String {
    let out = TEMPLATE_HELP_SHORT.replace("!!version!!", &generate_version());

    let (mut column_1, mut column_2) = (Vec::new(), Vec::new());
    let (mut max_col_1, mut max_col_2) = (0, 0);
    for flag in FLAGS.iter().copied() {
        let (col_1, col_2) = generate_flag_short(flag);
        max_col_1 = std::cmp::max(max_col_1, col_1.len());
        max_col_2 = std::cmp::max(max_col_2, col_2.len());
        column_1.push(col_1);
        column_2.push(col_2);
    }

    let var = "!!options!!";
    let val = format_short_colums(column_1, column_2, max_col_1, max_col_2);

    out.replace(var, &val)
}

fn generate_flag_short(flag: &dyn Flag) -> (String, String) {
    let (mut col_1, mut col_2) = (String::new(), String::new());

    // Generate the first column, the flag name.
    if let Some(byte) = flag.name_short() {
        let name = char::from(byte);
        write!(col_1, r"-{name}");
        write!(col_1, r", ");
    }

    write!(col_1, r"--{name}", name = flag.name_long());

    // Generate the second column, with the flag description.

    write!(col_2, "{}", flag.doc_short());

    (col_1, col_2)
}

fn format_short_colums<'a>(
    column_1: Vec<String>,
    colums_2: Vec<String>,
    max_col_1: usize,
    _max_col_2: usize,
) -> String {
    assert_eq!(
        column_1.len(),
        colums_2.len(),
        "columns must have equal length"
    );

    const PAD: usize = 2;
    let mut out = String::new();

    for (i, (col_1, col_2)) in zip(column_1.iter(), colums_2.iter()).enumerate() {
        if i > 0 {
            write!(out, "\n");
        }

        let pad = max_col_1 - col_1.len() + PAD;
        write!(out, "{}", " ".repeat(4));
        write!(out, "{col_1}");
        write!(out, "{}", " ".repeat(pad));
        write!(out, "{col_2}");
    }

    out
}

pub fn generate_help_long() -> String {
    generate_help_short()
}
