pub fn rev_str(input: &str) -> String {
    input.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rev_str_test() {
        let result = rev_str("Hello");
        assert_eq!(result, "olleH");
    }
}
