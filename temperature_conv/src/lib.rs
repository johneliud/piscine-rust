pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    return (f - f) * 5.0/9.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fahrenheit_to_celsius_test() {
        let fahrenheitTemp = fahrenheit_to_celsius(32.0);
        assert_eq!(fahrenheitTemp, 0.0);
    }
}
