#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

#[must_use]
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.chars()
                .enumerate()
                .filter_map(|(j, ch)| {
                    (ch == '*').then_some(ch).or_else(|| {
                        let mines = count_mines(minefield, i, j);
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
fn count_mines(minefield: &[&str], i: usize, j: usize) -> u32 {
    let rows = minefield.len();
    let cols = minefield.first().map_or(0, |first| first.len());
    let bytes = minefield
        .iter()
        .map(|row| row.as_bytes())
        .collect::<Vec<_>>();

    (i.saturating_sub(1)..=(i + 1).min(rows - 1))
        .flat_map(|x| (j.saturating_sub(1)..=(j + 1).min(cols - 1)).map(move |y| (x, y)))
        .filter_map(|(x, y)| (bytes[x][y] == b'*').then_some(1))
        .sum()
}
