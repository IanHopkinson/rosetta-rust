use std::fs;

pub fn run(file_path: &String) -> usize {
    let mut word_count = 0;
    for line in fs::read_to_string(file_path).unwrap().lines() {
        let words: Vec<&str> = line.split(char::is_whitespace).collect();
        word_count += words.len()
    }

    word_count
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
