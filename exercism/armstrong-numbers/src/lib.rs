pub fn is_armstrong_number(num: u32) -> bool {
    let s_num = num.to_string();
    let len = s_num.len() as u32;
    s_num
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(len))
        .sum::<u32>()
        == num
}
