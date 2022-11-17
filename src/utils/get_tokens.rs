use std::fs;

pub fn get_tokens() -> Vec<String> {
    let data = fs::read_to_string("./discord_tokens.txt").expect("Unable to read file");
    let tokens: Vec<String> = data.split("\n").map(|s| s.replace("\r", "").to_owned()).collect();
    if tokens.len() == 0 {
        panic!("There is no tokens provided.")
    }
    tokens
}
