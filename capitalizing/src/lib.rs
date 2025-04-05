pub fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    let first_char = chars.next().unwrap().to_uppercase().to_string();
    first_char + chars.as_str()
}

pub fn title_case(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
