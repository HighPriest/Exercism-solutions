use std::collections::HashSet;

const PRIMES:[i64; 30] = [ 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31,
    37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103,
    107, 109, 113 ];

// Solution inspired by: https://stackoverflow.com/a/28948975
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let mut anagrams: HashSet<&str> = HashSet::new();
    let word_hash = calculate_product(word);
    for candidate in possible_anagrams.iter() {
        // !Debug: let _a:String = candidate.to_ascii_lowercase();
        // !Debug: let _b:String = word.to_ascii_lowercase();
        if word_hash == calculate_product(candidate) && !candidate.to_ascii_lowercase().eq(&word.to_ascii_lowercase()) {
            anagrams.insert(candidate);
        }
    }

    anagrams
}

pub fn calculate_product(letters: &str) -> i64{
    let mut result:i64 = 1;
    for c in letters.to_ascii_lowercase().chars() {
        // !Debug: let _value:usize = c as usize - 96; 
        result *= PRIMES[c as usize - 96];
    }
    result
}