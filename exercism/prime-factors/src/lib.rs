pub fn factors(n: u64) -> Vec<u64> {
    let mut vec: Vec<u64> = vec![];
    let mut i = 2;
    let mut n = n;
    while n != 1 {
        while n % i == 0 {
            vec.push(i);
            n /= i;
        }
        i += 1;
    }
    vec
}
