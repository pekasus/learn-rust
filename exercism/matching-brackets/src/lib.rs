#[derive(PartialEq, Eq)]
enum LeftBracket {
    Square,
    Curly,
    Paren,
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let stack: Option<Vec<LeftBracket>> =
        string
            .chars()
            .try_fold(Vec::<LeftBracket>::new(), |mut stack, c| {
                let bracket: Result<LeftBracket, char> = c.try_into();
                if let Ok(bracket) = bracket {
                    stack.push(bracket);
                } else {
                    let expected_top = match c {
                        '}' => Some(LeftBracket::Curly),
                        ']' => Some(LeftBracket::Square),
                        ')' => Some(LeftBracket::Paren),
                        _ => None,
                    };
                    if let Some(expected_top) = expected_top {
                        let top = stack.pop()?;
                        if top != expected_top {
                            return None;
                        }
                    }
                }
                Some(stack)
            });

    if let Some(stack) = stack {
        stack.is_empty()
    } else {
        false
    }
}

impl TryFrom<char> for LeftBracket {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '[' => Ok(LeftBracket::Square),
            '(' => Ok(LeftBracket::Paren),
            '{' => Ok(LeftBracket::Curly),
            _ => Err(value),
        }
    }
}
