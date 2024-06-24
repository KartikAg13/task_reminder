use std::process::exit;
use std::io;
use colored::*;

pub fn choice() {
    let choice;
    println!("0. Exit\n1. Login\n2. Signup");
    choice = input(2);
    if choice == 0 {
        println!("{}", "Thank you".green());
        exit(0);
    }
    else if choice == 1 {
        println!("Lets login");
        crate::input::user::User::login();
    }
    else if choice == 2 {
        println!("Lets signup");
        let user = crate::input::user::User::signup();
        crate::input::user::print_details(user);
    }

}

pub fn input(maximum:u32) -> u32 {
    let mut value = String::new();
    loop {
        match io::stdin().read_line(&mut value) {
            Ok(_) => {
                let _value:u32 = match value.trim().parse() {
                    Ok(number) => {
                        if number <= maximum {
                            return number;
                        }
                        else {
                            continue;
                        }
                    }
                    Err(e) => {
                        crate::input::print_error(&e);
                        continue;
                    }
                };
            }
            Err(e) => crate::input::print_error(&e)
        };
    }
}