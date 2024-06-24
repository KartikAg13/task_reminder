use std::io;
use colored::*;
use serde::{Serialize, Deserialize};
use std::fmt::Display;

use crate::input;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Date {
    day: u32,
    month: u32,
    year: u32
}


impl Default for Date {
    fn default() -> Self {
        Date {
            day: 0,
            month: 0,
            year: 0
        }
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}/{}\n", self.day, self.month, self.year)
    }
}


impl Date {
    pub fn new() -> Date {
        loop {
            let mut final_date:Vec<u32> = Vec::new();
            let mut date = String::new();
            println!("Please enter the due date of the task (dd/mm/yyyy): ");
            loop {
                match io::stdin().read_line(&mut date) {
                    Ok(_) => break,
                    Err(e) => input::print_error(&e)
                };
            }
            let date:Vec<&str> = date.trim().split('/').collect();
            if (date.len() == 3 && (date[0].is_empty() || date[1].is_empty() || date[2].is_empty())) || date.len() < 3 {
                eprintln!("{}", "Invalid date entered. Please try again.".red());
                continue;
            }
            for &elements in date.iter() {
                match elements.trim().parse() {
                    Ok(value) => final_date.push(value),
                    Err(e) => {
                            input::print_error(&e);
                            continue;
                    }
                }
            }
            let date = Date {
                day: final_date[0],
                month: final_date[1],
                year: final_date[2]
            };
            if validate_date(Date { day: final_date[0], month: final_date[1], year: final_date[2] }) {
                println!("{}", "Date entered successfully.".green());
                return date;
            }
            else {
                eprintln!("{}", "Invalid date entered. Please try again.".red());
            }
            final_date.clear();
        }
    }
}


fn validate_date(date:Date) -> bool {
    let days:Vec<u32> = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if date.year >= 2025 {
        if date.month >= 1 && date.month <= 12 {
            if date.month == 2 {
                let mut leap = false;
                if date.year % 4 == 0 {
                    leap = true;
                    if date.year % 100 == 0 && date.year % 400 != 0 {
                        leap = false;
                    }
                }
                if leap && date.day >= 1 && date.day <= 29 {
                    return true;
                }
                else {
                    return false;
                }
            }
            else if date.day <= days[(date.month - 1) as usize] {
                return true;
            }
        }
    }
    return false;
}

