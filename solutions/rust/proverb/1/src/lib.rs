#![deny(clippy::all)]
//#![warn(clippy::pedantic)]
//#![allow(clippy::cast_possible_wrap)]
//#![allow(clippy::cast_sign_loss)]
//#![allow(clippy::cast_possible_truncation)]

pub fn build_proverb(list: &[&str]) -> String {
    let result = String::new();
    if list.is_empty() {
        return result;
    }

    list.windows(2)
        .fold(result, |a,b|
            a + &format!("For want of a {} the {} was lost.\n", b[0], b[1]))
            + &format!("And all for the want of a {}.", list.first().unwrap_or(&""))
}