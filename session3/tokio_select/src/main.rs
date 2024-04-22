use std::time::Duration;

async fn do_work(){
    tokio::time::sleep(Duration::from_secs(1)).await;
}

async fn timeout(){
    tokio::time::sleep(Duration::from_secs(2)).await;
}


#[tokio::main]
async fn main() {
    tokio::select! {
        _ = do_work() => println!("do_work() finished first"),
        _ = timeout() => println!("timeout() finished first "),
    }
}
