pub fn is_empty(v: &str) -> bool {
    v.chars().count() == 0
}

pub fn is_ascii(v: &str) -> bool {
    v.chars().all(|c| c.is_ascii())
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
