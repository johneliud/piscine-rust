pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exp = c as f64;
    let abs = exp.abs();
    (c, exp.exp(), abs.ln())
}
