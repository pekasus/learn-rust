use std::iter::zip;

/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        None
    } else {
        Some(
            zip(s1.chars(), s2.chars())
                .filter(|(c1, c2)| c1 != c2)
                .count(),
        )
    }
}
