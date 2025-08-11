#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

use itertools::Itertools;
use std::collections::{HashMap, HashSet};
// The core idea is to transform equation before permutations testing, to calculate it as quickly as possible.
// The simplest data model for testing would be just a list of letter factors (coefficiens),
//     so that factors multiplied by letter values and summized will give 0 for correct solution.
// For that, we should got through the equation, sum and remember factors per letter,
//     determined by the letter position in the word (x1, x10, x100, etc).
// After "==" we should change the sign of the factors, and it's convenient to parse reversed input string.
// Sorting letters/factors also impact on performance, as I've found.
// Additionally, we have to check found solution against "no zero first" rule,
//     so we need to know which letters occurs at the first position in an encoded number.
// Explanation by gemini: https://gemini.google.com/app/bdd6b2c0ce864b45

#[must_use]
fn calc_factors(input: &str) -> (Vec<char>, Vec<i64>) {
    let mut factors = HashMap::new();
    // We start from the back, which means the values need to be moved to the left of equation, resulting in negative sign
    let mut sign = -1;
    let mut pos = 0; 

    for c in input.chars().filter(|c| !c.is_whitespace()).rev() {
        match c {
            '=' => {
                // If we cross to the left, now values have positive factor
                sign = 1;
                pos = 0;
            }
            // We have just crossed between words on the left side, reset factor power
            '+' => pos = 0,
            // If we encounter a letter
            _ => {
                // From list of factors
                *factors
                    // Find key matchin current letter
                    .entry(c)
                    // If doesn't exist, insert current sign
                    .or_insert(0) += sign 
                        // Times it's factor
                        * 10_i64.pow(pos);
                // Increase power for the next letter
                pos += 1;
            }
        }
    }
    
    // Return list of factors as sorted Vectors of letters and factors
    factors.into_iter()
        // Sort letters by their factor, starting from the biggest
        .sorted_by_key(|(_, v)| -v.abs())
        // turn collection of pairs, into two collections
        .unzip()
}

#[must_use]
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
// Entry point
// The program ends up creating and working on
    // a hashmap of first letters (to check for "No leading zero" rule)
    // Three sorted vectors: (Letters, Factors, Generated Values aka. Perm)
    // Output Hashmap of Letters to Generated Values


    // Make a collection of first characters in words. This is used to check for "No leading zero" rule
    // Explanation: If our algorithm tries to assign zero, to a character which is first in some word, we are violating a rule
    let firsts = input
        // Get individual 'words' in the equation (removing + and = signs in effect)
        .split(&['+', '='])
        // For each word
        .filter_map(|s| s
                // Remove whitespaces from words
                .trim()
                // Get first character of word
                .chars().next()
            )
        // Collect into hashmap
        .collect::<HashSet<_>>();
    // Collect list of letters AND their collective impact on the equation (factors)
    let (letters, factors) = calc_factors(input);

    // Brute-force search
    // The permutations function is part of `itertools`
    // The permutations function is going to generate all permutations of values between 0..=9,
        // consisting of however many letters are in letters collection
    for perm in (0u8..=9u8).permutations(letters.len()) {
        // Calculate sum of values assigned to each letter in this permutation, times letters assigned factor
        // It is of type i64, because that's the biggest denominator of its individual parts
        let sum = perm
            .iter()
            .enumerate()
            // Because factors can be BIG (of type i64), we quickly cas our 0..=9 (of type u8) to i64
            .map(|(i, v)| factors[i] * i64::from(*v))
            .sum::<i64>();

        // For the answer to be correct, we need the sum to be equal 0
        if sum == 0
            // && none of the letters which are first, can be equal to 0
            && !perm
                .iter()
                .enumerate()
                // Go through all letters, if its value is 0 && if it exists in firsts
                .any(|(i, v)| *v == 0 && firsts.contains(&letters[i]))
        {
            // Combine the collection of letters, with collection of values (perm)
            return Some(perm.iter()
                            .enumerate()
                            .map(|(i, v)| (letters[i], *v))
                            .collect::<HashMap<char,u8>>()
            );
        }
    }
    None
}
