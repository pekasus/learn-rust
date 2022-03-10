pub fn abbreviate(phrase: &str) -> String {
    let phrase = phrase.to_owned().replace('-', " ").replace('_', " ");

    phrase
        .split_whitespace()
        .map(|word| handle_word(word))
        .collect()
}

fn handle_word(word: &str) -> String {
    let has_upper_case = word.chars().any(|c| c.is_uppercase());
    let has_lower_case = word.chars().any(|c| c.is_lowercase());
    let is_camel_case = has_lower_case && has_upper_case;

    if is_camel_case {
        word.chars()
            .filter(|c| c.is_uppercase())
            .collect::<String>()
    } else {
        word.chars().nth(0).unwrap().to_uppercase().to_string()
    }
}
