use std::io;
use colored::*;
use serde::{Serialize, Deserialize};
use std::fmt::Display;

use crate::input;

pub mod date;

trait IntoOption {
   fn into_option(self) -> Option<String>;
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub name: String,
    pub description: Option<String>,
    pub due_date: date::Date
}

 
impl Default for Task {
    fn default() -> Self {
        Task {
            name: String::new(),
            description: None,
            due_date: date::Date::default()
        }
   }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name: {}\n", self.name)?;
        write!(f, "Description: {}\n", self.description.as_ref().unwrap_or(&"No description\n".to_string()))?;
        write!(f, "Due date: {}", self.due_date)
    }
}

impl IntoOption for String {
   fn into_option(self) -> Option<String> {
       Some(self)
   }
}

impl Task {
   pub fn new() -> Task {
        let mut task = Task::default();
        task.name = get_name();
        task.description = get_description();
        task.due_date = date::Date::new();
        task
   }
}

pub fn get_name() -> String {
    let mut word = String::new();
    loop {
        println!("Please enter the name of the task: ");
        match io::stdin().read_line(&mut word) {
            Ok(_) => {
                if word.trim().is_empty() {
                    eprintln!("{}", "Name cannot be empty".red());
                    continue;
                }
                word = word.trim().to_string();
                break;
            }
            Err(e) => input::print_error(&e)
        };
    }
    word
}

pub fn get_description() -> Option<String> {
    println!("Please enter the description of the task (can be empty): ");
    let mut description = String::new();
    let mut value = Some(description.clone());
    loop {
        match io::stdin().read_line(&mut description) {
            Ok(_) => break,
            Err(e) => input::print_error(&e)
        };
    }
    if description != "\n" && description != " " && description.trim().is_empty() == false {
        value = description.trim().to_string().into_option();
    }
    value
}