use std::fs;

fn santa_move((x, y): (i32, i32), direction: char) -> (i32, i32) {
    match direction {
        '>' => (x + 1, y),
        '<' => (x - 1, y),
        '^' => (x, y + 1),
        'v' => (x, y - 1),
        _ => unreachable!("Bad char {}", direction),
    }
}

fn part1(input: &str) -> usize {
    let mut houses = std::collections::HashSet::new();
    houses.insert((0, 0));

    input.chars().fold((0, 0), |position, direction| {
        let new_position = santa_move(position, direction);
        houses.insert(new_position);
        new_position
    });

    houses.len()
}

fn part2(input: &str) -> usize {
    let mut houses = std::collections::HashSet::new();
    houses.insert((0, 0));
    let (santa_instruction1, santa_instruction2): (Vec<(usize, char)>, Vec<(usize, char)>) =
        input.chars().enumerate().partition(|(idx, _)| idx % 2 == 0);
    let santa_instructions = vec![santa_instruction1, santa_instruction2];
    santa_instructions.iter().for_each(|instructions| {
        instructions
            .iter()
            .fold((0, 0), |position, (_, direction)| {
                let new_position = santa_move(position, *direction);
                houses.insert(new_position);
                new_position
            });
    });
    houses.len()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read File.");
    let input = input.trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}
