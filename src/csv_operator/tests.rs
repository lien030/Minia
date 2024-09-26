use std::fs;
use super::*;

#[test]
fn test_csv_operator() {
    let csv = CsvOperator::new(true).unwrap();
    let file_name = csv.file_name.clone();
    csv.write_vec(vec!["aaa", "bbb", "ccc"]).unwrap();
    csv.write_vec(vec!["ddd", "eee", "fff"]).unwrap();
    // print file content and delete it
    println!("{}", fs::read_to_string(&file_name).unwrap());
    fs::remove_file(file_name).unwrap();
}