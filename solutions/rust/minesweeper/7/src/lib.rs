#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

#[must_use]
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // Convert to bytes because operations on chars (utf-8) is slower
    // chars.nth is O(n) operation
    let minefield_bytes = minefield.iter().map(|row| row.as_bytes()).collect::<Vec<_>>();

    let rows = minefield_bytes.len();
    let cols = minefield_bytes.first().map_or(0, |first| first.len());

    // Declaring a function inside a function. NEAT!
    // This pattern allows us to do recursive work while utilising previously declared variables.
    // (Notice that I am using minefield_bytes in this function, without passing it)
    let count_mines = |i: usize, j: usize| {
        // saturating_sub sticks to the lower_bound of the type instead of wrapping around the infinity when underflowed.
        // usize's lower_bound is 0. You can use this to handle index out of range edge case
        (i.saturating_sub(1)..=(i + 1).min(rows - 1))
            // (i+1).min(rows - 1) & (j+1).min(cols - 1) allows us to only go up if we are not going to go out of bounds!
            .flat_map(|x| (j.saturating_sub(1)..=(j + 1).min(cols - 1))
                // Collect values around current spot, into a flat array
                .map(move |y| (x, y))
            )
            // Remove all values which are not a bomb and give bombs value of 1
            .filter_map(|(x, y)| (minefield_bytes[x][y] == b'*').then_some(1))
            // Sum all bombs, which now have value of 1
            .sum()
    };

    // Key to solving this cleanly is using Option. Option/Result are iterable in Rust. That means methods such as flat_map can unwrap a Option/Result collection into unwrapped value collection.

    // Disassemble the minefield into iterator over rows
    minefield_bytes.iter().enumerate().map(|(i, row)| {
        // Disassemble each row into iterator over individual characters.
        // Then filter_map, because for each input (byte), we are returning one output value (char)
        row.iter().enumerate().filter_map(|(j, &ch)| {
            // Check for mine
            (ch == b'*')
                // If there is a mine, we leave the character there
                .then_some('*')
                .or_else(|| {
                    let mines = count_mines(i, j);
                    // Special case: Leave empty space for 0 mines
                    (mines == 0)
                        .then_some(' ')
                        .or_else(|| char::from_digit(mines, 10))
                })
        }).collect::<String>()
    }).collect::<Vec<String>>()
}
