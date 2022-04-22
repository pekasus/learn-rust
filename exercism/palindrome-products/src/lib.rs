/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        let number_in_string = value.to_string();
        let reverse_string: String = number_in_string.chars().rev().collect();
        if number_in_string == reverse_string {
            return Some(Palindrome(value));
        } else {
            return None;
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    //
    // MIN first
    let mut min_palindrome: Option<Palindrome> = None;
    'outer: for i in min..=max {
        if let Some(min_palindrome) = min_palindrome {
            if i * i > min_palindrome.into_inner() {
                break 'outer;
            }
        }
        for j in i..=max {
            let candidate = Palindrome::new(i * j);
            if candidate.is_some()
                && (min_palindrome.is_none()
                    || min_palindrome.unwrap().into_inner() > candidate.unwrap().into_inner())
            {
                min_palindrome = candidate;
                break;
            }
        }
    }

    let mut max_palindrome: Option<Palindrome> = None;
    'outer: for i in (min..=max).rev() {
        if let Some(max_palindrome) = max_palindrome {
            if i * i < max_palindrome.into_inner() {
                break 'outer;
            }
        }
        for j in (min..=i).rev() {
            let candidate = Palindrome::new(i * j);
            if candidate.is_some()
                && (max_palindrome.is_none()
                    || max_palindrome.unwrap().into_inner() < candidate.unwrap().into_inner())
            {
                max_palindrome = candidate;
                break;
            }
        }
    }

    if min_palindrome.is_some() {
        Some((min_palindrome.unwrap(), max_palindrome.unwrap()))
    } else {
        None
    }
}
