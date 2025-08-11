#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

#[must_use]
pub fn egg_count(display_value: u32) -> usize {
    // (0..32).map (|n| (display_value >> n) & 1)
    (0..32).fold(0, |result, n| {
        result + (((display_value >> n) & 1) as usize)
    })
}
