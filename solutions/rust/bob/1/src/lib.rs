pub fn reply(message: &str) -> &str {
    let message = message.trim();
    if message.is_empty() {
        return "Fine. Be that way!";
    }

    let is_question = message.ends_with('?');

    let has_alphabetic = message.chars().any(|c| c.is_alphabetic());
    let is_yelling = has_alphabetic && message.to_uppercase() == message;

    match (is_yelling, is_question) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Whoa, chill out!",
        (false, true) => "Sure.",
        _ => "Whatever.",
    }
}