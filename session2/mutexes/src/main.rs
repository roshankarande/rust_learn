use std::sync::Mutex;

static NUMBERS: Mutex<Vec<u32>> = Mutex::new(Vec::new());

fn main() {
    let mut handles = Vec::new();

    for _ in 0..10{
        let handle = std::thread::spawn(||{
            let mut lock = NUMBERS.lock().unwrap(); // before accessing NUMBERS..acquire the lock
            lock.push(1); // as scope ends the lock ends.. 

        });
        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());

    let lock = NUMBERS.lock().unwrap();
    println!("{:?}",lock);

}
