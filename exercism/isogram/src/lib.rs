use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut letter_map = HashSet::new();
    for c in candidate.chars() {
        let c = c.to_ascii_lowercase();
        if c != '-' && c != ' ' {
            if letter_map.contains(&c) {
                return false;
            } else {
                letter_map.insert(c);
            }
        }
    }
    true
}
