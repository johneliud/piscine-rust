pub fn to_url(s: &str) -> String {
    s.replace(" ", "%20")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_url_test() {
        let result = to_url("Hello, world!");
        assert_eq!(result, "Hello,%20world!");
    }
}
