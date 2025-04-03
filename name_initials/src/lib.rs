pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut initials = Vec::new();

    for name in names {
        let split = name.split_whitespace();
        let mut initial = String::new();

        for word in split {
            initial.push_str(&word[..1].to_uppercase());
        }
        initials.push(initial);
    }
    initials
}
