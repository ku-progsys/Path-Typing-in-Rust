use syn::{File};

use crate::analysis::is_declassify;

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
    println!("-------------DEBUG-----------\n");
    //println!("{}", locals[0]);
    //println!("{}", locals[1]);
    dbg!(locals);
    //dbg!(exprs);
    //dbg!(&exprs);
    let mut path_to_declassify: Vec<item_abs::ExprAbstract> = vec![];
    let mut path_to_implicit_flow: Vec<item_abs::ExprAbstract> = vec![];
    for i in 0..exprs.len() {
        //dbg!(&exprs[i]);
        exprs[i].get_path_cond();
        println!("{}\n\n", exprs[i].clone());
        // checks for declassify function calls
        if exprs[i].clone().is_declassify() {
            for x in 0..i+1 {
                path_to_declassify.push(exprs[x].clone());
                println!("path_to_classify[{}]\n{}",x, path_to_declassify[x]);
            }
        }
    }
    //dbg!(path_to_declassify);
    // after this point, we should go through our expressions, check the expression type, 
    // if its a conditional, then we'll go through the condition, and check if a secret value is used 
    


    // after checking conditionals, then we need to check any then and else branches for the conditional

}
