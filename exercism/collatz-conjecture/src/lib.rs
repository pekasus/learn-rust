pub fn collatz(mut n: u64) -> Option<u64> {
    let mut counter = 0;

    if n == 0 {
        return None;
    }
    while n != 1 {
        if n % 2 ==0 {
            n = n / 2;
        } else {
            n = n.checked_mul(3)?.checked_add(1)?;
        }
        counter += 1;
    }
    
    Some(counter)
}
