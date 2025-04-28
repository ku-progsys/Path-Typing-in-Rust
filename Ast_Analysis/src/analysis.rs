use std::fs;
use std::io;
use syn::Expr;
use syn::{parse_file, File};
use syn::visit::{self, Visit};

pub fn get_ast(file_path: String) -> File {
    let source = fs::read_to_string(file_path).expect("Failed to read file path");
    let ast: File = parse_file(&source).expect("Failed to parse source into ast");
    ast
}

// call this for top-level function you would like to analyze 
pub fn get_fn_item(ast: File) -> Option<syn::Item> {
    /*-----------------------------------------
        Steps: 
            iterate through the ast 
                extract the function which should be analyzed's node (not necessarily a main function)  
    -----------------------------------------*/
    println!("Enter Function to be Analyzed: ");
    let mut function_to_analyze: String = String::new(); // enter top level fn to analyze
    io::stdin().read_line(&mut function_to_analyze).expect("Failed to read line");

    for item in ast.items {
        if let syn::Item::Fn(func) = item {
            if func.sig.ident.to_string() == function_to_analyze.trim() {
               return Some(syn::Item::Fn(func));
            }
        }
    }
    // if we exit the loop w/o return an ItemFn we can return None
    None
}

pub fn get_fn_stmts(fn_item: syn::Item) -> Vec<syn::Stmt> {
    /*-----------------------------------------
        step through the blocks w/in the top function,
            when you encounter: fn, or stmt (conditional)
                add that block to the returned vector
    -----------------------------------------*/
    let mut path_vec: Vec<syn::Stmt> = Vec::<syn::Stmt>::new();
    if let syn::Item::Fn(func) = fn_item {
        let block = func.block;
        //dbg!(&block);
        for stmt in block.stmts {
            //dbg!(&stmt);
            path_vec.push(stmt);
        }
    }

    path_vec
}

pub fn gather_path<'ast>(node: &'ast syn::Stmt) -> Option<Vec<syn::Expr>> {
    let mut prog_trace = Vec::new();
    if gather_prog_trace(node, &mut prog_trace) {
        return Some(prog_trace);
    } 
    None
}

pub fn gather_prog_trace<'ast>(node: &'ast syn::Stmt, trace: &mut Vec<Expr>) -> bool {

    struct DeclassifyVisitor<'a> {
        trace: &'a mut Vec<Expr>,
        found: bool,
    }

    impl<'ast> Visit<'ast> for DeclassifyVisitor<'_> {
        fn visit_expr(&mut self, expr: &'ast Expr) {
            if let Expr::Call(call) = expr {
                if let Expr::Path(ref path_expr) = *call.func {
                    if let Some(seg) = path_expr.path.segments.first() {
                        if seg.ident == "declassify" {
                            self.found = true;
                            self.trace.push(expr.clone());
                            return;
                        }
                    }
                }
            }
            self.trace.push(expr.clone());

            visit::visit_expr(self, expr);
            if self.found {
                self.trace.push(expr.clone());
            }
        }
        fn visit_stmt(&mut self, stmt: &syn::Stmt) {
            visit::visit_stmt(self, stmt);
        }
    }
    let mut visitor = DeclassifyVisitor {
        trace,
        found: false,
    };
    visitor.visit_stmt(node);
    visitor.found
}

pub fn is_declassify(fn_stmt: syn::Stmt) -> bool {
    if let syn::Stmt::Expr(syn::Expr::Call(expr_call), _) = fn_stmt {
        if let syn::Expr::Path(expr_path) = *expr_call.func {
            if let Some(path_segment) = expr_path.path.segments.first() {
                return path_segment.ident == "declassify";
            }
        }
    }
    false
}
