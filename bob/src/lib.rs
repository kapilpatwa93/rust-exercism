pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let contains_char = message.chars().any(|c| c.is_alphabetic());
    let is_question = message.ends_with('?');
    let upper_string = message.to_string().to_uppercase();
    let is_uppercase = upper_string.eq(&String::from(message));
    let is_empty = message.is_empty();
    match message {
        _ if is_empty => "Fine. Be that way!",
        _ if is_question && is_uppercase && contains_char => "Calm down, I know what I'm doing!",
        _ if is_question => "Sure.",
        _ if is_uppercase && !is_empty && contains_char => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
