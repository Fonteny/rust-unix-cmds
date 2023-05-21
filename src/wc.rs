use std::cmp::max;
use std::fs;
use std::io::{BufRead, BufReader, Read};

fn file_byte_count(path: &str) -> Option<usize> {
    let mut file = fs::File::open(path).ok()?;
    let mut buffer = [0; 4096];

    let mut count = 0;

    loop {
        match file.read(&mut buffer) {
            Ok(bytes_read) if bytes_read > 0 => count += bytes_read,
            _ => break,
        }
    }

    Some(count)
}

fn file_line_count(path: &str) -> Option<usize> {
    let file = fs::File::open(path).ok()?;
    let reader = BufReader::new(file);
    Some(reader.lines().count())
}

fn file_word_count(path: &str) -> Option<usize> {
    let file = fs::File::open(path).ok()?;
    let reader = BufReader::new(file);

    let mut count = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            count += line.split_whitespace().count();
        }
    }

    Some(count)
}

fn file_char_count(path: &str) -> Option<usize> {
    let mut file = fs::File::open(path).ok()?;
    let mut buffer = [0; 4096];

    let mut count = 0;

    loop {
        match file.read(&mut buffer) {
            Ok(bytes_read) if bytes_read > 0 => {
                let chunk = &buffer[..bytes_read];
                count += chunk.iter().filter(|&&c| c != 0).count();
            }
            _ => break,
        }
    }

    Some(count)
}

fn file_longest_line_len(path: &str) -> Option<usize> {
    let file = fs::File::open(path).ok()?;
    let reader = BufReader::new(file);

    let mut max_len = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            max_len = max(max_len, line.len());
        }
    }

    Some(max_len)
}
pub fn word_count(input: &String) -> Option<String> {
    let mut input = input.split_whitespace()
        .skip(1); //skip wc in front of args
    let command = input.next()?
        .trim();
    let file_path = input.next()
        .map(str::trim)
        .or(Some(command)); //use cmd as file_path if arg not provided

    let output = match command {
        "-c" => file_path.and_then(file_byte_count),
        "-l" => file_path.and_then(file_line_count),
        "-w" => file_path.and_then(file_char_count),
        "-m" => file_path.and_then(file_word_count),
        "-L" => file_path.and_then(file_longest_line_len),
        _ => {
            let byte_count = file_path.and_then(file_byte_count)?;
            let line_count = file_path.and_then(file_line_count)?;
            let word_count = file_path.and_then(file_word_count)?;
            return Some(format!("{line_count} {word_count} {byte_count} {command}"));
        }
    };

    output
        .zip(file_path)
        .map(|(output_val, file_path_val)| format!("{output_val} {file_path_val}"))
}
