pub fn talking(text: &str) -> &str {
    let trimmed = text.trim();

    match trimmed {
        "" => "Just say something!",
        _ if is_yelling_question(trimmed) => "Quiet, I am thinking!",
        _ if is_yelling(trimmed) => "There is no need to yell, calm down!",
        _ if !is_yelling(trimmed) && is_question(trimmed) => "Sure.",
        _ => "Interesting",
    }
}
