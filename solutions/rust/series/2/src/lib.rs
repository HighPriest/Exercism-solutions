#![deny(clippy::all)]
#![warn(clippy::pedantic)]

#[must_use]
pub fn series(digits: &str, len: usize) -> Vec<String> {
    if digits.is_empty() || len > digits.len() {
        return Vec::new();
    }

    digits
        .chars()
        .collect::<Vec<_>>()
        .windows(len)
        .map(|window| window.iter().collect::<String>())
        .collect()
}
