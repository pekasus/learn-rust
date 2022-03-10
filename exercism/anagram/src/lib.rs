use std::collections::HashSet;

pub fn anagrams_for<'a>(
    word: &str,
    possible_anagrams: &[&'a str],
) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let mut sorted_word = word.chars().collect::<Vec<_>>();
    sorted_word.sort_unstable();

    possible_anagrams
        .iter()
        .copied()
        .filter(|candidate| {
            let candidate_lower = candidate.to_lowercase();
            if candidate_lower == word {
                return false;
            }
            let mut sorted_canditate =
                candidate_lower.chars().collect::<Vec<_>>();
            sorted_canditate.sort_unstable();
            sorted_canditate == sorted_word
        })
        .collect()
}
