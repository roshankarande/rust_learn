use tokio::join;

async fn hello1() -> u32{
    3
}

async fn hello2() -> u32 {
    4
}

async fn ticker() {
    for i in 0..10{
        println!("tick {i}");
        tokio::task::yield_now().await;
    }
}

#[tokio::main]
async fn main() {
    let result = join!(hello1(), hello2());
    println!( "{result:?}");

    tokio::spawn(ticker());
    tokio::spawn(ticker()).await.unwrap();  // if you don't await it here on in future ... it might not always run
    hello1().await;


    let _ = join!(
        tokio::spawn(hello1()),
        tokio::spawn(ticker()),
        tokio::spawn(ticker()),
    );

    
}
