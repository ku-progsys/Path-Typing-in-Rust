mod analysis;
fn main() {
    let file_path: String = "../../POC_simple/src/simple_1.rs".to_string();
    analysis::parse_ast(file_path);
}
