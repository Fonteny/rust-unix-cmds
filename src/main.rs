mod wc;
mod uniq;
mod grep;

fn get_first_word(text: &str) -> Option<&str> {
    text.split_whitespace().next()
}

fn main() {
    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let command = get_first_word(&input);

    if command == Some("wc") {
        let message = wc::word_count(&input);

        if let Some(output) = message {
            println!("{output}");
        }
    }

    if command == Some("uniq") {
        let message = uniq::unique(&input);


        if let Some(output) = message {

            let formatted_output = output.join("\n");

            println!("{formatted_output}");
        }
    }

    if command == Some("grep") {
        let message = grep::regex_search(&input);

        if let Some(output) = message {
            let formatted_output = output.join("\n");

            println!("{}", formatted_output);
        }
    }
}
