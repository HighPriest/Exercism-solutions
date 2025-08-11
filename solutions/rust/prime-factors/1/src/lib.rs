#![deny(clippy::all)]
//#![warn(clippy::pedantic)]
//#![allow(clippy::cast_possible_wrap)]
//#![allow(clippy::cast_sign_loss)]
//#![allow(clippy::cast_possible_truncation)]

// Thanks to: https://rustp.org/number-theory/prime-factorization-of-a-number/

pub fn factors(mut n: u64) -> Vec<u64> {
    // The problem we are facing is called "Prime Factorization of a number"
    let mut prime_factors: Vec<u64> = Vec::new();

    // Step 1 : Divide by 2
    while n & 1 == 0 {          // if the number is even, divide it by 2 and add 2 to the list of factors
        n >>= 1;                // shift right by one bit (dividing by 2)
        prime_factors.push(2);  // push 2 to the list of factors
    }

    // Step 2 : Start from 3, and go till square root of n
    let mut i = 3;
    while i<<1 <= n { // while `i` is less than square root of our number

        // Step 3 : Check if i is factor of n
        while n % i == 0 {          // make sure to check for repetitions
            n /= i;                 // divide the number by i
            prime_factors.push(i);  // add `i` to the list of factors
        }
        i += 2; // Add 2 to `i` to skip even numbers
    }

    // Step 4 : When we run out of possible primes. Check if n become 1 or not.
    if n > 1 {  // If we still haven't divided the number with primes
        prime_factors.push(n);  // Add the non-prime to the end of list
    }

    prime_factors
}