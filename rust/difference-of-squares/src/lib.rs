pub fn square_of_sum(n: u32) -> u32 {
    (0..n+1).fold(0, |sum, val| sum + val).pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (0..n+1).fold(0, |sum, val| sum + val.pow(2))
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
