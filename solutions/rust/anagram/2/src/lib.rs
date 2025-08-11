use std::collections::HashSet;

const PRIMES:[i64; 30] = [ 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31,
    37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103,
    107, 109, 113 ];

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
        .filter(|candidate| candidate.to_lowercase() != word.to_lowercase() && candidate.len() == word.len())
        // TO LEARN: This thing does some filter_map of a *candidate pointer. But the part before then, is it a shorthand if?
        .filter_map(|candidate| (word_hash == calculate_product(candidate)).then(|| *candidate))
        // Transform the iterator enhanced Set, into a new Set
        .collect()
}

pub fn calculate_product(letters: &str) -> i64{
    let mut result:i64 = 1;
    for c in letters.to_ascii_lowercase().chars() {
        // !Debug: let _value:usize = c as usize - 96; 
        result *= PRIMES[c as usize - 96];
    }
    result
}