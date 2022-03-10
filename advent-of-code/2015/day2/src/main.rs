use itertools::Itertools;
use std::fs;

fn part1(string: &str) -> usize {
    string.lines().fold(0usize, |total, line| {
        let mut line_parts: Vec<usize> = line.split('x').map(|n| n.parse().unwrap()).collect();

        let main_area = line_parts
            .iter()
            .combinations(2)
            .fold(0usize, |subtotal, pair| subtotal + 2 * pair[0] * pair[1]);

        assert!(line_parts.len() == 3);
        line_parts.sort_unstable();
        total + main_area + line_parts[0] * line_parts[1]
    })
}

fn part2(string: &str) -> usize {
    string.lines().fold(0usize, |total, line| {
        let mut line_parts: Vec<usize> = line.split('x').map(|n| n.parse().unwrap()).collect();
        assert!(line_parts.len() == 3);

        let volume: usize = line_parts.iter().product();

        line_parts.sort_unstable();
        let _ = line_parts.pop();

        let perimeter: usize = line_parts.iter().sum::<usize>() * 2;
        total + volume + perimeter
    })
}

fn main() {
    let text = fs::read_to_string("./input.txt")
        .expect("Can't read input file")
        .trim()
        .to_owned();

    // For our input.txt, the result was 280. However, youd Advent Of Code input will probably be different.
    println!("Part1 Result: {}", part1(&text));
    println!("Part2 Result: {}", part2(&text));
}
