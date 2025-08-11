pub fn raindrops(n: u32) -> String {
    let mut result = String::new();
    if (n % 3).eq(&0) {
        result.push_str("Pling");
    } 
    if (n % 5).eq(&0) {
        result.push_str("Plang");
    }
    if (n % 7).eq(&0) {
        result.push_str("Plong");
    }

    if result.is_empty() {
        return n.to_string()
    }

    result
}
