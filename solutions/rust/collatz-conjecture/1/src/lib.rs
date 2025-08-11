#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

#[must_use]
pub fn collatz(n: u64) -> Option<u64> {
    let mut steps = 0;
    let mut n= n;
    while n != 1  {
        match n {
            0 => return None,
            _ if n % 2 == 0 => n /= 2,
            _ => n = 3 * n + 1,
        }
        steps += 1;
    }
    Some(steps)
}
