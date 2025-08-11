#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

use std::collections::HashMap;
use std::thread;

#[must_use]
pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() {
        return HashMap::new();
    }

    // Thread Join Handlers
    let mut handles = Vec::new();
    // How many characters to go through for each worker
    //let chunk_size = (input.len() + worker_count - 1) / worker_count;
    let chunk_size = input.len().div_ceil(worker_count);

    // Divide the input into individual "chunks"
    for chunk in input.chunks(chunk_size) {
        // Clone the chunk into a String so it can be moved into the thread
        let chunk: String = chunk.iter().flat_map(|s| s.chars()).collect();
        // Create thread for each worker
        let handle = thread::spawn(move || {
            // Store frequency of each letter in a var
            let mut freq = HashMap::new();
            // Go over all chars
            for c in chunk.chars().filter(|c| c.is_alphabetic()) {
                // If a char doesn't exist in hashmap, add it with 0. If exists add 1 to its count
                *freq.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
            }
            freq
        });
        handles.push(handle);
    }

    let mut frequency = HashMap::new();
    for handle in handles {
        match handle.join() {
            Ok(thread_freq) => {
                for (c, count) in thread_freq {
                    *frequency.entry(c).or_insert(0) += count;
                }
            }
            Err(_) => {
                eprintln!("A worker thread panicked and its results are ignored.");
            }
        }
    }

    frequency
}
