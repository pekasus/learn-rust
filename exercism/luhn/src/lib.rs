pub fn is_valid(code: &str) -> bool {
    code.chars()
        .filter(|&character| character != ' ')
        .rev()
        .map(|character| character.to_digit(10))
        .enumerate()
        .map(|(index, digit)| {
            digit.map(|digit| {
                let should_double = index % 2 == 1;
                let digit = if should_double {
                    let digit = digit * 2;
                    if digit > 9 {
                        digit - 9
                    } else {
                        digit
                    }
                } else {
                    digit
                };
                (index, digit)
            })
        })
        .try_fold((0, 0), |(_index, sum), digit| {
            digit.map(|(index, digit)| (index, sum + digit))
        })
        .map(|(index, sum)| {
            let count = index + 1;
            count > 1 && sum % 10 == 0
        })
        .unwrap_or(false)
}
