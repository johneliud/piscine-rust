pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

pub fn diff(a: i32, b: i32) -> i32 {
    a - b
}

pub fn pro(a: i32, b: i32) -> i32 {
    a * b
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
    fn it_works() {
        let sum_result = sum(2, 2);
        assert_eq!(sum_result, 4);

        let diff_result = diff(2,2);
        assert_eq!(diff_result, 0);

        let pro_result = pro(2, 2);
        assert_eq!(pro_result, 4);

        let quo_result = quo(2.0, 2.0);
        assert_eq!(quo_result, 1.0);

        let rem_result = rem(2.0, 2.0);
        assert_eq!(rem_result, 0.0);
    }
}
