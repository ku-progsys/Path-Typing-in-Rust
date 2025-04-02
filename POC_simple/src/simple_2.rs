
pub fn run_simple2(flag: bool) {
    let new_user: User = User::new("jack", "jack@mail.com", "1232 Address st.", "111-11-1111");
    let mut user_info: String;
    if flag {
        user_info = safe_get_usr_data(new_user);
    }
    else {
        user_info = new_user.unsafe_get_usr_data();
    }
    println!("User Info:\n{:?}", user_info);
}


struct User {
    pub name: String,
    pub email: String,
    address: String,
    ssn:  String
}

impl User {
    pub fn new(usr_name: &str, usr_email: &str, usr_address: &str, usr_ssn: &str) -> Self {
        Self {
            name: usr_name.to_string(),
            email: usr_email.to_string(),
            address: usr_address.to_string(),
            ssn: usr_ssn.to_string()
        }
    }
    fn unsafe_get_usr_data(&self) -> String {
        let usr_data: String = format!("Name: {}, Email: {}, Address: {}, SSN: {}",
        &self.name, &self.email, &self.address, &self.ssn);
        usr_data
    } 
}

pub fn safe_get_usr_data(user: User) -> String {
    let usr_data: String = format!("Name: {}, Email: {}", user.name, user.email);
    usr_data
}