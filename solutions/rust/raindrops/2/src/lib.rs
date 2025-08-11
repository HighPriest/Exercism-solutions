pub fn raindrops(n: u32) -> String {
    let result = ["Pling", "Plang", "Plong", ""]
        .iter()
        .zip([3, 5, 7].iter())
        .filter_map(|(s, &d)| {
            if n % d == 0 {
                Some(s.to_string())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("");

    if result.is_empty() {
        return n.to_string();
    }
    result
}
