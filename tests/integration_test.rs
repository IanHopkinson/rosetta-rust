#[test]
fn test_wordcount() {
    let file_path = String::from("tests/fixtures/word_count_test_file.txt");
    assert_eq!(wordcount_lib::run(&file_path), 6);
}
