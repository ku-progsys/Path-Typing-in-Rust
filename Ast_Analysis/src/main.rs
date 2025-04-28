use syn::{File};

mod analysis;
fn main() {
    let file_path: String = "../../POC_simple/src/simple_1.rs".to_string();
    let ast: File = analysis::get_ast(file_path);
    let top_level_fn_item: syn::Item = analysis::get_fn_item(ast)
                            .expect("Could not find top-level function for analysis");
    //dbg!(top_level_fn_item);
    let path_vec: Vec<syn::Stmt> = analysis::get_fn_stmts(top_level_fn_item);
    println!("{:?}", path_vec.len());
    for stmt in &path_vec {
        if let Some(trace) = analysis::gather_path(stmt) {
            println!("Trace to declassify: ");
            for expr in trace.iter().rev() {
                println!("{:?}\n\n", expr);
            } 
        }
    }
}
