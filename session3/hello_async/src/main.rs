use futures::{executor::block_on, future::join_all, join};

async fn say_hello() {
    println!("Hello World");
    // second_function().await;

    join!(second_function(),say_goodbye());

    let result = double_it(3).await; // can also do result.await afterwards;
    println!("Result {result}");
     
    let futures = vec![double_it(1), double_it(2), double_it(3)];

    let results = join_all(futures).await;

    println!("{results:?}");
    
}

async fn second_function(){
    println!("Second function");
}

async fn say_goodbye() {
    println!("Good Bye");  
}

async fn double_it(a: u32) -> u32 {
    2*a
}

fn main() {

    block_on(say_hello());
 
}
