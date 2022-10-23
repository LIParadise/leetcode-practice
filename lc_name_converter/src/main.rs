/// Given LeetCode problem e.g. "240. Search a 2D Matrix II",
/// try to make a library crate "lc_240_search_a_2d_matrix_ii" in folder "240"
/// with some simple snippet in corresponding "src/lib.rs".
///
/// Usage:
/// <exe> "240. Search a 2D Matrix II"
/// Notice the quotes!
use regex::Regex;
use std::env;
use std::io::Write;
use std::process::Command;

const LIB_RS_CONTENTS: &'static str = "pub struct Solution;
    
impl Solution {}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        todo!()
    }
}";

fn main() {
    let pwd = env::current_dir()
        .expect("Cannot work in this directory, possibly because lack of permissions?");
    let arg: String = env::args().skip(1).collect();
    println!("You seem to want to solve LeetCode \"{arg}\"");
    let re_tokens = Regex::new(r"[[:space:]]?([[:word:]]*)\.?").unwrap();
    let mut crate_dir_name = String::from(
        re_tokens
            .captures(&arg)
            .unwrap()
            .get(1)
            .map(|s| s.as_str())
            .expect("Invalid name, abort."),
    );
    if let Err(_) = crate_dir_name.trim().parse::<usize>() {
        println!("Invalid name, abort.");
    }
    let mut crate_name = String::from("lc");
    crate_name.push_str(&re_tokens.replace_all(&arg, "_$1"));
    crate_name = crate_name.to_lowercase();

    while std::path::Path::new(&crate_dir_name).exists() {
        crate_dir_name.push_str("_rust");
    }
    println!(
        "Create crate \"{crate_dir_name}\" in {}? (y/N)",
        pwd.display()
    );
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    match input.trim().chars().take(1).next() {
        Some('y') | Some('Y') => println!("Got it. Try to launch `cargo...`"),
        _ => {
            println!("Abort.");
            std::process::exit(1)
        }
    }

    Command::new("cargo")
        .arg("init")
        .arg(&crate_dir_name)
        .arg("--name")
        .arg(&crate_name)
        .arg("--lib")
        .arg("--vcs")
        .arg("none")
        .output()
        .expect("Cannot launch cargo.");

    let mut lib_rs_filename = "./".to_string();
    lib_rs_filename.push_str(&crate_dir_name);
    lib_rs_filename.push_str("/src/lib.rs");
    let mut lib_rs = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&lib_rs_filename)
        .expect("Cannot work with lib.rs???");
    for line in LIB_RS_CONTENTS.split('\n') {
        writeln!(&mut lib_rs, "{}", line).unwrap();
    }
}
