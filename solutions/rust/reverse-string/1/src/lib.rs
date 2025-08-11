pub fn reverse(input: &str) -> String {
    let mut reversed: String = String::with_capacity(input.len());

    for (_i, c) in input.chars().enumerate() {
        reversed.insert(0, c);
    }

    return reversed;
}
