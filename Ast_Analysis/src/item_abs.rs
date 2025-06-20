use syn::Expr;
use std::fmt;
use syn::Token;

/*--------------------------------
    there are many more expression types that will need to be added in the future
--------------------------------*/
#[derive(Debug)]
enum LocalType {
    LOCAL,
    NotTracked,
}
#[derive(Debug, Clone)]
enum Datatype {
    INT32,
    STRING,
    NotTracked,
}


#[derive(Debug)]
pub struct LocalStmt {
    raw_stmt: syn::Stmt,
    stmt_type: LocalType,
    fmt_stmt: Option<String>,
    id: Option<String>,
    value_type: Option<Datatype>, 
    taint: Option<bool>,
}
impl fmt::Display for LocalStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"Stmt{{
            id: {:?}
            stmt_type: {:?}
            value_type: {:?}
            taint: {}
            }}"#, self.id.clone().unwrap(),
            self.stmt_type, 
            self.value_type.clone().unwrap(),
            self.taint.clone().unwrap())
        }
    }
    impl LocalStmt {
        pub fn new(process_stmt: syn::Local) -> Self {
            match process_stmt {
                syn::Local{..} => {
                    // println!("stmt is Local data");
                    let type_of_stmt = LocalType::LOCAL;
                    let formatted_stmt = format!("{:?}", process_stmt);
                    let local_id = Self::id_local(syn::Stmt::Local(process_stmt.clone()));
                    let value_type = Self::get_val_type(syn::Stmt::Local(process_stmt.clone()));
                    let assigned_taint = Self::get_taint(&local_id);
                    Self {
                        raw_stmt: syn::Stmt::Local(process_stmt),
                        stmt_type: type_of_stmt,
                        fmt_stmt: Some(formatted_stmt),
                        id: Some(local_id),
                        value_type: Some(value_type),
                        taint: Some(assigned_taint),
                    }
                }
            }
        }
        
        fn get_taint(loc_id: &str) -> bool {
            if loc_id == "secret" {
                return true;
            }
            false
        }
        
        fn id_local(raw_stmt: syn::Stmt) -> String {
            if let syn::Stmt::Local(stmt) = raw_stmt {
                let pattern = stmt.pat;
                if let syn::Pat::Ident(id) = pattern {
                    return format!("{:?}", id).to_string();
                }
                else if let syn::Pat::Type(pat_type) = pattern {
                    if let syn::Pat::Ident(id) = *pat_type.pat {
                        let local_id = id.ident;
                        let local_id = format!("{:?}", local_id);
                        //println!("local_id: {:?}", local_id);
                        return Self::parse_ident(&local_id)
                        .expect("couldn't parse id from ident");
                    }
                }
                else {
                    return "syn::Pat::Ident not matched".to_string();
                }
            }
            "syn::Stmt::Local not matched".to_string()
        }
        pub fn parse_ident(ident: &str) -> Option<String> {
            let mut ret_str = String::new();
            let mut raw_str = ident.chars();
            while let Some(ref mut ch) = raw_str.next() {
                if *ch == '(' {
                    while let Some(ref mut id_chars) = raw_str.next() {
                        if *id_chars == ')' {
                            break;
                        }
                        ret_str.push(*id_chars);
                        continue;
                    }
                }
            }
            Some(ret_str)
        }
        
        fn get_val_type(raw_stmt: syn::Stmt) -> Datatype {
            if let syn::Stmt::Local(stmt) = raw_stmt {
                if let syn::Pat::Type(loc_type) = stmt.pat {
                    if let syn::Type::Path(loc_path) = *loc_type.ty {
                        let loc_path_str = format!("{:?}", loc_path.path.segments[0].ident);
                        let loc_path_ident = Self::parse_ident(&loc_path_str).expect("couldn't parse ident");
                        if loc_path_ident == "String" {
                            return Datatype::STRING;
                        }
                        else if loc_path_ident == "i32" {
                            return Datatype::INT32;
                        }
                    }
                }
                else {
                    println!("Type not matched with stmt.pat");
                }
            } else {
                println!("local type not matched with raw_stmt");
            }
            Datatype::NotTracked
        }
    }
/*--------------------------------
this will be a custom enum for easier information extraction from syn::Exprs 
--------------------------------*/
#[derive(Debug, Clone)]
enum ExprType {
    CALL,
    IF,
    WHILE,
    NotTracked,
}

#[derive(Debug, Clone)]
pub struct ExprAbstract {
    raw_expr: syn::Expr,
    expr_type: ExprType, 
    fmt_expr: Option<String>,
    path_cond: Option<PathCond>,
    fn_called: Option<String>,
    then_branch: Option<syn::Block>,
    else_branch: Option<Option<(Token![else], Box<Expr>)>>
}
impl fmt::Display for ExprAbstract {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"Expr{{
        expr_type: {:?}
        path_cond: {}
        then_branch: {:?}
        else_branch: {:?}
    }}"#, self.expr_type,
          self.path_cond.clone().unwrap(),
          self.then_branch.clone().ok_or("None"),
          self.else_branch.clone().ok_or("None"))
    }
}




impl ExprAbstract {
    pub fn new(process_expr: syn::Expr) -> Self {
        // match on expr type, set type, call fns to process specific expr types
        match process_expr {
            syn::Expr::Call{..} => {
                // println!("expression is call");
                let formatted_expr = format!("{:?}", process_expr);
                let expr_type = ExprType::CALL;
                Self {
                    raw_expr: process_expr,
                    expr_type: expr_type,
                    fmt_expr: Some(formatted_expr),
                    path_cond: None,
                    fn_called: None,     
                    then_branch: None,           
                    else_branch: None,           
                }
            }
            syn::Expr::If {..} => {
                // println!("expression is if");
                let formatted_expr = format!("{:?}", process_expr);
                let expr_type = ExprType::IF;
                Self {
                    raw_expr: process_expr,
                    expr_type: expr_type,
                    fmt_expr: Some(formatted_expr),
                    path_cond: Some(PathCond::new()),
                    fn_called: None,
                    then_branch: None,           
                    else_branch: None,       
                                
                }
            }
            syn::Expr::While {..} => {
                println!("expression is while");
                let formatted_expr = format!("{:?}", process_expr);
                let expr_type = ExprType::WHILE;
                Self {
                    raw_expr: process_expr,
                    expr_type: expr_type,
                    fmt_expr: Some(formatted_expr),
                    path_cond: Some(PathCond::new()),
                    fn_called: None,
                    then_branch: None,           
                    else_branch: None,                   
                }
            }
            _ => {
                println!("not in ExprType");
                let formatted_expr = format!("{:?}", process_expr);
                let expr_type = ExprType::NotTracked;
                Self {
                    raw_expr: process_expr,
                    expr_type: expr_type,
                    fmt_expr: None,
                    path_cond: None,
                    fn_called: None,
                    then_branch: None,           
                    else_branch: None,   
                }
            }
        }
    }
    
    pub fn get_path_cond(&mut self) {
        let expr = self.raw_expr.clone();
        // if expr == None {
        //     self.path_cond = None;
        //     return;
        // }
        let mut path_condition: PathCond = PathCond::new();
        /*-------------------
            here i will need to make sure process_cond is only called on conditional expressions
        -------------------*/
        path_condition = Self::process_cond(self.raw_expr.clone(), &mut path_condition);
        //dbg!(&path_condition);
        self.path_cond = Some(path_condition);
        if let syn::Expr::If(if_expr) = self.raw_expr.clone() {
            self.then_branch = Some(if_expr.then_branch);
            self.else_branch = Some(if_expr.else_branch);
        }
    }

    /*----------------------------
    process_if_cond will extract condition and operation type from 
        - nested path conds need to be handled elsewhere (I believe)
    ----------------------------*/
    fn process_cond(expr: syn::Expr, path_condition: &mut PathCond) -> PathCond { 
        let mut path_cond = path_condition.clone();
        if let syn::Expr::If(if_expr) = expr.clone() {
            //println!("Expr::If");
            // dbg!(&if_expr);
            let next_expr = if_expr.clone();
            path_cond.raw_expr = Some(syn::Expr::If(if_expr.clone()));
            // cond can be either Path or Binary 
            if let syn::Expr::Binary(bin_expr) = *next_expr.cond {
                path_cond.left = Some(format!("{:?}", bin_expr.left));
                path_cond.right = Some(format!("{:?}", bin_expr.right));
                //dbg!(&bin_expr);
                // get left 
                /*------------------
                now, we need to get nested path conditions 
                    ex: data == 23 && (flag == false) || false
                    both left and right Exprs can have nested Exprs
                    this is the time when I may need to write a new function to handle nested path conds
                this will involve calling a fn process_bin_op(cur_bin_expr, nested_path_cond: PathCond) -> PathCond
                    - call this function while there is a left or right branch to be processed 
                    - change type of nested to be a tuple of PathConds (left and right)
                    - process_bin_op will recursively call itself while there is a left/right binary expr to process
                    - should clean up code of process_cond after initial match w/ If Expr
                much of the following code this will be cleaned up by this process
                ------------------*/
                let mut left_nested_path_cond = PathCond::new();
                let mut right_nested_path_cond = PathCond::new();
                if let syn::Expr::Path(left_expr) = *bin_expr.left {
                    Self::process_bin_op(syn::Expr::Path(left_expr.clone()), &mut left_nested_path_cond);
                    //dbg!(&left_expr);
                    // check Path {PathSegment} for Ident or literal
                    let cond = left_expr.path;
                    //dbg!(&cond);
                    
                    // if PathSegment, then we have a variable in path condition
                    let syn::PathSegment{ident: path_id, ..} = &cond.segments[0];
                    //dbg!(&path_id);
                    let left_cond_id = format!("{:?}", path_id);
                    let left_cond_id = LocalStmt::parse_ident(&left_cond_id).expect("couldn't parse ident");
                    path_cond.left = Some(left_cond_id);
                    
                    // if Lit, then we have a literal value in path condition 
                    if let syn::Expr::Lit(right_expr) = *bin_expr.right {
                        let cond = right_expr.lit;
                        let cond = format!("{:?}", cond);
                        path_cond.right = Some(cond);
                    }
                } else if let syn::Expr::Binary(left_expr) = *bin_expr.left {

                    Self::process_bin_op(syn::Expr::Binary(left_expr.clone()), &mut right_nested_path_cond);
                }
                /*------------------
                here we need to get the op of the expression 
                ------------------*/
                let bin_op = bin_expr.op;
                path_cond.nested = Some(Box::new((left_nested_path_cond, right_nested_path_cond)));
                path_cond.op = format!("{:?}", bin_op).into();
                //dbg!(&bin_op);
            } 
            // handle unary if condition
            else if let syn::Expr::Path(unary) = *next_expr.cond {
                //dbg!(&unary);
                let cond = "UNARY".to_string();
                let syn::Path {leading_colon: _, segments, ..} = unary.path;
                let syn::PathSegment{ident: path_id, ..} = &segments[0];
                let cond_id = format!("{:?}", path_id);
                let cond_id = LocalStmt::parse_ident(&cond_id).expect("couldn't parse ident");
                path_cond.id = Some(cond_id);
                path_cond.op = Some(cond);
                path_cond.left = Some("None".to_string());
                path_cond.right = Some("None".to_string());
                

            }

        }
        if let syn::Expr::While(while_expr) = expr.clone() {
            // println!("Expr::While");
            path_cond.raw_expr = Some(syn::Expr::While(while_expr));
        }
        if let syn::Expr::Call(call_expr) = expr.clone() {
            // println!("Expr::Call");
            //dbg!(&call_expr);
            path_cond.raw_expr = Some(syn::Expr::Call(call_expr.clone()));
            if let syn::Expr::Path(call_path) = *call_expr.func {
                //dbg!(&left_expr);
                // check Path {PathSegment} for Ident or literal,
                let call = call_path.path;
                //dbg!(&call);
                if let syn::PathSegment{ident: path_id, ..} = &call.segments[0] {
                    //dbg!(&path_id);
                    let call_id = format!("{:?}", path_id);
                    let call_id = LocalStmt::parse_ident(&call_id).expect("couldn't parse ident");
                    path_cond.id = Some(call_id);
                }                    
            }
        }

        path_cond
    }

    /******************* 
    after If expression is matched, we want to match the passed expr with either Path (identifier used) or Binary
        - to process the (possibly) nested expressions w/in an if statement)
    this should recursively check for Expr::Path or Expr::Binary in the left and right fields of the expr
    *******************/
    fn process_bin_op(expr: syn::Expr, path_cond: &mut PathCond) {
        // println!("in process bin op");
        //dbg!(&expr);
        let expression = expr.clone();
        path_cond.raw_expr = Some(expr);
        // if ExprPath, we have hit an id
        if let syn::Expr::Path(path) = expression {
            //dbg!(path);
            let syn::Path{leading_colon: _, segments} = path.path;
            let syn::PathSegment{ident, ..} = &segments[0];
            let cond_id = format!("{:?}", ident);
            let cond_id = LocalStmt::parse_ident(&cond_id).expect("couldn't parse ident");
            path_cond.id = Some(cond_id);
            path_cond.op = None;
            path_cond.op_type = None;
            path_cond.left = None;
            path_cond.right = None;
            path_cond.nested = None;

        // if ExprBinary, extract op, left/right exprs 
        } else if let syn::Expr::Lit(literal) = expression {
            path_cond.id = Some(format!("{:?}", literal));
        } else if let syn::Expr::Binary(bin_cond) = expression {
            //dbg!(&bin_cond);
            let bin_op = bin_cond.op;
            let mut nested_left: PathCond = PathCond::new();
            let mut nested_right: PathCond = PathCond::new();
            path_cond.left = Some(format!("{:?}", bin_cond.left));
            path_cond.right = Some(format!("{:?}", bin_cond.right));
            
            path_cond.op = Some(format!("{:?}", bin_op)); 
            // dbg!(&path_cond);
            if let syn::Expr::Binary(nested) = *bin_cond.left {
                // println!("left binary expr");
                // dbg!(&nested);
                Self::process_bin_op(syn::Expr::Binary(nested), &mut nested_left);
            } else if let syn::Expr::Path(nested) = *bin_cond.left {
                // println!("left path expr");
                // dbg!(&nested);
                Self::process_bin_op(syn::Expr::Path(nested), &mut nested_left);
            }
            if let syn::Expr::Binary(nested) = *bin_cond.right {
                // println!("right binary expr");
                // dbg!(&nested);
                Self::process_bin_op(syn::Expr::Binary(nested), &mut nested_right);
            } else if let syn::Expr::Path(nested) = *bin_cond.right {
                // println!("right path expr");
                // dbg!(&nested);
                Self::process_bin_op(syn::Expr::Path(nested), &mut nested_right);
            } else if let syn::Expr::Lit(nested) = *bin_cond.right {
                // println!("right literal expr");
                // dbg!(&nested);
                path_cond.id = Some(format!("{:?}", nested));
            }
            path_cond.nested = Some(Box::new((nested_left, nested_right)));
        }
        //dbg!(&path_cond);
    }

    pub fn is_declassify(self) -> bool {
        match self.expr_type {
            ExprType::CALL => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct PathCond {
    raw_expr: Option<syn::Expr>,
    id: Option<String>,
    left: Option<String>,
    op: Option<String>,
    op_type: Option<String>,
    right: Option<String>,
    nested: Option<Box<(PathCond, PathCond)>>,
}
impl fmt::Display for PathCond {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"Expr{{
        id: {:?}
        left: {:?}
        op: {:?}
        right: {:?}
        nested: {:?}
    }}"#,
    self.id.clone().unwrap_or("None".to_string()), 
    self.left.clone().unwrap(),
    self.op.clone().unwrap(),
    self.right.clone().unwrap(),
    self.nested.clone())
    }
}

impl PathCond {
    pub fn new() -> Self {
        Self {
            raw_expr: None,
            id: None,
            left: Some(String::new()),
            op: Some(String::new()),
            op_type: Some(String::new()),
            right: Some(String::new()),
            nested: None,
        }
    }
}
    