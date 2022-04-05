use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut alphabet: HashSet<char> = (b'a'..=b'z').fold(HashSet::new(), |mut acc, b| {
        acc.insert(b as char);
        acc
    });

    sentence.chars().for_each(|c| {
        alphabet.remove(&c.to_ascii_lowercase());
    });

    alphabet.len() == 0
}
