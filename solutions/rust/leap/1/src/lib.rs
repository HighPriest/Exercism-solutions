#![deny(clippy::all)]
//#![warn(clippy::pedantic)]
//#![allow(clippy::cast_possible_wrap)]
//#![allow(clippy::cast_sign_loss)]

pub fn is_leap_year(year: u64) -> bool {
    year % 4 == 0 
        &&
    (year % 100 != 0 || year % 400 == 0)
    //todo!("true if {year} is a leap year")
}
