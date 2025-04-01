pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    return (f - 32.0) * 5.0/9.0;
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    return (c * 9.0/5.0) + 32.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fahrenheit_to_celsius_test() {
        let fahrenheit_temp = fahrenheit_to_celsius(32.0);
        assert_eq!(fahrenheit_temp, 0.0);
    }

    #[test]
    fn celsius_to_fahrenheit_test() {
        let celsius_temp = celsius_to_fahrenheit(32.0);
        assert_eq!(celsius_temp, 89.6);
    }
}
