const MAP: [&str; 11] = [
    "Zero",
    "One",
    "Two",
    "Three",
    "Four",
    "Five",
    "Six",
    "Seven",
    "Eight",
    "Nine",
    "Ten",
];

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (0..take_down)
        .fold(String::new(), |mut acc, i| {
        match start_bottles - i {
            0 => {
                // Should never hit
                acc.push_str("No green bottles hanging on the wall.\n");
                acc.push_str("No green bottles hanging on the wall.\n");
                acc.push_str("And if one green bottle should accidentally fall,\n");
                acc.push_str("There'll be no green bottles hanging on the wall.\n");
            }
            1 => {
                // Special case, intro is singular, outro is plural
                acc.push_str("One green bottle hanging on the wall,\n");
                acc.push_str("One green bottle hanging on the wall,\n");
                acc.push_str("And if one green bottle should accidentally fall,\n");
                acc.push_str("There'll be no green bottles hanging on the wall.\n");
            }
            2 => {
                // Special case, intro is plural, outro is singular
                acc.push_str("Two green bottles hanging on the wall,\n");
                acc.push_str("Two green bottles hanging on the wall,\n");
                acc.push_str("And if one green bottle should accidentally fall,\n");
                acc.push_str("There'll be one green bottle hanging on the wall.\n");
            }
            _ => {
                // Both intro & outro are plural
                acc.push_str(&format!(
                    "{} green bottles hanging on the wall,\n",
                    MAP[start_bottles as usize - i as usize]
                ));
                acc.push_str(&format!(
                    "{} green bottles hanging on the wall,\n",
                    MAP[start_bottles as usize - i as usize]
                ));
                acc.push_str("And if one green bottle should accidentally fall,\n");
                acc.push_str(&format!(
                    "There'll be {} green bottles hanging on the wall.\n",
                    MAP[start_bottles as usize - i as usize - 1].to_ascii_lowercase()
                ));
            }
        }
        acc.push('\n');
        acc
        })
}
