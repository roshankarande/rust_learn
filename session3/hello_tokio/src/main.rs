use tokio::join;

async fn hello() {
    println!("Hello Tokio");
}

// #[tokio::main]

#[tokio::main(flavor="current_thread")]
async fn main(){
    hello().await;
}

//// The Hard way
// fn main() {
//     // Have to specify runtime because tokio offers more features...
//     let rt = runtime::Builder::new_current_thread()
//         .enable_all() // enable runtime features
//         .build() // builds the runtime
//         .unwrap();

//     rt.block_on(hello());

//     // 4 Multi-threaded environment
//     // let rt = runtime::Builder::new_multi_thread()
//     //     .worker_threads(4)    
//     //     .enable_all() // enable runtime features
//     //     .build() // builds the runtime
//     //     .unwrap();
// }
