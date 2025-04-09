use std::fs::File;

pub fn open_file(s: &str) -> File {
    File::open(s).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_file() {
        let result = open_file("not_a_file.txt");
        assert!(result.is_err());
    }
}
