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

pub fn do_operations(v: &mut [String]) {
    for s in v.iter_mut() {
        if let Some(op_pos) = s.find(|c: char| c == '+' || c == '-') {
            let (left, right) = s.split_at(op_pos);
            let left_num = left.trim().parse::<i32>().unwrap_or(0);
            let right_num = right[1..].trim().parse::<i32>().unwrap_or(0);

            *s = match s.chars().nth(op_pos) {
                Some('+') => (left_num + right_num).to_string(),
                Some('-') => (left_num - right_num).to_string(),
                _ => s.clone(),
            };
        }
    }
}
