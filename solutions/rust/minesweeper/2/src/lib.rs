#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // Special case when minefield is empty
    if minefield.is_empty() {
        return vec![];
    }

    // Create an empty copy of the minefield, to be filled with positions of bombs & counts of neighboring bombs
    let mut annotated_field = vec![vec![' '; minefield[0].len()]; minefield.len()];

    // Iterate over each row in the minefield
    // Add iterator to each row we get
    // Capture the iterators index & contents of the row
    minefield.iter().enumerate().for_each(|(y, row)| {
        // Iterate over characters in each row
        // Add iterator to each character we get
        // Capture the iterators index & contents of the character
        row.chars().enumerate().for_each(|(x, _)| {
            // Count bombs or return bomb character
            annotated_field[y][x] = count_bombs(minefield, y as isize, x as isize).unwrap_or(' ');
        });
    });

    // Convert annotated_field to Vec<String>
    annotated_field
        .into_iter()
        .map(|row| row.into_iter().collect()) // Now collect each row of characters into a string
        .collect()
}

#[must_use]
fn count_bombs(minefield: &[&str], y: isize, x: isize) -> Option<char> {
    if minefield[y as usize].chars().nth(x as usize) == Some('*') {
        Some('*') // Keep the mine if it exists in original field
    } else {
        // Define the possible positions of bombs to check
        let neighbors: [(isize, isize); 8] = [
            (y - 1, x - 1), // top-left
            (y - 1, x),     // top
            (y - 1, x + 1), // top-right
            (y, x - 1),     // left
            (y, x + 1),     // right
            (y + 1, x - 1), // bottom-left
            (y + 1, x),     // bottom
            (y + 1, x + 1), // bottom-right
        ];

        // Take each neighbor & do filtration on those that do not match all the conditions
        let mine_count = neighbors
            .iter()
            .filter(|&&(ny, nx)| {
                // Check if we are within y bounds
                ny >= 0 && ny < minefield.len() as isize &&
        // Check if we are within x bounds
        nx >= 0 && nx < minefield[0].len() as isize &&
        // Check if the neighboring cell contains a mine in the original field
        minefield[ny as usize].chars().nth(nx as usize) == Some('*')
            })
            .count(); // Count how many are left after filtration & set this value to mine_count

        // If there are one or more mines around the cell, we can replace the placeholder '_'
        if mine_count > 0 {
            // We replace the placeholder with the calculated value, transformed into a character
            Some(char::from_digit(mine_count as u32, 10).unwrap())
        } else {
            None
        }
    }
}
