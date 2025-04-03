pub fn str_len(s: &str) -> usize {
    s.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_len_test() {
        let result = str_len("Hello");
        assert_eq!(result, 5);
    }
}
