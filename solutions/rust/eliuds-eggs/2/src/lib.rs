#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

#[must_use]
pub fn egg_count(display_value: u32) -> usize {
    (0..32).fold(0, |result, n| {
        result + (((display_value >> n) & 1) as usize)
    })

    // display_value.count_ones() as usize

    /*
    let mut result = 0;
    let mut current_value = display_value;
    while current_value > 0 {
        if current_value % 2 == 1 {
            result += 1
        }
        current_value /= 2
    }
    result
    */
}
