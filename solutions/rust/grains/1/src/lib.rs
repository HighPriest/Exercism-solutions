#![deny(clippy::all)]
//#![warn(clippy::pedantic)]
//#![allow(clippy::cast_possible_wrap)]
//#![allow(clippy::cast_sign_loss)]

#[must_use]
pub fn square(s: u32) -> u64 {
    2_u64.pow(s-1) 
    //todo!("grains of rice on square {s}");
}

pub fn total() -> u64 {
    (1..=64).fold(0, |result, value| {
        result + square(value)
    })
}
