use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    // let mut output: BTreeMap<char, i32> = BTreeMap::new();
    h.iter().fold(BTreeMap::new(), |mut acc, (score, c_vec)| {
        c_vec.iter().for_each(|&c| {
            acc.insert(c.to_ascii_lowercase(), *score);
        });
        acc
    })
}
