use core::hash;
use std::{collections::HashMap, fmt::format, hash::{Hash, Hasher}, io::stdin, path::Path};
use serde::{Serialize, Deserialize};

pub fn hash_password(password: &str) -> String{
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(password);
    format!("{:X}", hasher.finalize())
}

pub fn greet_user(name: &str) -> String{
    format!("Hello {name}")
}

#[derive(PartialEq, Debug, Clone)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub enum LoginRole {
    Admin,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User{
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User{
    pub fn new(username: &str, password: &str, role: LoginRole) -> User{
        Self{
            username: username.to_lowercase(),
            password: hash_password(password),
            role,
        }
    }
}

pub fn get_default_users() -> HashMap<String, User>{
    let mut users: HashMap<String, User> = HashMap::new();
    users.insert("admin".to_string(), User::new("admin", "password", LoginRole::Admin));
    users.insert("bob".to_string(), User::new("bob", "password", LoginRole::User));

    users
}

pub fn save_users(users: HashMap<String, User>) {
    let users_path = Path::new("users.json");
    let users_json = serde_json::to_string(&users).unwrap();
    std::fs::write(users_path, users_json).unwrap();
}


pub fn get_users() -> HashMap<String, User>{
    let users_path = Path::new("users.json");
    if users_path.exists(){
        let users_json = std::fs::read_to_string(users_path).unwrap();
        let users: HashMap<String, User> = serde_json::from_str(&users_json).unwrap();
        users
    }else{
        let users = get_default_users();
        let users_json = serde_json::to_string(&users).unwrap();
        std::fs::write(users_path, users_json).unwrap();
        users
    }
}

pub fn get_admin_users() -> Vec<User>{
    get_users().into_iter()
    .filter(|(_,v)| v.role == LoginRole::Admin)
    .map(|(_,v)| v)
    .collect()
}



pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let users = get_users();

    if let Some(user) = users.get(username) {
        if user.password == hash_password(password) {
            Some(LoginAction::Granted(user.role.clone()))
        } else {
            Some(LoginAction::Denied)
        }
    } else {
        None
    }
}

pub fn readline() -> String{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user() {
        assert_eq!("Hello Roshan", greet_user("Roshan"));
    }

    #[test]
    fn test_login(){
        assert_eq!(login("admin", "password"), Some(LoginAction::Granted(LoginRole::Admin)));
        assert_eq!(login("bob", "password"), Some(LoginAction::Granted(LoginRole::User)));
        assert_eq!(login("admin", "xyz"), Some(LoginAction::Denied));
    }
    
}

