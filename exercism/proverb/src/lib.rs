pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }
    let mut vec: Vec<String> = list
        .windows(2)
        .map(|pair| format!("For want of a {} the {} was lost.\n", pair[0], pair[1]))
        .collect();
    vec.push(format!("And all for the want of a {}.", &list[0]));
    vec.concat()
}
