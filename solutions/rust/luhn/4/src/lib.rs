#![deny(clippy::all)]
#![warn(clippy::pedantic)]
// #![allow(clippy::cast_possible_wrap)]
// #![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]

/// Check a Luhn checksum.
pub fn is_valid(code_: &str) -> bool {
    let code = code_.replace(' ', "");

    if code.len() > 1 {
        match calc_checksum(&code[..code.len() - 1]) { // Get slice of the string, without last digit (which is the checksum)
            None => false,
            Some(checksum) => code_.ends_with((b'0' + checksum as u8) as char) 
                                    // char::from_u32(checksum) is not going to work here
                                    // because it was designed to convert a Unicode code point into a char!
                                    // We convert the digit directly to a char, by adding the ascii value for '0' (equal 48).
                                    // Now our checksum is understood as an ASCII character & converted succesfully
        }
    } else {
        false
    }
}

#[must_use]
pub fn calc_checksum(code: &str) -> Option<u32> {
    code.chars()
        .try_rfold((0u32, 0u32), |(len, sum), c| { // execute reverse fold (with iterator, sum & current char) to start summing up values from the back
            c.to_digit(10).map(|num| (len + 1, sum + luhn(len, num))) // convert char to digit, calculate luhn, add it to the sum & up the iterator
        })
        .map(|(_, sum)| (10 - (sum % 10)) % 10) // final unzip of the Option into the resultant checksum
}

/// Apply Luhn formula
fn luhn(i: u32, num: u32) -> u32 {
    match num {
        0..=4 if i % 2 == 0 => num * 2, // When we are on position divisible by 2, do multiplication
        5..=9 if i % 2 == 0 => num * 2 - 9, // Same, but we are going to overflow 10
        _ => num,
    }
}
