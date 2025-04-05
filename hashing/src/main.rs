pub fn mean(list: &[i32]) -> f64 {
    list.iter().sum::<i32>() as f64 / list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mut l = list.to_vec();
    l.sort();
    let mid = l.len() / 2;
    if l.len() % 2 == 0 {
        (l[mid - 1] + l[mid]) / 2
    } else {
        l[mid]
    }
}
