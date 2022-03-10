use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

fn process_single_input(mut stack: Vec<i32>, input: CalculatorInput) -> Option<Vec<i32>> {
    let result: i32 = if let CalculatorInput::Value(value) = input {
        value
    } else {
        let b = stack.pop()?;
        let a = stack.pop()?;
        let operation = match input {
            CalculatorInput::Add => Add::add,
            CalculatorInput::Subtract => Sub::sub,
            CalculatorInput::Multiply => Mul::mul,
            CalculatorInput::Divide => Div::div,
            CalculatorInput::Value(_) => unreachable!(),
        };
        operation(a, b)
    };
    stack.push(result);
    Some(stack)
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    inputs
        .iter()
        .copied()
        .try_fold(Vec::new(), process_single_input)
        .and_then(|stack| (stack.len() == 1).then(|| stack[0]))
}
