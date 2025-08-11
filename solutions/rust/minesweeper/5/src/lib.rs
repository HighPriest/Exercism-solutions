#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

#[must_use]
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // Turn minefield into Array of Array of bytes instead of strs
    let minefield_bytes = minefield
        .iter()
        .map(|row| row.as_bytes())
        .collect::<Vec<_>>();

    // Create size vars for readability 
    let rows = minefield_bytes.len();
    let cols = minefield_bytes.first().map_or(0, |first| first.len());


    // Iterate over the whole minefield
    minefield_bytes.iter().enumerate()
        // map all rows
        .map(|(i, row)| { 
            // map all characters (like going through columns)
            row.iter().enumerate()
                .filter_map(|(j, &ch)| {
                    // Check if we are on a bomb
                    (ch == b'*')
                        // If on bomb, return bomb char
                        .then_some('*')
                        // If not on bomb, count bombs
                        .or_else(|| {
                            // Minefield has to be passed by reference, because we are using it at the same time as iterator
                            // Alternative would be to make a copy here, but we are not editing the value inside, so it is better to pass by reference
                            let mines = count_mines(&minefield_bytes, rows, cols, i, j);
                            // Special case: No bombs around = empty space, not 0
                            (mines == 0)
                                .then_some(' ')
                                .or_else(|| char::from_digit(mines, 10))
                        })
                })
                .collect()
        })
        .collect()
}

#[must_use]
fn count_mines(minefield_bytes: &[&[u8]], rows: usize, cols: usize, i: usize, j: usize) -> u32 {    
    (i.saturating_sub(1)..=(i + 1).min(rows - 1))
        .flat_map(|x| (j.saturating_sub(1)..=(j + 1).min(cols - 1)).map(move |y| (x, y)))
        .filter_map(|(x, y)| (minefield_bytes[x][y] == b'*').then_some(1))
        .sum()
}
