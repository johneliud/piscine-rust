pub fn delete_and_backspace(s: &mut String) {
    let chars: Vec<char> = s.chars().collect();
    let mut result = String::new();
    let mut index = 0;
    while index < chars.len() {
        match chars[index] {
            '-' => {
                result.pop();
                index += 1;
            }
            '+' => {
                let mut count = 0;
                while index < chars.len() && chars[index] == '+' {
                    count += 1;
                    index += 1;
                }
                index += count;
            }
            c => {
                result.push(c);
                index += 1;
            }
        }
    }
    *s = result;
}
