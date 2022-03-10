fn main() {
    let content = std::fs::read_to_string("input.txt").expect("Could not read from file.");
    let lines_iter = content.lines();
    println!("part1: {}", part1(lines_iter.clone()));
    println!("part2: {}", part2(lines_iter.clone()));
}

fn part1<'a>(lines: impl Iterator<Item = &'a str>) -> usize {
    lines
        .filter(|&line| {
            contains_3_vowels(line)
                && contains_double_letter(line)
                && does_not_contain_patterns(line)
        })
        .count()
}

fn contains_3_vowels(string: &str) -> bool {
    string
        .chars()
        .filter(|c| vec!['a', 'e', 'i', 'o', 'u'].iter().any(|v| v == c))
        .count()
        > 2
}

fn contains_double_letter(string: &str) -> bool {
    string
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .any(|window| window[0] == window[1])
}

fn does_not_contain_patterns(string: &str) -> bool {
    !string
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .any(|window| {
            let window = window.iter().collect::<String>();
            vec!["ab", "cd", "pq", "xy"].iter().any(|p| **p == window)
        })
}

fn part2<'a>(lines: impl Iterator<Item = &'a str>) -> usize {
    lines
        .filter(|&line| has_repeating_pairs(line) && has_letter_between(line))
        .count()
}
fn has_repeating_pairs(string: &str) -> bool {
    string
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .enumerate()
        .any(|(index, window)| {
            let window = &String::from_iter(window);
            let remaining = &string[index + 2..];
            remaining.contains(window)
        })
}

fn has_letter_between(string: &str) -> bool {
    string
        .chars()
        .collect::<Vec<_>>()
        .windows(3)
        .any(|window| window[0] == window[2])
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
        use crate::{has_letter_between, has_repeating_pairs};
        assert!(has_repeating_pairs("qjhvhtzxzqqjkmpb"));
        assert!(has_letter_between("qjhvhtzxzqqjkmpb"));
        assert!(!has_letter_between("uurcxstgmygtbstg"));
        assert!(!has_repeating_pairs("ieodomkazucvgmuy"));
    }
}
