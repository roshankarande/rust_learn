use std::time::Duration;

use tokio::{self, time::sleep};
use anyhow::Result;
use tracing::subscriber;
use tracing_subscriber::fmt::format::FmtSpan;

#[tracing::instrument]
async fn hello_world() {
   println!("Hello World"); 
   sleep(Duration::from_secs(1)).await;
}



#[tokio::main]
async fn main() -> anyhow::Result<()> {

    // let subscriber = tracing_subscriber::FmtSubscriber::new();

    let subscriber = tracing_subscriber::fmt()
                                                    .compact()
                                                    // Display source code file paths
                                                    .with_file(true)
                                                    // Display source code line numbers
                                                    .with_line_number(true)
                                                    // Display the thread ID an event was recorded on
                                                    .with_thread_ids(true)
                                                    // Don't display the event's target (module path)
                                                    .with_span_events(FmtSpan::ENTER | FmtSpan::EXIT | FmtSpan::CLOSE)
                                                    .with_target(false)
                                                    // Build the subscriber
                                                    
                                                    .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    tracing::info!("Starting up!");
    tracing::warn!("Are you sure this is a good idea?");
    tracing::error!("Something went horribly wrong");
    tracing::debug!("Debug level");

    hello_world().await;

    Ok(())
}
