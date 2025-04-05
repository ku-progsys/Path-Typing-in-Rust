use std::fs;
use syn::{parse_file, File};

pub fn parse_ast(file_path: String) {
    let source = fs::read_to_string(file_path).expect("Failed to read file path");
    let ast: File = parse_file(&source).expect("Failed to parse source into ast");

    println!("{:#?}", ast);
}