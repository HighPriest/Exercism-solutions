#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]


pub fn verse(n: u32) -> String {
    //todo!("emit verse {n}")
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        2 => "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string(),
        _ => format!(
            "{n} bottles of beer on the wall, {n} bottles of beer.\nTake one down and pass it around, {m} bottles of beer on the wall.\n", m = n-1
        ).to_string()
    }
    
}

pub fn sing(start: u32, end: u32) -> String {
    //todo!("sing verses {start} to {end}, inclusive")
    (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<_>>()
        .join("\n")
 }
