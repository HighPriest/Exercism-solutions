#[must_use]
pub fn brackets_are_balanced(string: &str) -> bool {
    string.chars().try_fold(Vec::new(), |mut stack, ch| {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' => if stack.pop() != Some('(') { return None; },
            ']' => if stack.pop() != Some('[') { return None; },
            '}' => if stack.pop() != Some('{') { return None; },
            _ => ()
        }
        Some(stack)
    }).unwrap_or(vec![' ']).is_empty()
}