const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const VOWEL_LIKE: [&str; 2] = ["xr", "yt"];
const SOUNDS_2: [&str; 7] = ["sh", "th", "ch", "qu", "rh", "xr", "yt"];
const SOUNDS_3: [&str; 2] = ["sch", "thr"];

fn starts_with_vowel(input: &str) -> bool {
    let c = input.chars().next().unwrap();
    VOWELS.contains(&c)
}

fn starts_with_vowel_like(input: &str) -> bool {
    let two = input.chars().take(2).collect::<String>();
    VOWEL_LIKE.contains(&two.as_str())
}

fn starts_with_consonant_sound_2(input: &str) -> bool {
    let two = input.chars().take(2).collect::<String>();

    SOUNDS_2.contains(&two.as_str())
}

fn starts_with_consonant_sound_3(input: &str) -> bool {
    let three = input.chars().take(3).collect::<String>();

    SOUNDS_3.contains(&three.as_str())
}

fn qu_after_cons(input: &str) -> bool {
    let first_three: Vec<char> = input.chars().take(3).collect();
    if !VOWELS.contains(&first_three[0]) && first_three[1] == 'q' && first_three[2] == 'u' {
        true
    } else {
        false
    }
}

fn find_split(input: &str) -> usize {
    if starts_with_vowel(input) || starts_with_vowel_like(input) {
        0
    } else if starts_with_consonant_sound_3(input) || qu_after_cons(input) {
        3
    } else if starts_with_consonant_sound_2(input) {
        2
    } else {
        1
    }
}

fn translate_word(input: &str) -> String {
    let split = find_split(input);
    let (first, second) = input.split_at(split);
    format!("{}{}ay", second, first)
}

pub fn translate(input: &str) -> String {
    let words: Vec<String> = input
        .split_whitespace()
        .map(|word| translate_word(word))
        .collect();
    words.join(" ")
}
