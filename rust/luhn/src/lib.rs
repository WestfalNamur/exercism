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

fn check_code_form(s: &str) -> bool {

    // Check length
    if s.len() <= 1 {
        return false
    }

    // All chars are numeric?
    s.chars().all(|c| c.is_numeric())
}

fn double_digits(s: &str) -> String {
    //doubled_digits = Vec<u32>
    s.chars().rev().enumerate().map(|(i, c)| {
        if i%2==0 {
            let cx2 = c.to_digit(10) * 2;
            return cx2.to_char().unwrap();
        } else {
            return c
        }
    }).collect()
}

pub fn is_valid(code: &str) -> bool {

    // Check if input string is in valid form and clean it.
    let s: String = code.chars().filter(|c| !c.is_whitespace()).collect();
    if !check_code_form(&s) {
        return false
    }

    // Double every other digit (convert chars to numbers.)

    unimplemented!("Is the Luhn checksum for {} valid?", code);
}
