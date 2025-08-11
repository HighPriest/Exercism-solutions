use std::collections::HashSet;

const PRIMES: [u64; 64] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311,
];

// Solution inspired by: https://stackoverflow.com/a/28948975
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    // Instead of collecting a new Set, we can just filter values out of a "string SLICE".
    // The slice concept seems very important in Rust
    let word_hash = calculate_product(word);

    possible_anagrams
        // Execute following methods for individual elements
        .iter()
        // Filtration keeps values from the Set when values are true
        // In this case, when the values are different & if their lenghts are equal (only words composed of the same amount of characters can be anagrams)
        .filter(|candidate| {
            candidate.to_lowercase() != word.to_lowercase() && candidate.len() == word.len()
        })
        // This line used to be 
        // `.filter_map(|candidate| (word_hash == calculate_product(candidate)).then(|| *candidate))`
        // then
        // `.filter(|&candidate| (word_hash == calculate_product(candidate))).map(|candidate| *candidate)`
        // but by recommendation of Exercism bot, it has been replaced with the line below
        // All this does is filter the values, then replaces the list of pointers, with a list of values (copied literally copies values)
        .filter(|&candidate| (word_hash == calculate_product(candidate))).copied()
        // And finally, transforms the iterator enhanced Set, into a new Set
        .collect()
}

// We create two value checkers
// The first calculated coefficient, consists of letter positions binary shifted. This is an easy way to list all character signatures in a single variable
// The second coefficient, consists of letter values mapped to prime numbers and added together. This way we get an always unique hash of letters list.
pub fn calculate_product(letters: &str) -> (u64, u64) {
    // We don't lowercase, because we are going to handle different letter types in match
    // Turn the string into iterator over characters
    letters
        .chars()
        // Oooh, this one is called "fold", because it applies the result of calculation on top of previous value.
        // It presents an accumulation of previous values & next value to do with it as we please.
        // Could be used to implement a simple prev_values += next_value. Or used for something much more complex
        .fold((0, 0), |result, ch| {
            let ch_value = match ch {
                'a'..='z' => ch as u8 + 1 - b'a', // this is so clever compared to my initial idea. Instead of seeking the value of a character, just substract them...
                'A'..='Z' => ch as u8 + 1 - b'A',
                'Α'..='Ω' => (ch as u32 - 0x0391) as u8 + 27, // Handling greek characters by first moving them to the u8 ascii range
                'α'..='ω' => (ch as u32 - 0x03B1) as u8 + 27, // and then doing some weird + 27
                _ => 0,
                // This match needs to be so verbose, because we can encounter chars with the same code, that are not actual letters.
                // Values that are not letters that we are interested in, need to be dicarded.
            };
            (
                result.0 | (0x01 << ch_value), // Generates a binary representation
                result.1 + PRIMES[ch_value as usize] + if ch_value == 0 { ch as u64 } else { 0 },
            )
        })
}
