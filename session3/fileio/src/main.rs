use std::{fs::File, io::{self, BufRead}, path::Path, time::Instant};

use anyhow::Ok;

// Taken from: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// where P: AsRef<Path>, {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }

// fn main() {
//     let now = std::time::Instant::now();
//     let mut line_count = 0;
//     if let Ok(lines) = read_lines("warandpeace.txt") {
//         lines.for_each(|line| {
//             if let Ok(line) = line {
//                 if !line.trim().is_empty() {
//                     line_count += 1;
//                 }
//             }
//         });
//     }
//     println!("Read {} lines in {:.3} seconds", line_count, now.elapsed().as_secs_f32());
// }

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Result::Ok(io::BufReader::new(file).lines())
}

async fn line_count(filename: String) -> anyhow::Result<usize> {
    let now = std::time::Instant::now();
    let mut line_count = 0;
    if let Result::Ok(lines) = read_lines(filename) {
        lines.for_each(|line| {
            if let Result::Ok(line) = line {
                if !line.trim().is_empty() {
                    line_count += 1;
                }
            }
        });
    }
    println!("Read {} lines in {:.3} seconds", line_count, now.elapsed().as_secs_f32());
    Ok(line_count)
}

async fn async_line_count(filename: String) -> anyhow::Result<usize>{
    use tokio::io::AsyncBufReadExt;
    use tokio::io::BufReader;
    use tokio::fs::File;

    println!("Reading {filename}");
    let now = Instant::now();
    let mut line_count = 0;

    let file = File::open(filename).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();   // stream and not iterator -> tokio

    while let Some(line) = lines.next_line().await?  {
        if !line.trim().is_empty(){
            line_count += 1;
        }
    }
    println!("Read {} lines in {:.3} seconds", line_count, now.elapsed().as_secs_f32());
    Ok(line_count)


}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let now = std::time::Instant::now();
    let (c1, c2,..) = tokio::join!(
        async_line_count("warandpeace.txt".to_string()),
        async_line_count("warandpeace.txt".to_string()),
        async_line_count("warandpeace.txt".to_string()),
        async_line_count("warandpeace.txt".to_string()),
        async_line_count("warandpeace.txt".to_string()),
        async_line_count("warandpeace.txt".to_string())
    );
    println!("Total lines: {}", c1? + c2?);
    println!("In {:.3} seconds", now.elapsed().as_secs_f32());
    Ok(())
}