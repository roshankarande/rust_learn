use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::thread;
use std::time::{SystemTime, Duration};
// use std::path::Path;

fn main() -> io::Result<()> {
    // Specify the folder path
    let folder_path = "assets";

    // Get the current time
    let current_time = SystemTime::now();

    println!("Current Directory :: {:?} Current Time :: {:?}", env::current_dir().unwrap().file_name().unwrap(), current_time);

    loop{

    let entries = fs::read_dir(folder_path)?;

    for entry in entries {
        let entry = entry?;
        let metadata = entry.metadata()?;

        // Get the creation time of the file
        let created_time = metadata.created()?;
        let modified_time = metadata.modified()?;

        // Calculate the duration since the file was created
        let duration_since_creation = match created_time.elapsed() {
            Ok(duration) => duration,
            Err(_) => {
                // Unable to get the duration, skip this file
                continue;
            }
        };

        println!("{:?} created {:?} s ago | modified {:?} s ago | ext {:?}", entry.file_name(), created_time.elapsed().unwrap().as_secs_f64(), modified_time.elapsed().unwrap().as_secs_f64(), entry.path().extension().unwrap() );

        // Check if the duration is greater than 5 minutes
        if duration_since_creation > Duration::from_secs(5 * 60) {
            // Delete the file
            let file_path = entry.path();
            let extension = file_path.extension().unwrap().to_ascii_lowercase();
            if metadata.is_file() && extension == "toml"{
                fs::remove_file(&file_path)?;
                println!("Deleted file: {:?}", file_path);
            } 
            // else if metadata.is_dir() {
            //     // fs::remove_dir_all(&file_path)?;
            //     println!("Deleted directory: {:?}", file_path);
            // }
        }
    }

    thread::sleep(Duration::from_secs(10));
}

    Ok(())
}
