//! Given LeetCode problem e.g. "240. Search a 2D Matrix II",
//! try to make a library crate "lc_240_search_a_2d_matrix_ii" in folder "240"
//! with some simple snippet in corresponding "src/lib.rs".
//!
//! Usage:
//! <exe> "240. Search a 2D Matrix II"
//! Notice the quotes!

use regex::Regex;
use std::env;
use std::io::Write;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::process::Command;

const LIB_RS_CONTENTS: &str = r#"pub struct Solution;

impl Solution {}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        todo!()
    }
}

macro_rules! lprintln {
    // Match when a format string and additional arguments are provided
    ($fmt:expr, $($arg:tt)*) => {{
        if cfg!(feature = "local_test") {
            println!($fmt, $($arg)*);
        }
    }};

    // Match when only a format string is provided
    ($fmt:expr) => {{
        if cfg!(feature = "local_test") {
            println!($fmt);
        }
    }};
}"#;

fn main() -> Result<(), Error> {
    let pwd = env::current_dir()?;
    let args = Vec::from_iter(env::args().skip(1));
    if args.len() != 1 {
        return Err(Error::from(ErrorKind::InvalidInput));
    }
    // len is 1 so impossible to panic here
    let arg = args.into_iter().next().unwrap();
    println!("You seem to want to solve LeetCode \"{arg}\"");

    let re =
        Regex::new(r"(?<problem_serial>[[:digit:]]+)\. (?<problem_name>[[:alnum:] ]+)").unwrap();

    let problem_serial = re
        .captures(&arg)
        .ok_or(ErrorKind::UnexpectedEof)?
        .name("problem_serial")
        .as_ref()
        .map(regex::Match::as_str)
        .ok_or(ErrorKind::UnexpectedEof)?
        .trim();

    let problem_name = re
        .captures(&arg)
        .ok_or(ErrorKind::UnexpectedEof)?
        .name("problem_name")
        .as_ref()
        .map(regex::Match::as_str)
        .ok_or(ErrorKind::UnexpectedEof)?;

    let crate_name =
        format!("lc_{}_{}", problem_serial, problem_name.replace(' ', "_")).to_ascii_lowercase();
    // prevent accidentally ruin existing data
    let mut crate_dir_name = problem_serial.to_owned();
    while std::path::Path::new(&crate_dir_name).exists() {
        crate_dir_name += "_rust";
    }
    let crate_dir_name = crate_dir_name.to_ascii_lowercase();

    println!(
        r#"Run 'cargo init "{crate_dir_name}" --name "{crate_name}"' in "{}"? (y/N)"#,
        pwd.display()
    );

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    match input.trim() {
        "y" | "Y" => println!("Got it. Try to launch `cargo...`"),
        _ => {
            println!("Abort.");
            return Err(Error::from(ErrorKind::Interrupted));
        }
    }

    Command::new("cargo")
        .arg("init")
        .arg(&crate_dir_name)
        .arg("--name")
        .arg(crate_name)
        .arg("--lib")
        .arg("--vcs")
        .arg("none")
        .output()?;

    let lib_rs = Path::new("./").join(&crate_dir_name).join("src/lib.rs");
    let mut lib_rs = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(lib_rs)?;
    write!(&mut lib_rs, "{}", LIB_RS_CONTENTS)?;
    Ok(())
}
