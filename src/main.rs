use std::env;
use wordcount;

fn main() {
    // let args: Vec<String> = env::args().collect();

    // let file_path = &args[1];
    let file_path = String::from("word_count_test_file.txt");
    println!("Counting words in {file_path}");

    let n_words: usize = wordcount::run(&file_path);

    println!("Found wordcount: {n_words}");
}
