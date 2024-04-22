use core::panic;
use std::sync::Mutex;

static my_shared: Mutex<u32> = Mutex::new(3);
// fn main() {

//     let lock = my_shared.lock().unwrap();

//     std::mem::drop(lock); // Explicitly release the lock

//     if let Ok(_lock) = my_shared.try_lock(){
//         println!("We got the lock");
//     }else{
//         println!("Cannot acquire the lock");
//     }    
// }

fn main(){
    let handle = std::thread::spawn(poisoner);
    println!("trying to return from the thread");
    println!("{:?}", handle.join());

    // let lock = my_shared.lock();
    // println!("{lock:?}");

    if let Ok(_lock) = my_shared.try_lock(){
        println!("We got the lock");
    }else{
        println!("Cannot acquire the lock");
    }


}


fn poisoner(){
    let mut lock = my_shared.lock().unwrap();

    *lock += 1;
    panic!("The poisoner strikes"); // lock won't be released
}