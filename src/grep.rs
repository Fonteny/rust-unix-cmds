use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn matching_lines(path: &str, pattern: Regex) -> Option<Vec<String>> {
    let file = File::open(path).ok()?;
    let reader = BufReader::new(file);

    let matched_lines: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| pattern.is_match(line) || pattern.as_str().is_empty())
        .collect();

    if !matched_lines.is_empty() {
        Some(matched_lines)
    } else {
        None
    }
}

pub fn regex_search(input: &str) -> Option<Vec<String>> {
    let mut input = input.split_whitespace().skip(1); // Skip grep in front of args
    let pattern_str = input.next()?.trim();
    let pattern = Regex::new(pattern_str).ok()?;
    let file_path = input.next().map(str::trim);

    match file_path {
        Some(path) => matching_lines(&path, pattern),
        None => {
            // Handle the case where no file path is provided
            // You can implement a different behavior here if needed
            println!("No file path provided.");
            None
        }
    }
}
