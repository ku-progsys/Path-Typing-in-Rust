
pub fn run_simple1(flag: bool) {
    let mut usr_data: String = get_usr_data();
    let data: i32 = 12;
    if flag == true && data == 12 { // make implicit example similar 
        process_data(&usr_data);
    }
    else {
        sanitize_secret(&mut usr_data);
    }
    declassify(&usr_data);
}

pub fn sanitize_secret(secret: &mut String) -> String {
    *secret = "not secret".to_string();
    secret.to_string()
}

pub fn declassify(data: &String) {
    println!("data is: {}", data);
}

pub fn get_usr_data() -> String {
    "secret data".to_string()
}

pub fn process_data(data: &String) -> String {
    "Address is: XXXXXXXX".to_string()
}