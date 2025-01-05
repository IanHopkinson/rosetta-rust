//! A simple crate to count words in a file
//!
use std::fs;

/// The run function takes a file_path (&String) and returns a word count (usize)
pub fn run(file_path: &String) -> usize {
    let mut word_count = 0;

    let contents = fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("File read failed for {:?}", file_path));

    for line in contents.lines() {
        let words: Vec<&str> = line.split(char::is_whitespace).collect();
        word_count += words.len()
    }

    word_count
}

/// The add function is provided as an example for unit tests
///
/// Takes two `usize` arguments and adds them together, returning a `usize`
/// /// # Examples
///
/// ```
/// let result = wordcount_lib::add(3, 2);
/// assert_eq!(result, 5);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// the tests module contains a test for the add function
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
