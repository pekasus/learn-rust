use std::{collections::HashMap, fmt::Write};
//use std::iter::win

fn replace_apostrophes(sentence: &str) -> Vec<char> {
    let mut result = sentence.to_owned();
    // let mut slice = &mut result[..];
    let mut slice: Vec<char> = sentence.chars().collect();
    //sentence.chars().enumerate().windows();
    let mut index = 1;
    //let derefed = *sentence;
    let chars: Vec<(usize, char)> = sentence.chars().enumerate().collect();
    chars.windows(3).for_each(|window| {
        if window[1].1 == '\'' {
            if window[0].1.is_alphabetic() && window[2].1.is_alphabetic() {
                {};
            } else {
                slice[window[1].0] = ' ';
            }
        }
    });

    if slice[0] == '\'' {
        slice[0] = ' ';
    }
    let last_index = slice.len() - 1;
    if slice[last_index] == '\'' {
        slice[last_index] = ' ';
    }

    slice
}

fn replace_punctuation(sentence: Vec<char>) -> Vec<char> {
    sentence
        .iter()
        .map(|&c| {
            if c.is_alphanumeric() || c == '\'' {
                c
            } else {
                ' '
            }
        })
        .collect()
}

fn create_words(sentence: Vec<char>) -> Vec<String> {
    let sentence: String = sentence.iter().collect();
    sentence.split_whitespace().map(str::to_string).collect()
}

fn create_map(words: Vec<String>) -> HashMap<String, u32> {
    words
        .iter()
        .fold(HashMap::<String, u32>::new(), |mut map, word| {
            *map.entry(word.clone()).or_insert(0) += 1;
            map
        })
}

pub fn word_count(sentence: &str) -> HashMap<String, u32> {
    let sentence = sentence.to_lowercase();
    let sentence = replace_apostrophes(&sentence);
    let sentence = replace_punctuation(sentence);
    let words = create_words(sentence);
    create_map(words)
}
