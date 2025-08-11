#![deny(clippy::all)]
#![warn(clippy::pedantic)]

use std::cmp::Ordering;

#[must_use]
pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut left = 0usize;
    let mut right = array.len();
    let mut mid: usize;
    while left < right {
        mid = left + ((right - left) / 2);
        match array[mid].cmp(&key) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => {
                left = mid + 1;
            }
            Ordering::Greater => {
                right = mid;
            }
        }
    }
    None
}
