use regex::Regex;
use std::fs;
use std::str::Lines;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines();
    println!("Part 1: {}", part1(lines.clone()));
    println!("Part 2: {}", part2(lines));
}

fn part1(lines: Lines) -> usize {
    lines
        .map(|line| {
            println!("{}", line);
            let input_length = line.len();
            let line = &line[1..input_length - 1];
            let re1 = Regex::new(r#"\\\\"#).unwrap();
            let line = re1.replace_all(line, "a").as_ref().to_owned();
            let re2 = Regex::new(r#"\\""#).unwrap();
            let line = re2.replace_all(&line, "b").as_ref().to_owned();
            let re3 = Regex::new(r#"\\x[0-9a-f]{2}"#).unwrap();
            let line = re3.replace_all(&line, "c").as_ref().to_owned();
            let output_length: usize = line.len();
            println!("{}: {} | {}", line, input_length, output_length);
            input_length - output_length
        })
        .sum()
}

fn part2(lines: Lines) -> usize {
    lines.map(|line| {
        let input_length = line.len();
        println!("original: {}", line);
        let re1 = Regex::new(r#"\\"#).unwrap();
        let line = re1.replace_all(&line, "\\\\").as_ref().to_owned();
        println!("replaced backslashes: {}", line);
        let re2_string = r#"""#;
        //println!("re2_string: {}", re2_string);
        let re2 = Regex::new(re2_string).unwrap();
        let line = re2.replace_all(&line, "\\\"");
        println!("replaced quotes: {}", line);
        line.len() - input_length + 2
    }).sum()
}
