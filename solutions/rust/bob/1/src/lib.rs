pub fn reply(message: &str) -> &str {
    let message = message.trim();

    if message.is_empty() {
        return "Fine. Be that way!";
    }

    let has_qm = message.ends_with('?');

    let is_yelling = message.chars().any(|c| c.is_alphabetic())
        && message == message.to_uppercase();

    if is_yelling && has_qm {
        return "Calm down, I know what I'm doing!";
    } else if is_yelling {
        return "Whoa, chill out!";
    } else if has_qm {
        return "Sure.";
    } else {
        return "Whatever.";
    }
}