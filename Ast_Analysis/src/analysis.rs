use std::fs;
use syn::{parse_file, File};

pub fn get_ast(file_path: String) -> File {
    let source = fs::read_to_string(file_path).expect("Failed to read file path");
    let ast: File = parse_file(&source).expect("Failed to parse source into ast");
    ast
}

pub fn gen_path(ast: File) {
    
}