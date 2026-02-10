use anyhow::{self, Context};
use std::fs::File;
use std::io::Read;
use std::process::exit;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: can-tracetool status <file>");
        exit(1);
    }
    let mut args_left = args.len() - 1;
    let filepath = args
        .get(2)
        .with_context(|| format!("'status' requires an argument"))?;
    let mut file =
        File::open(filepath).with_context(|| format!("Failed to open input file: {}", filepath))?;

    let mut contents: String = String::new();
    file.read_to_string(&mut contents)
        .with_context(|| format!("Invalid file format!"))?;
    args_left -= 2;
    // Debug print:
    println!("File contents:\n{}", contents);
    let lines: Vec<&str> = contents.lines().collect();
    let mut ignored_lines = 0;
    let mut invalid_lines = 0;
    let mut valid_lines = 0;
    for line in lines {
        let s = line.trim();

        // Ignore empty lines
        if s.is_empty() {
            ignored_lines += 1;
            continue;
        }

        // Ignore commented lines
        if s.starts_with('#') {
            ignored_lines += 1;
            continue;
        }

        // Split string into tokens
        let tokens: Vec<&str> = s.split_whitespace().collect();
        if tokens.len() == 3 {
            valid_lines += 1;
        } else {
            invalid_lines += 1;
        }
    }

    println!(
        "Valid lines: {}, Invalid lines: {}, Ignored lines: {}",
        valid_lines, invalid_lines, ignored_lines
    );
    Ok(())
}
