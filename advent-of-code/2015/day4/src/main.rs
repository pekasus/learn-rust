use std::fs;

fn search_hash_with_prefix(input: &str, hash_prefix: &str) -> usize {
    (1..)
        .find(|x| {
            let hash = md5::compute(format!("{}{}", input, x));

            let hash = format!("{:?}", hash);
            hash.starts_with(hash_prefix)
        })
        .unwrap()
}

fn search_hash_with_bytes(input: String) {
    let digest = md5::compute(input);
    println!("{:?}", digest.0[0].leading_zeros());
    println!("{:?}", digest.0);
    // println!("{:?}", digest.0as_bytes());
}

fn main() {
    let _input = fs::read_to_string("input.txt").expect("Could not read from File.");
    // println!("part1: {}", search_hash_with_prefix(&input, "00000"));
    // println!("part2: {}", search_hash_with_prefix(&input, "000000"));
    search_hash_with_bytes("abcdef609043".to_owned())
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        use crate::search_hash_with_prefix;
        let _digest = md5::compute("abcdef609043");

        assert_eq!(search_hash_with_prefix(&"abcdef", "00000"), 609043);
    }
}

// [00,00,00,00,01,02,03,04...]
// [00,00,00,00]
// hex from has as bytes

//
