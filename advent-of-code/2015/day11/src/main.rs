use core::num;
use std::char;
use std::fs;
use std::str::Chars;

const NUMBER_OF_LATIN_LETTERS: u32 = 'z' as u32 - 'a' as u32 + 1;
const I_VALUE: u32 = 9;
const L_VALUE: u32 = 12;
const O_VALUE: u32 = 15;
// a -> 0
// z -> 25

// a -> 1
// z -> 26
// increment of z:

fn letter_to_value(letter: char) -> u32 {
    println!("c: {}", letter);
    letter as u32 - 'a' as u32 + 1
}

fn value_to_letter(value: u32) -> char {
    char::from_u32(value + 'a' as u32 - 1).unwrap()
}

fn map_to_value(input: Chars) -> Vec<u32> {
    input.map(|c| letter_to_value(c)).collect()
}

fn map_to_string(input: &Vec<u32>) -> String {
    input.iter().map(|&c| value_to_letter(c)).collect()
}

fn increment_value(mut password: &mut Vec<u32>, idx: usize) {
    let mut value = password[idx] + 1;
    if value == I_VALUE || value == L_VALUE || value == O_VALUE {
        value += 1;
    }
    if value > 26 {
        password[idx] = 1;
        if idx > 0 {
            increment_value(&mut password, idx - 1);
        }
    } else {
        password[idx] = value;
    }
}

fn check_for_consecutive(password: &Vec<u32>) -> bool {
    password
        .windows(3)
        .any(|window| window[0] + 1 == window[1] && window[1] + 1 == window[2])
}

fn check_for_doubles(password: &Vec<u32>) -> bool {
    let result_state = password.iter().fold(
        (0, false, 0usize),
        |(previous, previous_was_in_pair, number_of_pairs), &current| {
            if previous_was_in_pair {
                (current, false, number_of_pairs)
            } else {
                if previous == current {
                    (current, true, number_of_pairs + 1)
                } else {
                    (current, false, number_of_pairs)
                }
            }
        },
    );
    let number_of_pairs = result_state.2;
    number_of_pairs >= 2
}

fn part1(mut password: Vec<u32>) -> Vec<u32> {
    let mut valid_password: bool = false;
    while !valid_password {
        let password_len = password.len();
        increment_value(&mut password, password_len - 1);
        if check_for_consecutive(&password) && check_for_doubles(&password) {
            valid_password = true;
        }
    }
    password
}

fn main() {
    let input_lines = fs::read_to_string("input.txt").unwrap();
    let input_chars = input_lines.chars();
    let converted_password = map_to_value(input_chars);

    let part1_vec = part1(converted_password);
    let result1 = map_to_string(&part1_vec);
    println!("{}", result1);

    let part2_vec = part1(part1_vec);
    let result2 = map_to_string(&part2_vec);
    println!("{}", result2);
}
