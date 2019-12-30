use std::env::args;
use std::ffi::OsStr;
use std::path::Path;
use std::time::Instant;
use tokio::fs;
use tokio::io::AsyncBufReadExt;
use tokio::prelude::*;
use tokio::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();

    let pathname = args().nth(1).expect("missing path argument");

    let p = Path::new(&pathname);

    let mut tasks = vec![];

    let mut entries = fs::read_dir(p).await?;

    while let Some(res) = entries.next().await {
        let entry = res?;
        let path = entry.path();
        //println!("{:?}", path);

        if path.extension() == Some(OsStr::new("csv")) {
            tasks.push(tokio::spawn(async {
                let file = tokio::fs::File::open(path).await?;
                let file = io::BufReader::new(file);
                let mut lines = file.lines();
                let mut count = 0u64;
                while let Some(line) = lines.next_line().await? {
                    let content: Vec<_> = line.split(",").collect();
                    if content.len() > 1 {
                        count += 1;
                    }
                }

                Ok(count) as Result<u64, std::io::Error>
            }));
        }
    }

    let mut count = 0;
    for task in tasks {
        count += task.await??;
    }

    //println!("Total lines: {}", count);
    println!("{}, {}", count, now.elapsed().as_millis());

    Ok(())
}
