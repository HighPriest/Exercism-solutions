#![deny(clippy::all)]
#![warn(clippy::pedantic)]

#[must_use]
pub fn name_to_id(name: &str) -> Option<usize> {
    // Static hash maps don't exist in Rust...
    match name {
        "Alice" => Some(0),
        "Bob" => Some(1),
        "Charlie" => Some(2),
        "David" => Some(3),
        "Eve" => Some(4),
        "Fred" => Some(5),
        "Ginny" => Some(6),
        "Harriet" => Some(7),
        "Ileana" => Some(8),
        "Joseph" => Some(9),
        "Kincaid" => Some(10),
        "Larry" => Some(11),
        _ => None, // Handle cases where the name isn't found
    }
}

#[must_use]
fn plant_name(c: char) -> &'static str {
    match c {
        'V' => "violets",
        'R' => "radishes",
        'C' => "clover",
        'G' => "grass",
        _ => "unknown",
    }
}

#[must_use]
pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let child_index = match name_to_id(student) {
        Some(id) => id * 2,
        None => 0
    };

    diagram
        .lines()
        .flat_map(|row| {
            row[child_index..child_index+2].chars().map(plant_name)
        })
        .collect()
}
