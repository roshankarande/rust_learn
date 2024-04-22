// fn greet(s: String){
//     println!("{s}");
// } 

fn main() {
    let s = "Hello".to_string();
    // greet(s);
    // greet(s);

    greet1(&s);
    greet1(&s);

    let mut s = "Roshan".to_string();

    greet_borrow_mut(&mut s);
    println!("{s}");

    let input = readline();
    println!("[{input}]");

}

fn readline() -> String{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn greet1(s : &String){
    println!("{s}");
}

fn greet_borrow_mut(s: &mut String){
    *s = format!("Hello {s}");
}