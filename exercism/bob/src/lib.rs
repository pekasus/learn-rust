pub fn reply(message: &str) -> &str {
    let stripped: String = message.chars().filter(|c| !c.is_whitespace()).collect();

    let has_text = stripped
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '?' || *c == '!')
        .collect::<String>().len() > 0;

    let all_caps;
    {
        let alphabetics: String = stripped.chars().filter(|c| c.is_alphabetic()).collect();
        all_caps = alphabetics.len() > 0 && alphabetics.chars().all(|c| c.is_uppercase());
    }
    
    let is_question: bool = stripped.ends_with("?");

    match (has_text, is_question, all_caps) {
        (false, _, _) => "Fine. Be that way!",
        (_, true, false) => "Sure.",
        (_, false, true) => "Whoa, chill out!",
        (_, true, true) => "Calm down, I know what I'm doing!",
        (_, _, _) => "Whatever.",
    }
}
