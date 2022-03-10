fn is_prime(n: u64) -> bool {
    !(2..n / 2 + 1).any(|x| n % x == 0)
}
pub fn nth(n: u32) -> u32 {
    (2..).filter(|x| is_prime(*x)).nth(n as usize).unwrap() as u32
}
