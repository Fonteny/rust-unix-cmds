use std::fs;
use std::io::{BufRead, BufReader};

fn remove_adjacent_duplicates(path: &str) -> Option<Vec<String>> {
    let file = fs::File::open(path).ok()?;
    let reader = BufReader::new(file);

    let unique_lines: Vec<String> = reader
        .lines()
        .filter_map(Result::ok)
        .collect::<Vec<String>>()
        .windows(2)
        .filter(|pair| pair[0] != pair[1])
        .map(|pair| pair[0].clone())
        .collect();

    Some(unique_lines)
}

pub fn unique(input: &String) -> Option<Vec<String>> {
    let mut input = input.split_whitespace().skip(1); //skip uniq in front of args
    let command = input.next()?.trim();
    let file_path = input.next()
        .map(str::trim)
        .or(Some(command)); //use cmd as file_path in case no option provided

    let output = match command {
        _ => file_path.and_then(remove_adjacent_duplicates)
    };

    Some(output).flatten()
}