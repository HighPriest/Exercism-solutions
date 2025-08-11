#![deny(clippy::all)]
#![warn(clippy::pedantic)]

#[must_use]
pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 || digits.len() < len {
        return Vec::new();
    }

    let mut result = Vec::new();
    for i in 0..=digits.len() - len {
        result.push(digits[i..i + len].to_string());
    }
    
    result
}
