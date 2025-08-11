#![deny(clippy::all)]
//#![warn(clippy::pedantic)]
//#![allow(clippy::cast_possible_wrap)]
//#![allow(clippy::cast_sign_loss)]
//#![allow(clippy::cast_possible_truncation)]

// use std::iter::{self}; for Method 1

pub fn nth(n: u32) -> u32 {
    /* METHOD 1
    iter::successors(   // Create infinite iterator
        Some(2),        // Starting from two
        |x| Some(x+1)   // Next value being incremented by 1
    ).filter(|&x| is_prime(x))  // Add values to output list, which fulfill a check
    .nth(n as usize)    // Get only the n-th value from the list
    .unwrap_or(0)   // Just return value, without condition. 
                    // If there were no primes found return 0
    */

    /* METHOD 2
    (2..)
    .filter(|&x| is_prime(x))  // Add values to output list, which fulfill a check
    .nth(n as usize)    // Get only the n-th value from the list
    .unwrap_or(0)   // Just return value, without condition. 
                    // If there were no primes found return 0
    */

    // METHOD 3
    (2..)
    .filter(|x| is_prime(*x))
    .nth(n as usize)
    .unwrap_or(0)
}

#[must_use]
fn is_prime(n: u32) -> bool {
    /* METHOD 1
    match n {
        0 | 1           => false,
        2               => true,
        _ if n % 2 == 0 => false,
        _ => {
            let sqrt_n = f64::from(n).sqrt().floor() as u32;  // Calculate the lowest value where a divider can be found
            (2..=sqrt_n).all(|d| n % d != 0)                // Check this range of values if a divider exists
        }
    }
    */

    // METHOD 2
    !                                       // Negate output.
                                            // If something is found, we return false & vice-versa
    (2..=(f64::from(n).sqrt() as u32))     // Iterate from 2, to n.sqrt()
    .any(|x| n % x == 0)                    // Return true if ANY check returns true
}
