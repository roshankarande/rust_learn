use once_cell::sync::Lazy;
use std::sync::RwLock;

static USERS: Lazy<RwLock<Vec<String>>> = Lazy::new(|| RwLock::new(build_users()));

fn build_users() -> Vec<String>{
    vec!["Roshan".to_string(), "Naren".to_string(), "Parikshit".to_string()]
}

fn read_line() -> String {
    let mut buf: String = String::new(); 
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn main() {
    // Background Thread
    std::thread::spawn(|| {
        loop {
            println!("Current users (in a thread)");
            let users = USERS.read().unwrap();
            println!("{users:?}");
            std::thread::sleep(std::time::Duration::from_secs(3));
        }
    });

    loop{
        println!("Enter a name to add to user list (q to quit)");
        let input = read_line();

        if input.to_lowercase() == "q"{
            break;
        }else{
            let mut lock = USERS.write().unwrap();
            lock.push(input);
        }

    }
   
}
