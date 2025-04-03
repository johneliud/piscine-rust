pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut initials = Vec::new();

    for name in names {
        let split = name.split_whitespace();
        let mut initial = String::new();

        for word in split {
            initial.push_str(&format!("{}. ", &word[..1].to_uppercase()));
        }
        initials.push(initial.trim_end().to_string());
    }
    initials
}
