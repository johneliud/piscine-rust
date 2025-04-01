pub fn km_per_hour_to_meters_per_second(km_h: f64) -> f64 {
    return km_h / 3.6;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn km_per_hour_to_meters_per_second_test() {
        let result = km_per_hour_to_meters_per_second(100.0);
        assert_eq!(result, 27.77777777777778);
    }
}
