#![deny(clippy::all)]
//#![warn(clippy::pedantic)]
//#![allow(clippy::cast_possible_wrap)]
//#![allow(clippy::cast_sign_loss)]
//#![allow(clippy::cast_possible_truncation)]

pub fn build_proverb(list: &[&str]) -> String {
    match list.first() {
        None => String::new(), // If the list is empty, return empty
        Some(word) => list.windows(2)
            .map(|w| format!("For want of a {} the {} was lost.\n", w[0], w[1]))
            // with only one item, windows is not even going to create an iterator, so "map" is not going to be triggered
            .chain(std::iter::once(format!("And all for the want of a {}.", word))) 
            // "word" has been grabbed in the first match step!
            .collect()
    }

    // SOLUTION 2
    /* let result = String::new();
    if list.is_empty() {
        return result;
    }

    list.windows(2)
        .fold(result, |a,b|
            a + &format!("For want of a {} the {} was lost.\n", b[0], b[1]))
            + &format!("And all for the want of a {}.", list.first().unwrap_or(&"")) */
}