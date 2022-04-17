#![allow(unused)]

use anyhow::{Context, Result};
use clap::Parser;
use std::env::args;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    // the pattern to look for
    pattern: String,
    // the path for the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) 
-> Result<()> {
    for line in content.lines() {
        if line.contains(&pattern) {
            writeln!(writer, "{}", line)?;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    // println!("Hello, world!");
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}

fn answer() -> i32 {
    return 42 ;
}

#[test]
fn check_answer_validity() { 
    assert_eq!(answer(), 42);
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum \ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum \n");
}
