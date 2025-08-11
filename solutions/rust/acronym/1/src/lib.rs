#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

#[must_use]
pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| !c.is_alphabetic() && c != '\'')
        .filter(|w| !w.is_empty())
        .flat_map(|word| {
            word.chars().take(1).chain(
                word.chars()
                    .skip_while(|ch| ch.is_uppercase())
                    .filter(|ch| { ch.is_uppercase()})
                )
        }).flat_map(char::to_uppercase)
        .collect()
}
