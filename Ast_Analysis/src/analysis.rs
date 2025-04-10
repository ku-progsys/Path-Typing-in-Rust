use std::fs;
use std::io;
use syn::{parse_file, File};


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
            dbg!(&stmt);
            path_vec.push(stmt);
        }
    }

    path_vec
}