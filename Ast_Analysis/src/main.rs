use syn::{File};

mod analysis;
fn main() {
    let file_path: String = "../../POC_simple/src/simple_2.rs".to_string();
    let ast: File = analysis::get_ast(file_path);
    println!("{:#?}", ast);
}
