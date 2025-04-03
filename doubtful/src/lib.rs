pub fn doubtful(s: &mut String) {
    let modified_s = s.push('?');

    println!("{:?}", modified_s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubtful_test() {
        let mut s = "Hello".to_string();
        doubtful(&mut s);
        assert_eq!(s, "Hello?".to_string());
    }
}
