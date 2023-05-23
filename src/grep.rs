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

/*
pub fn regex_search(token1: Option<&String>, token2: Option<&String>) -> Option<Vec<String>> {
    if metadata(token1?).ok()?.is_file() {
        return default_case(token1?);
    }
    if metadata(token2?).ok()?.is_file() {
        return option_case(token1?, token2?);
    }
    None
}
*/