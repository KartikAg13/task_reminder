use std::fs::File;
use std::io::Read;

use crate::input;
use crate::constants::FILENAME;

pub fn search_user(username:String, password:String) -> Option<input::user::User> {
    let mut data = String::new();
    let mut file = match File::open(FILENAME) {
        Ok(value) => value,
        Err(e) => {
            input::print_error(&e);
            return None;
        }
    };
    match file.read_to_string(&mut data) {
        Ok(_) => (),
        Err(e) => input::print_error(&e)
    }
    let users: Vec<input::user::User> = match serde_json::from_str(&data) {
        Ok(value) => value,
        Err(e) => {
            input::print_error(&e);
            return None;
        }
    };
    for user in users.iter() {
        if user.username == *username && user.password == *password {
            let value = user.clone();            
            return Some(value);
        }
    }
    None
}