// Script to count number of lines, words, and characters in a text file.
use std::io;
use std::fs;

fn get_string_input() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to parse line.");

    input.trim().to_string()
}

fn read_file(path: String) -> String {
    fs::read_to_string(path).expect("Could not open file.")
}

fn main() {
    println!("Enter a file path: ");
    let path = get_string_input();
    
    println!("Opening {}", path);
    let file_contents = read_file(path);

    let line_count = file_contents.lines().count();
    let word_count = file_contents.split_whitespace().count();
    let char_count = file_contents.chars().count();
    println!("Number of lines: {}", line_count);
    println!("Number of words: {}", word_count);
    println!("Number of chars: {}", char_count);
}
