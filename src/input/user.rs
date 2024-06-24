use std::io;
use colored::*;
use serde::{Deserialize, Serialize};
use std::process::exit;
use regex::Regex;

pub mod task;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    email: String,
    pub username: String,
    pub password: String,
    task: task::Task
}

impl Default for User {
    fn default() -> Self {
        User {
            email: String::new(),
            username: String::new(),
            password: String::new(),
            task: task::Task::default()
        }
    }
}

impl User {
    pub fn new() -> User {
        println!("Create a new account");
        let mut user = User::default();
        user.email = get_email();
        user.username = get_username();
        user.password = get_password();
        user.task = task::Task::new();
        user
    }

    pub fn signup() -> User {
        let user = User::new();
        match crate::file::add::save_to_json(&user) {
            Ok(_) => (),
            Err(e) => crate::input::print_error(&e)
        };
        user
    }

    pub fn login() {
        let username = crate::input::user::get_username();
        let password = crate::input::user::get_password();
        let user = match crate::file::search::search_user(username, password) {
            Some(value) => value,
            None => {
                println!("User not found\n0. Exit\n1. Signup\n2. Try Again");
                let value = crate::choice::input(2);
                if value == 0 {
                    println!("{}", "Thank you".green());
                    exit(0);
                }
                else if value == 1 {
                    let _info = crate::input::user::User::signup();
                }
                else if value == 2 {
                    crate::input::user::User::login();
                }
                crate::input::user::User::default()
            }
        };
        crate::input::user::print_details(user);
    }
}

pub fn get_username() -> String {
    let mut word = String::new();
    loop {
        println!("Please enter the username: ");
        match io::stdin().read_line(&mut word) {
            Ok(_) => {
                if word.trim().is_empty() {
                    eprintln!("{}", "Name cannot be empty".red());
                    continue;
                }
                word = word.trim().to_string();
                break;
            }
            Err(e) => crate::input::print_error(&e)
        }
        word.clear();
    }
    word
}

pub fn get_password() -> String {
    let mut word = String::new();
    loop {
        println!("Please enter the password: ");
        match io::stdin().read_line(&mut word) {
            Ok(_) => {
                if word.trim().is_empty() {
                    eprintln!("{}", "Password cannot be empty".red());
                    continue;
                }
                else if validate_password(word.clone().trim().to_string()) == false { continue; }
                word = word.trim().to_string();
                break;
            }
            Err(e) => crate::input::print_error(&e)
        }
        word.clear();
    }
    word
}

pub fn get_email() -> String {
    let mut word = String::new();
    loop {
        println!("Please enter the email id: ");
        match io::stdin().read_line(&mut word) {
            Ok(_) => {
                if word.trim().is_empty() {
                    eprintln!("{}", "Email cannot be empty".red());
                    continue;
                }
                else if validate_email(word.clone().trim().to_string()) == false {
                    eprintln!("{}", "Invalid email id".red());
                    continue;
                }
                word = word.trim().to_string();
                break;
            }
            Err(e) => crate::input::print_error(&e)
        }
        word.clear();
    }
    word
}

pub fn print_details(user:User) {
    let length = user.password.len();
    println!("Email: {}", user.email);
    println!("Username: {}", user.username);
    print!("Password: ");
    for _index in 0..length {
        print!("{}", "*".red());
    }
    println!("\nTask: {}", user.task);
}

fn validate_password(password:String) -> bool {
    if password.len() < 8 {
        eprintln!("{}", "Password is too small".red());
        return false;
    }
    let compare = ",.(){}[]|/\\;:'\"";
    for letter in password.chars() {
        if compare.contains(letter) {
            eprintln!("{}", "Invalid password".red());
            return false;
        }
    }
    true
}

fn validate_email(email:String) -> bool {
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,64}$").unwrap();
    email_regex.is_match(&email)
}