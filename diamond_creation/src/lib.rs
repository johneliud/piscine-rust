pub fn get_diamond(c: char) -> Vec<String> {
    let n = c as usize - 'A' as usize;
    let width = 2 * n + 1;
    let mut diamond = Vec::new();

    for i in 0..=n {
        diamond.push(build_line(i, width));
    }

    for i in (0..n).rev() {
        diamond.push(build_line(i, width));
    }

    diamond
}
