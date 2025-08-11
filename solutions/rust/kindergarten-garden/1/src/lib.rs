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
    let mut result: Vec<&'static str> = vec![];

    diagram.split_whitespace().for_each(|row| {
        let plants: (&str, &str) = match row
            .chars()
            .enumerate()
            .collect::<Vec<_>>()
            .windows(2)
            .step_by(2)
            .nth(
                name_to_id(student).unwrap_or(0) // Get order of the child
            )
            .map(|pair| (pair[0].1, pair[1].1)) {
                Some((plant1,plant2)) => (plant_name(plant1),plant_name(plant2)),
                None => ("unknown", "unknown")
            };

        result.push(plants.0);
        result.push(plants.1);
    });
    result
}
