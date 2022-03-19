#![feature(vec_retain_mut)]

use std::collections::HashMap;

#[derive(PartialEq, Clone, Copy)]
enum Operation {
    Assign,
    Not,
    And,
    Or,
    LShift(u8),
    RShift(u8),
}

#[derive(Clone)]
struct Instruction {
    operation: Operation,
    values: [Option<u16>; 2],
    output_key: String,
    keys: [Option<String>; 2],
}

impl Instruction {
    /// Return true if successfully processed the instruction and completed it.
    fn try_process(&mut self, computed: &mut HashMap<String, u16>) -> bool {
        let is_and_or = self.operation == Operation::And || self.operation == Operation::Or;

        if is_and_or {
            if self.value_is_some(0, &computed) && self.value_is_some(1, &computed) {
                let output_value: u16;
                if self.operation == Operation::And {
                    output_value = self.values[0].unwrap() & self.values[1].unwrap()
                } else {
                    output_value = self.values[0].unwrap() | self.values[1].unwrap()
                }
                computed.insert(self.output_key.clone(), output_value);
                return true;
            }
        } else {
            if self.value_is_some(0, &computed) {
                let output_value: u16;
                if self.operation == Operation::Not {
                    output_value = !self.values[0].unwrap();
                } else if let Operation::LShift(shift) = self.operation {
                    output_value = self.values[0].unwrap() << shift;
                } else if let Operation::RShift(shift) = self.operation {
                    output_value = self.values[0].unwrap() >> shift;
                } else if Operation::Assign == self.operation {
                    output_value = self.values[0].unwrap();
                } else {
                    unreachable!();
                }
                computed.insert(self.output_key.clone(), output_value);
                return true;
            }
        }
        return false;
    }

    fn value_is_some(&mut self, idx: usize, computed: &HashMap<String, u16>) -> bool {
        if self.values[idx].is_some() {
            return true;
        } else {
            let value_option = computed.get(&self.keys[idx].clone().unwrap());
            if let Some(&value) = value_option {
                self.values[idx] = Some(value);
                return true;
            } else {
                return false;
            }
        }
    }
}

//fn value(i: usize) -> Option<u16>

fn main() -> () {
    let input_lines = std::fs::read_to_string("input.txt")
        .expect("Could not read from file.")
        .trim()
        .to_owned();
    let input_lines = input_lines.lines();

    let instructions = parse_lines_to_instructions(input_lines);
    let first_run_a = find_a(instructions.clone(), HashMap::new(), None);
    println!("Result Part A: {}", first_run_a);

    // instructions - remove "b"
    //instructions.retain(f)

    let mut computed = HashMap::new();
    computed.insert("b".to_owned(), first_run_a);
    let second_run_a = find_a(instructions, computed, Some("b".to_owned()));
    println!("Result Part B: {}", second_run_a)
}

fn parse_lines_to_instructions<'a>(input_lines: impl Iterator<Item = &'a str>) -> Vec<Instruction> {
    let instructions = input_lines.map(|line| {
        let line: Vec<&str> = line.split_ascii_whitespace().collect();
        let value1;
        if line[0].chars().all(|c| c.is_numeric()) {
            value1 = Some(line[0].parse::<u16>().unwrap());
        } else {
            value1 = None;
        }

        let instruction = if line[0] == "NOT" {
            Instruction {
                operation: Operation::Not,
                values: [value1, None],
                output_key: line[3].to_owned(),
                keys: [Some(line[1].to_owned()), None],
            }
        } else {
            let operation = match line[1] {
                "AND" => Operation::And,
                "OR" => Operation::Or,
                "LSHIFT" => Operation::LShift(line[2].parse().unwrap()),
                "RSHIFT" => Operation::RShift(line[2].parse().unwrap()),
                "->" => Operation::Assign,
                _ => unreachable!(),
            };
            let key2 = if operation == Operation::And || operation == Operation::Or {
                Some(line[2].to_owned())
            } else {
                None
            };
            let output_key_index = if Operation::Assign == operation { 2 } else { 4 };
            Instruction {
                operation,
                output_key: line[output_key_index].to_owned(),
                values: [value1, None],
                keys: [Some(line[0].to_owned()), key2],
            }
        };
        instruction
    });
    instructions.collect()
}

fn find_a(
    mut pending_instructions: Vec<Instruction>,
    mut computed: HashMap<String, u16>,
    ignore_string: Option<String>,
) -> u16 {
    let mut remaining_instructions = pending_instructions.len();
    while computed.get("a").is_none() {
        pending_instructions.retain_mut(|instruction| {
            if ignore_string.is_some() {
                if instruction.output_key == ignore_string.clone().unwrap() {
                    return false;
                }
            }
            let processed = instruction.try_process(&mut computed);
            !processed
        });
        if pending_instructions.len() == remaining_instructions {
        } else {
            remaining_instructions = pending_instructions.len();
        }
    }

    *computed.get("a").unwrap()
}
