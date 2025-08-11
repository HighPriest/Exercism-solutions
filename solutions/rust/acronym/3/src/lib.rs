#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

#[must_use]
pub fn abbreviate(phrase: &str) -> String {
    phrase.chars().take(1)
        // This just takes first character, because we can't check for it after creating a window
        // Alternative:
            // format!(" {phrase}")
        // Alternative with checking if the first letter is alphabetic
            // .find(|c| c.is_ascii_alphabetic())
            // .map(|c| c.to_ascii_uppercase());
        .chain(
            // chain the rest of the checking
            phrase.as_bytes()
            // alternative to as_bytes is `.chars().collect::<Vec<char>>()`
            // collect needs type annontation, because FromIterator(char) can be collected into String aswell
                .windows(2)
                // Go over current and next value
                .filter(|pair| {
                    let is_camel = pair[0].is_ascii_lowercase() && pair[1].is_ascii_uppercase();
                    let is_new_word =
                        (pair[0].is_ascii_whitespace() || pair[0] == b'-' || pair[0] == b'_') && pair[1].is_ascii_alphabetic();
                    is_new_word || is_camel
                })
                .map(|pair| char::from(pair[1]).to_ascii_uppercase()),
        )
        .collect()
}