// Script that parses Markdown files into HTML.
use std::fs;
use std::io;
use pulldown_cmark::{html, Options, Parser};

fn get_input_from_user() -> String {
    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).expect("Failed to read line.");

    user_input.trim().to_string()
}

fn load_file(file_path: &str) -> String {
    let data = match fs::read_to_string(file_path) {
        Ok(data) => data,
        Err(e) => panic!("Unable to read file: {e:?}"),
    };
    data
}

fn main() {
    println!("Enter path to MD file to convert into HTML:");
    let file_path = get_input_from_user();
    let markdown_input = load_file(&file_path);
    println!("{}", markdown_input);

    let options = Options::empty();
    let parser = Parser::new_ext(&markdown_input, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    println!("{}", html_output);
}
