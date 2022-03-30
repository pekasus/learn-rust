use itertools::Itertools;
use std::{char, fs};

fn one_generation(input: &Vec<char>) -> Vec<char> {
    let mut result = Vec::new();
    let mut last_letter: char = input[0];
    let mut last_letter_occurrences = 0;

    input.iter().for_each(|&c| {
        if c == last_letter {
            last_letter_occurrences += 1;
        } else {
            result.push(char::from_digit(last_letter_occurrences, 10).unwrap());
            result.push(last_letter);
            last_letter = c;
            last_letter_occurrences = 1;
        }
    });
    result.push(char::from_digit(last_letter_occurrences, 10).unwrap());
    result.push(last_letter);

    // input.iter().coalesce(|first, second| {
    //     if *first == *second {
    //         last_letter_occurrences += 1;
    //         Ok(second)
    //     } else {
    //         result.push(char::from_digit(last_letter_occurrences, 10).unwrap());
    //         result.push(*first);
    //         // last_letter = *second;
    //         last_letter_occurrences = 1;
    //         Ok(second)
    //     }
    // });
    // for (position, digit) in input.iter().enumerate() {
    //     if position == input.len() - 1 { // the last digit
    //     }
    // }
    //println!("{:?}", result);
    result

    /*let result = input.iter().fold( (-1, 0, Vec::<char>::new()), |(previous_digit, counts_of_digit, result)| {

    });

    result*/
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let line = input.lines().into_iter().next().unwrap();
    let chars = line.chars().collect::<Vec<_>>();

    let result = (0..40).fold(chars, |chars, _| one_generation(&chars));
    //println!("{:#?}", result);
    println!("{:#?}", result.len());
}
