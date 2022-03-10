use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    let primes: Vec<u64> = (2..p)
        .filter(|num| (2..(p / 2 + 1)).all(|x| num % x != 0))
        .collect();
    let index = rng.gen::<usize>() % primes.len();
    *primes.iter().nth(index).unwrap()
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    let mut result = 1;
    let mut g = g % p;
    if g == 0 {
        return 0_u64;
    }

    let mut a = a;
    while a > 0 {
        if (a & 1) == 1 {
            result = (result * g) % p;
        }
        a >>= 1;
        g = (g * g) % p;
    }
    result
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    public_key( p, b_pub, a)
}
