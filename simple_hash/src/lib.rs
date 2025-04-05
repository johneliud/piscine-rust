use std::collections::HashMap;

pub fn word_frequency_counter<'a>(words: &'a [&'a str]) -> HashMap<&'a str, usize> {
    let mut res = HashMap::new();
    for &word in words {
        *res.entry(word).or_insert(0) += 1;
    }
    res
}
