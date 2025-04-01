pub fn sum(a: i32, b: i32) -> i32 {
    if a < 0 || a > 255 || b < 0 || b > 255 {
        panic!("Overflow");
    }

    return a + b;
}

pub fn diff(a: i32, b: i32) -> i32 {
    if a < -32767 || b > 32767 {
        panic!("Overflow");
    }

    return a - b;
}

pub fn pro(a: i32, b: i32) -> i32 {
    if a < -128 || b > 127 {
        panic!("Overflow");
    }

    return a * b;
}

pub fn quo(a: f32, b: f32) -> f32 {
    a / b
}

pub fn rem(a: f32, b: f32) -> f32 {
    a % b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn sum_test() {
        let sum_result = sum(2, 2);
        assert_eq!(sum_result, 4);
    }

    #[test]
    fn diff_test() {
        let diff_result = diff(2,2);
        assert_eq!(diff_result, 0);
    }

    #[test]
    #[should_panic]
    fn pro_test() {
        let pro_result = pro(2,2);
        assert_eq!(pro_result, 0);
    }

    #[test]
    fn quo_test() {
        let quo_result = quo(2.0, 2.0);
        assert_eq!(quo_result, 1.0);
    }

    #[test]
    fn rem_test() {
        let rem_result = rem(2.0, 2.0);
        assert_eq!(rem_result, 0.0);
    }
}
