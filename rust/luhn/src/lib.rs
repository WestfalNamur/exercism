/// Check a Luhn checksum.
///
/// Strings of length 1 or less are not valid. Spaces are allowed in the input,
/// but they should be stripped before checking. All other non-digit characters
/// are disallowed.
///
/// Steps:
///     1. double every second digit, starting from the right, if doubling the
///     number results in a number greater than 9 then subtract 9 from the
///     product.
///     2. Sum all digits
///     3. If the sum is evenly divisible by 10, then the number is valid.
///     This number is valid!


fn check_form(s: &str) -> bool {

    // Check length
    if s.len() <= 1 {
        return false
    }
    
    // Check if all numeric
    if !s.chars().all(|c| c.is_ascii_digit()) {
        return false
    }
    
    true
}


pub fn is_valid(code: &str) -> bool {

    // Remove white spaces (They are valid chars but need to be removed).
    let s: String = code.chars().filter(|c| !c.is_whitespace()).collect();

    // Check form.
    if !check_form(&s) {
        return false;
    }

    // Convert to numbers, and double every other digit.
    let doubled_digits: Vec<u8> = s.as_bytes().iter().rev().enumerate().map(|(i,&d)| match d {
        n if i % 2 == 0 => n,
        n if n < 5 => n * 2,
        n => n * 2 - 9,
    }).collect();

    // Cast to u32 (avoid overflow) and sum up.
    let sum_doubled_digits: u32 = doubled_digits.iter().map(|&d| d as u32).sum();

    sum_doubled_digits % 10 == 0
}
