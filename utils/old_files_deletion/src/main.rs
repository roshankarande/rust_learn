use std::env;
use std::fs;
use std::time::Duration;

use tokio::join;
// use std::path::Path;

pub async fn cleanup_files(
    cleanup_path: &str,
    lifetime_secs: u64,
    polling_interval_secs: u64,
) -> anyhow::Result<()> {

    let ext = "toml";
    // let current_dir =  env::current_dir().unwrap().as_path().file_name().unwrap();

    if let Ok(dir) = env::current_dir() {
        let path = dir.into_os_string().into_string().unwrap();
        println!("Current Directory :: {:?}", path);
    };

    loop {
        let entries = fs::read_dir(cleanup_path)?;

        for entry in entries {
            let entry = entry?;
            let metadata = entry.metadata()?;

            let modified_time = metadata.modified()?;

            // Calculate the duration since the file was created
            let duration_since_creation = match modified_time.elapsed() {
                Ok(duration) => duration,
                Err(_) => {
                    // Unable to get the duration, skip this file
                    continue;
                }
            };

            println!(
                "{:?} modified {:.2?} secs ago",
                entry.file_name(),
                modified_time.elapsed().unwrap().as_secs_f64()
            );

            // Check if the modified duration is greater than lifetime_secs
            if duration_since_creation > Duration::from_secs(lifetime_secs) {
                // Delete the file
                let file_path = entry.path();
                let extension = file_path.extension().unwrap().to_ascii_lowercase();
                if metadata.is_file() && extension == ext {
                    fs::remove_file(&file_path)?;
                    println!("Deleted file: {:?}", file_path);
                }
                // else if metadata.is_dir() {
                //     // fs::remove_dir_all(&file_path)?;
                //     println!("Deleted directory: {:?}", file_path);
                // }
            }
        }

        tokio::time::sleep(Duration::from_secs(polling_interval_secs)).await;
    }
}

async fn hello_world() {
    println!("Hello");
}

#[tokio::main]
async fn main() {
    let result = join!(
        cleanup_files("assets", 20, 5), 
        hello_world()
    );

    println!("{:?}", result);
}
