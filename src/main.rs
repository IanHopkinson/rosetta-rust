//! A binary crate to do a word count for a file
use std::env;
use wordcount_lib;

/// A main function to take a file_path as a commandline argument for a word count
pub fn main() {
    // let args: Vec<String> = env::args().collect();

    // let file_path = &args[1];
    let file_path = String::from("word_count_test_file.txt");
    println!("Counting words in {file_path}");

    let n_words: usize = wordcount_lib::run(&file_path);

    println!("Found wordcount: {n_words}");
}
