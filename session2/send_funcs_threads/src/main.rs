use std::sync::mpsc;

type Job = Box<dyn FnOnce() + Send + 'static>; // advanced concepts... we will see it later

// this is static function
fn hi_there(){
    println!("Hello world");

}

fn main() {
    let (tx, rx) = mpsc::channel::<Job>();

    let handle = std::thread::spawn(move || {
        while let Ok(job) = rx.recv(){
            job();
        }

    });

    let job = || println!("hello from a closure");
    let job2 = || {
        for i in 0..10{
            println!("{i}");
        }
    };

    tx.send(Box::new(job)).unwrap();
    tx.send(Box::new(job2)).unwrap();
    tx.send(Box::new(hi_there)).unwrap();
    tx.send(Box::new(||println!("Hello There!"))).unwrap();

    handle.join().unwrap();


}
