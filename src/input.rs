use colored::*;
use std::error::Error;

pub mod user;

pub fn print_error(e:&dyn Error) {
    eprintln!("{}", format!("Error encountered: {}", e).red())
}