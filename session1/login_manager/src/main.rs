use authentication::{get_users, save_users, LoginRole, User};
use clap::{builder::Str, Parser, Subcommand};

#[derive(Parser)]
#[command()]
struct Args {
  #[command(subcommand)]
  command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
  /// List all users.
  List,
  /// Add a User
  Add {
    /// User's login name
    username: String,

    /// User's password (plaintext)
    password: String,

    /// Optional - mark as an admin
    #[arg(long)]
    admin: Option<bool>,
  },

  /// Delete a User
  Delete {
    /// Username to be deleted
    username: String,
  },

  /// Change a user's password
  ChangePassword{
    /// Username whose password need's to be changed
    username: String,
    /// New Password
    password: String,
  }
}

fn add_user(username: String, password: String, admin: bool){
    let mut users = get_users();
    let role = if admin{
        LoginRole::Admin
    }else{
        LoginRole::User
    };

    let user = User::new(&username, &password, role);
    users.insert(username, user);
    save_users(users);

}

fn delete_user(username: String){
    let mut users = get_users();

    if users.contains_key(&username){
        users.remove(&username);
        save_users(users);
    }else{
        println!("{username} does not exist");
    }
    
}

fn change_password(username: String, password: String){
    let mut users = get_users();

    if let Some(user) = users.get_mut(&username){
        user.password = authentication::hash_password(&password);
        save_users(users);

    }else{
        println!("Name doesn't exist!");
    }
}

fn list_users() {
    println!("{:<20} {:<20}","Username", "Password");
    println!("{:-<40}","");

    let users = get_users();
    users.iter()
    .for_each(|(_,v)| 
    {
        println!("{:<20} {:<20?}", v.username, v.role );
    })
}


fn main() {
    let cli = Args::parse();
    match cli.command {
        Some(Commands::List) => {
            println!("All Users Goes Here\n");
            list_users();
        },
        Some(Commands::Add { username, password, admin }) => {
            // println!("Add a user");
            add_user(username,password, admin.unwrap_or(false));
        },
        Some(Commands::Delete { username }) => {
            delete_user(username);
        },
        Some(Commands::ChangePassword { username, password }) => {
            change_password(username, password);
        },
        None => {
            println!("Run with --help to see instructions");
            std::process::exit(0);
        }
    }    
}