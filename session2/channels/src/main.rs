use std::sync::mpsc;

enum Command {
    SayHello,
    Quit,
}

fn main() {
    let (tx, rx) = mpsc::channel::<Command>(); // channel of Command type

    let handle = std::thread::spawn(move || 
        {
            while let Ok(command) = rx.recv(){
                match command{
                    Command::SayHello => println!("Hello"),
                    Command::Quit => {
                        println!("Quit");
                        break;
                    },

                }
            }
        });  // send the receiver inside

    

        for _ in 0..10 {
            tx.send(Command::SayHello).unwrap();
        }

        println!("Sending Quit");
        tx.send(Command::Quit).unwrap();

        handle.join().unwrap();
}
