use syn::{File};

mod analysis;
fn main() {
    let file_path: String = "../../POC_simple/src/simple_2.rs".to_string();
    let ast: File = analysis::get_ast(file_path);
    let top_level_fn_item: syn::Item = analysis::get_fn_item(ast)
                            .expect("Could not find top-level function for analysis");
    //dbg!(top_level_fn_item);
    let path_vec: Vec<syn::Stmt> = analysis::get_fn_stmts(top_level_fn_item);
    dbg!(&path_vec);
}
