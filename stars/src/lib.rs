pub fn stars(n: u32) -> String {
    let s = 2u32.pow(n);
    "*".repeat(s as usize)
}
