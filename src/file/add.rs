use serde_json::{from_str, to_string_pretty};
use std::fs::OpenOptions;
use std::io::{Read, Write, Error, ErrorKind};
use colored::*;

use crate::input;
use crate::constants::FILENAME;

pub fn save_to_json(user: &input::user::User) -> Result<(), Error> {
    let mut file_content = String::new();
    let _file = match OpenOptions::new().read(true).open(FILENAME) {
        Ok(mut file) => {
            file.read_to_string(&mut file_content)?;
            file
        },
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                file_content = "[]".to_string();
                OpenOptions::new().create(true).write(true).open(FILENAME)?
            } else {
                eprintln!("{}", format!("Error opening file: {}", e).red());
                return Err(e);
            }
        }
    };
    let mut users: Vec<input::user::User> = match from_str(&file_content) {
        Ok(data) => data,
        Err(_) => Vec::new(),
    };
    users.push(user.clone());
    let json_data = match to_string_pretty(&users) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("{}", format!("Error serializing user data: {}", e).red());
            return Err(e.into());
        }
    };
    let mut file = OpenOptions::new().write(true).truncate(true).open(FILENAME)?;
    match file.write_all(json_data.as_bytes()) {
        Ok(_) => {
            println!("{}", "Data written to file successfully.".green());
        },
        Err(e) => {
            eprintln!("{}", format!("Error writing to file: {}", e).red());
            return Err(e);
        }
    };
    Ok(())
}