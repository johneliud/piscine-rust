pub fn factorial(mut num: u64) -> u64 {
    let mut result: u64 = 1;
    
    for i in 1..=num {
        result *= i;
        num+=1;
    }
    return  result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_test() {
        let result = factorial(6);
        assert_eq!(result, 720);
    }
}
