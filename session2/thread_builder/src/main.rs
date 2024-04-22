use std::thread;


fn my_thread(){
    println!("Hello from thread named :: '{}'", thread::current().name().unwrap());

}


fn main() {
    println!("Hello, world!");

    let builder = thread::Builder::new()
    .name("Named Thread".to_string())
    .stack_size(std::mem::size_of::<usize>()*4);

    let handler = builder.spawn(my_thread).unwrap();
    handler.join().unwrap();


}
