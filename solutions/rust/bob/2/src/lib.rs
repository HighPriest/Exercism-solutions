pub fn reply(message: &str) -> &str {
    let message = message.trim();
    if message.is_empty() {
        return "Fine. Be that way!"
    }
    // (question, yelling)
    match (message.ends_with('?'), message.chars().any(|ch| ch.is_alphabetic()) && message == message.to_uppercase()) {
        (true, true) => { return "Calm down, I know what I'm doing!" },
        (_, true) => { return "Whoa, chill out!"}
        (true, _) => { return "Sure."}
        (_,_) => {return "Whatever."}
    }
}
