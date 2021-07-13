pub fn nth(n: u32) -> u32 {
    (2_u32..)
        .filter(|&i| is_prime(i))
        .nth(n as usize)
        .unwrap()
}

fn is_prime(n: u32) -> bool {
    match n {
        0 | 1 => false,
        n => (2..n).all(|i| n % i != 0)
    }
}
