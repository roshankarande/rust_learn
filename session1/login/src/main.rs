// use std::collections::HashMap;

use authentication::{get_admin_users, get_users, greet_user, login, readline, LoginAction, LoginRole};

fn main() {
    println!("Hello, world!");
    let output = greet_user("Roshan");
    println!("{output}");

    let admin_usernames = get_admin_users();
    println!("{:?}",admin_usernames);

    let users = get_users();
    println!("{:?}",users);


    let mut tries = 0;
    loop {
        println!("Enter your username:");
        let username = readline();
        println!("Enter your password:");
        let password = readline();

        match login(&username, &password) {
            Some(LoginAction::Granted(role)) => {
                match role{
                    LoginRole::Admin => println!("Welcome Admin!"),
                    LoginRole::User => println!("Welcome User!"),
                }
                break;
            }
            Some(LoginAction::Denied) => println!("Incorrect username or password!"),
            None => println!("New user"),
        }

        tries += 1;
        if tries >= 3{
            println!("Too many failed logins!!");
            break;
        }
    }

}
