pub fn sum(a: u32, b: u32) -> u32 {
    return a + b;
}

pub fn diff(a: i32, b: i32) -> i32 {
    return a - b;
}

pub fn pro(a: i32, b: i32) -> i32 {
    return a * b;
}

pub fn quo(a: f32, b: f32) -> f32 {
    if b == 0.0 {
        panic!("Cannot be 0.0")
    }

    a / b
}

pub fn rem(a: f32, b: f32) -> f32 {
    if b == 0.0 {
        panic!("Cannot be 0.0")
    }
    
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
