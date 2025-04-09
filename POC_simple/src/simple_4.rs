
pub fn run_simple4(user_loc: String) {
    if secret_computation(user_loc) <= 5 { // if user's location is w/in 5 miles, serve advertisement
        show_ad();
    }
}

pub fn show_ad() {
    println!("go to place close to you");
}
pub fn secret_computation(address_in: String) -> u32 {
    let ad_location: String = "1235 Address st.".to_string();
    return distance(address_in, ad_location);
}

pub fn distance(addr_in: String, ad_loc: String) -> u32 {
    // convert users address to a location
    // convert address of advertisement to serve to a location 
    // return abs(addr_in - ad_loc)
    5
}