pub fn sum(a: u8, b: u8) -> u8 {
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
