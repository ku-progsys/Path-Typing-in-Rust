use rand::Rng;

pub fn run_simple3() {
    let maybe_secret: String = api_call();
    println!("{}", maybe_secret);
}

pub fn api_call() -> String {
    inner_call()
}

pub fn inner_call() -> String {
    let mut rng = rand::thread_rng();
    let random_bool: bool = rng.r#gen();
    let ret_str: String;
    if random_bool {
        ret_str = safe_innermost_call();
    }
    else {
        ret_str = unsafe_innermost_call();
    }
    ret_str
}

pub fn safe_innermost_call() -> String {
    "not secret".to_string()
}

pub fn unsafe_innermost_call() -> String {
    "secret".to_string()
}