use std::{ascii::AsciiExt, fs};

fn part1(string: &str) -> isize {
    string.chars().fold(0isize, |level, c| match c {
        '(' => level + 1,
        ')' => level - 1,
        _ => panic!("Invalid character in the input: {}", c),
    })
}

fn part2(string: &str) -> usize {
    let mut level = 0 as isize;
    string
        .chars()
        .enumerate()
        .find_map(|(idx, c)| {
            match c {
                '(' => level += 1,
                ')' => level -= 1,
                _ => panic!("Invalid character in the input: {}", c),
            };
            if level == -1 {
                Some(idx+1)
            } else {
                None
            }
        })
        .unwrap()
}

fn main() {
    let text = fs::read_to_string("./input.txt")
        .expect("Can't read input file")
        .trim()
        .to_owned();

    // For our input.txt, the result was 280. However, youd Advent Of Code input will probably be different.
    println!("Result: {}", part1(&text));
    println!("Result: {}", part2(&text));
}
