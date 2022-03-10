pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len > digits.len() || digits.is_empty() {
        return vec![];
    }

    let digits: Vec<char> = digits.chars().collect();

    digits
        .windows(len)
        .map(|slice| slice.iter().collect::<String>())
        .collect()
    // TODO: Handle the case len = 0
}
