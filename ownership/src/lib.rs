pub fn first_subword(s: String) -> String {
    let mut chars = s.chars();
    let mut first_subword = String::new();

    while let Some(c) = chars.next() {
        if c == '_' || c.is_uppercase() && first_subword.chars().count() > 0 {
            break;
        }
        first_subword.push(c);
    }
    first_subword
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_subword_test() {
        let s = String::from("helloWorld");
        let s1 = String::from("HelloWorld");
        let s2 = String::from("hello_world");

        let result = first_subword(s);
        assert_eq!(result, "hello");
        assert_eq!(first_subword(s1), "Hello");
        assert_eq!(first_subword(s2), "hello");
    }
}
