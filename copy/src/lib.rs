pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exp = c as f64;
    let abs = exp.abs();
    (c, exp.exp(), abs.ln())
}

pub fn str_function(a: String) -> (String, String) {
    let input_parts: Vec<&str> = a.split_whitespace().collect();

    let exp_values: Vec<String> = input_parts
        .iter()
        .map(|&part| {
            let value: f64 = part.parse().unwrap();
            format!("{}", value.exp())
        })
        .collect();

    let exp_string = exp_values.join(" ");

    (a, exp_string)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut c = Vec::new();
    let mut d = Vec::new();
    for &x in b.iter() {
        c.push(x);
        if x > 0 {
            d.push((x as f64).ln());
        } else {
            d.push(0.0);
        }
    }
    (c, d)
}
