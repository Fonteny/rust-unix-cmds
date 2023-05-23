mod wc;
mod uniq;
mod grep;

fn main() {
    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: Vec<String> = input.split_whitespace().map(str::to_string).collect();


    for i in 0..input.len() {
        if input[i] == "wc" {
            let token1 = input.get(i+1).map(|x| &**x);
            let token2 = input.get(i+2).map(|x| &**x);
            let output = wc::word_count(token1, token2);

            if let Some(output_val) = output {
                println!("{output_val}");
            }
        }

        //if input[i] == "grep" {
            //let output = grep::regex_search(input.get(i+1), input.get(i+2));
            
        //}
    }
}
