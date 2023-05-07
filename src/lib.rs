pub mod dynamic_checking;

use std::io::{Read};
use crate::dynamic_checking::check_goldbach;

pub fn run() {
    let file_path = std::path::Path::new("last_number.txt");
    let mut last_number: usize = 4;
    if file_path.exists() {
        // Read and load the last number from the file
        let mut file = std::fs::File::open(file_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        last_number = contents.parse().unwrap();
        println!("Last number: {}", last_number)
    }
    println!("Starting to check the Goldbach conjecture for even integers greater than {}", last_number);
    check_goldbach(last_number);
}