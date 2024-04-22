use std::{io::ErrorKind, path::Path};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
enum UserError{
    #[error("No users found")]
    NoUsers,

    #[error("Too many users found")]
    TooManyUsers,
}

fn load_users_3() -> Result<Vec<User>, UserError> {
    let my_path = Path::new("users.json");

    // remap the error to NoUsers Error
    let raw_text = std::fs::read_to_string(my_path).map_err(|_| UserError::NoUsers)?; 
    let users: Vec<User> = serde_json::from_str(&raw_text).map_err(|_| UserError::TooManyUsers)?;
    
    Ok(users)
}


fn maybe_read_a_file(filename: &str) -> Result<String, std::io::Error>{
    let filehandler = Path::new(filename);
    std::fs::read_to_string(filehandler)
}

fn file_to_uppercase() -> Result<String, std::io::Error>{
    let content = maybe_read_a_file("myfile.txt")?;
    Ok(content)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User{
    user: String
}

fn load_users_0() -> Result<Vec<User>, std::io::Error> {
    let my_path = Path::new("users.json");
    let raw_text = std::fs::read_to_string(my_path)?;
    let users: Vec<User> = serde_json::from_str(&raw_text)?;
    
    Ok(users)
}

type GenericResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn load_users_1() -> GenericResult<Vec<User>> {
    let my_path = Path::new("users.json");
    let raw_text = std::fs::read_to_string(my_path)?;
    let users: Vec<User> = serde_json::from_str(&raw_text)?;

    Ok(users)
}

fn load_users_2() -> anyhow::Result<Vec<User>> {
    let my_path = Path::new("users.json");
    let raw_text = std::fs::read_to_string(my_path)?;
    let users: Vec<User> = serde_json::from_str(&raw_text)?;
    Ok(users)
}

fn main() {
    let myfile = Path::new("my_file.txt");

    let content: Result<String, std::io::Error> = std::fs::read_to_string(myfile);

    match content {
        Ok(contents) => {println!("{contents}");},
        Err(e) => match e.kind() {
            ErrorKind::NotFound => println!("File Not found {:?}", myfile.file_name().unwrap()),
            _ => println!("Error :: {e}"),
        }
    }

    let _ = file_to_uppercase();
    let _ = load_users_0();
    let _ = load_users_1();
    let _ = load_users_2();
    let _ = load_users_3();
}
