#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    }
    if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }
    number.iter().try_for_each(|&digit| {
        if digit >= from_base {
            Err(Error::InvalidDigit(digit))
        } else {
            Ok(())
        }
    })?;

    let converted_number = convert_from_by_multiplying(number, from_base);
    Ok(convert_to(converted_number, to_base))
}

fn convert_from_by_multiplying(number: &[u32], from_base: u32) -> u32 {
    number.iter().fold(0, |mut result, &digit| {
        result *= from_base;
        result += digit;
        result
    })
}

fn convert_from(number: &[u32], from_base: u32) -> u32 {
    let mut power = number.len() as u32 - 1;
    let mut output = 0u32;
    for n in number {
        output += n * (from_base.pow(power));
        power -= 1;
    }
    output
}

fn convert_to(number: u32, to_base: u32) -> Vec<u32> {
    let mut number: u32 = number;
    let mut rev_output: Vec<u32> = vec![];
    if number == 0 {
        return vec![0];
    }
    while number > 0 {
        let digit = number % to_base;
        number /= to_base;
        rev_output.push(digit);
    }
    let output: Vec<u32> = rev_output.iter().rev().map(|&n| n).collect();
    output
}

// 16 = 2^4
// 17 -> base(2) =
