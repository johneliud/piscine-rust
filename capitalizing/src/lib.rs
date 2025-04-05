pub fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    let first_char = chars.next().unwrap().to_uppercase().to_string();
    first_char + chars.as_str()
}
