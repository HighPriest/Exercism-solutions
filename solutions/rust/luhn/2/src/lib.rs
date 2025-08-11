#![deny(clippy::all)]
#![warn(clippy::pedantic)]
// #![allow(clippy::cast_possible_wrap)]
// #![allow(clippy::cast_sign_loss)]

/// Check a Luhn checksum.
pub fn is_valid(code_: &str) -> bool {
    let code = code_.replace(' ', "");

    if code.len() > 1 {
        match calc_checksum(code.get(0..(code.len().saturating_sub(1))).unwrap()) {
            None => false,
            Some(code) => code_.ends_with(code.to_string().chars().last().unwrap()),
        }
    } else {
        false
    }
}

#[must_use]
pub fn calc_checksum(code: &str) -> Option<u32> {
    let mut sum = 0;

    for (i, c) in code.chars().enumerate() {
        // For loop, closeable with `return`. Using .for_each doesn't allow for early exit
        match c.to_digit(10) {
            // Convert & check if the character is a digit
            None => return None, // If not a digit, exit conversion
            Some(digit) => {
                if (i + code.len()) % 2 == 0 {
                    // check parity. If we are on position not divisible by 2, don't do multiplication.
                    // This uses code.len() as an offset for calculating whether i is in n%2 place from start.
                    // Imagine it as if code.len would fill a buffer & each i would cause overflow + 1.
                    sum += digit;
                } else if digit > 4 {
                    // We passed the non-multiplying stage first, so we can check if we don't overflow 10
                    sum += 2 * digit - 9;
                } else {
                    // When we are on position divisible by 2, do multiplication
                    sum += 2 * digit;
                }
            }
        }
    }
    Some((10 - (sum % 10)) % 10)
}
