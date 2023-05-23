use std::cmp::max;
use std::fs;
use std::fs::metadata;
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

fn option_case(option: &str, file_path: &str) -> Option<String> {
    match option {
        "-c" => file_byte_count(file_path),
        "-l" => file_line_count(file_path),
        "-w" => file_word_count(file_path),
        "-m" => file_char_count(file_path),
        "-L" => file_longest_line_len(file_path),
        _    => None
    }
    .zip(Some(file_path))
    .map(|(output, file_path_val)| format!("{output} {file_path_val}"))
}

fn default_case(file_path: &str) -> Option<String> {
    let byte_count = file_byte_count(file_path)?;
    let line_count = file_line_count(file_path)?;
    let word_count = file_word_count(file_path)?;
    Some(format!("{line_count} {word_count} {byte_count} {file_path}"))
}

pub fn word_count(token1: Option<&str>, token2: Option<&str>) -> Option<String> {
    let token1 = token1.map(str::trim);
    let token2 = token2.map(str::trim);
    if metadata(token1?).ok()?.is_file() {
        println!("{token1:?} is a file");
        return default_case(token1?);
    }
    if metadata(token2?).ok()?.is_file() {
        println!("{token2:?} is a file");
        return option_case(token1?, token2?);
    }
    None
}