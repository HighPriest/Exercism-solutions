pub fn reply(message: &str) -> &str {
    if message.trim().is_empty() {
        return "Fine. Be that way!"
    }
    if message.chars().any(|ch| ch.is_alphabetic()) && message == message.to_uppercase() {
        match message.trim().chars().last() {
            Some('?') => {return "Calm down, I know what I'm doing!"}
            _ => { return "Whoa, chill out!" },
        }
    } 
    if message.trim().chars().last() == Some('?') {
        return "Sure." 
    }

    return "Whatever.";
    
    todo!("have Bob reply to the incoming message: {message}")
}
