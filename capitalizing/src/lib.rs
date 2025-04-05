pub fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    let first_char = chars.next().unwrap().to_uppercase().to_string();
    first_char + chars.as_str()
}

pub fn title_case(input: &str) -> String {
    let mut result = String::new();
    let mut new_word = true;
    for c in input.chars() {
        if c.is_whitespace() {
            result.push(c);
            new_word = true;
        } else if new_word {
            result.push(c.to_ascii_uppercase());
            new_word = false;
        } else {
            result.push(c.to_ascii_lowercase());
        }
    }
    result
}

pub fn change_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_uppercase() {
                c.to_ascii_lowercase()
            } else {
                c.to_ascii_uppercase()
            }
        })
        .collect()
}
