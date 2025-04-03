pub fn is_empty(v: &str) -> bool {
    v.chars().count() == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_empty_test() {
        let result = is_empty("");
        assert_eq!(result, true);
    }
}
