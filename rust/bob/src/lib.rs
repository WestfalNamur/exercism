fn is_yelling(message: &str) -> bool {
    // - Check if message has alphabetic letters by filtering them, count them and check if > 0.
    // - Return message to upper case if there is one and it has letters.
    let have_letters: bool = message.chars().filter(|x| x.is_alphabetic()).count() > 0;
    message.to_uppercase() == message && have_letters
}

pub fn reply(message: &str) -> &str {
    // Bob is a lackadaisical teenager. In conversation, his responses are very limited.
    // Bob answers 'Sure.' if you ask him a question, such as "How are you?".
    // He answers 'Whoa, chill out!' if you YELL AT HIM (in all capitals).
    // He answers 'Calm down, I know what I'm doing!' if you yell a question at him.
    // He says 'Fine. Be that way!' if you address him without actually saying anything.
    // He answers 'Whatever.' to anything else.
    // Use .trim() trait to remove whitespaces.
    match message.trim() {
        // Nothing (no letters)
        msg if msg.trim().len() == 0 => "Fine. Be that way!",
        // If question (ends with "?") and yelling
        msg if msg.ends_with("?") && is_yelling(msg) => "Calm down, I know what I'm doing!",
        // If question (ends with "?")
        msg if msg.ends_with("?") => "Sure.",
        // Only Yelling
        msg if is_yelling(msg) => "Whoa, chill out!",
        // Else
        _ => "Whatever.",
    }
}
