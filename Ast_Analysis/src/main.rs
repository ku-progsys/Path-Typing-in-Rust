use syn::{File};

mod analysis;
mod item_abs;
fn main() {
    let file_path: String = "../../POC_simple/src/simple_1.rs".to_string();
    let ast: File = analysis::get_ast(file_path);
    let top_level_fn_item: syn::Item = analysis::get_fn_item(ast, "run_simple1".to_string())
                            .expect("Could not find top-level function for analysis");
    //dbg!(top_level_fn_item);
    let path_vec: Vec<syn::Stmt> = analysis::get_fn_stmts(top_level_fn_item);
    println!("{:?}", path_vec.len());
    let mut exprs: Vec<item_abs::ExprAbstract> = vec![];
    let mut locals: Vec<item_abs::LocalStmt> = vec![];
    for stmt in &path_vec {
        if let syn::Stmt::Local(local_stmt) = stmt {
            let local_stmt = item_abs::LocalStmt::new(local_stmt.clone());
            locals.push(local_stmt);
        }
        if let syn::Stmt::Expr(expr, _) = stmt {
            let abst_expr = item_abs::ExprAbstract::new(expr.clone());
            exprs.push(abst_expr);
        }
        
    }
    //dbg!(path_vec);
    dbg!(locals);
    println!("\n\n");
    dbg!(exprs);
}
