pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exp = c as f64;
    let abs = exp.abs();
    (c, exp.exp(), abs.ln())
}

pub fn str_function(a: String) -> Result<(String, String), String> {
    match a.parse::<f64>() {
        Ok(num) => Ok((a.clone(), num.exp().to_string())),
        Err(_) => Err(format!("Failed to parse '{}' as a float", a)),
    }
}
pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let res = b
        .iter()
        .map(|&r| {
            let value = (r.abs() as f64).ln();
            value
        })
        .collect();
    (b, res)
}
