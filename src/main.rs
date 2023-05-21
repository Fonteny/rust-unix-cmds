use std::fs;
use std::io::{self, BufRead, BufReader, Read};

fn get_first_word(text: &str) -> Option<&str> {
    text.split_whitespace().next()
}

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


fn word_count(input: String) -> Option<String> {
    let mut input = input.split_whitespace().skip(1); //skip wc in front of args
    let command = input.next().unwrap_or_default().trim();
    let file_path = input.next().map(str::trim);

    let output = match (command, file_path) {
        ("-c", Some(file_path)) => file_byte_count(file_path),
        ("-l", Some(file_path)) => file_line_count(file_path),
        ("-m", Some(file_path)) => file_word_count(file_path),
        (_, None) => {
            let byte_count = file_byte_count(command)?;
            let line_count = file_line_count(command)?;
            let word_count = file_word_count(command)?;
            return Some(format!("{} {} {} {}", line_count, word_count, byte_count, command));
        }
        _ => return None,
    };

    if let (Some(output), Some(file_path)) = (output, file_path) {
        return Some(format!("{output} {file_path}"));
    }
    None
}


fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    if get_first_word(&input) == Some("wc") {
        let message = word_count(input);

        if let Some(output) = message {
            println!("{output}");
        }
    }
}