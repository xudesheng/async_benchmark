use std::env::args;
use std::ffi::OsStr;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;
use tokio::fs;
use tokio::io::AsyncBufReadExt;
use tokio::prelude::*;
use tokio::stream::StreamExt;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let pathname = args().nth(1).expect("missing path argument");
    let p = Path::new(&pathname);

    let mut tasks = vec![];

    let count = Arc::new(Mutex::new(0u64));

    let mut entries = fs::read_dir(p).await?;

    while let Some(res) = entries.next().await {
        let entry = res?;
        let path = entry.path();
        //println!("{:?}", path);

        let count = count.clone();
        if path.extension() == Some(OsStr::new("csv")) {
            tasks.push(tokio::spawn(async move {
                let file = tokio::fs::File::open(path).await?;
                let file = io::BufReader::new(file);
                let mut lines = file.lines();

                let mut local_count = 0u64;
                while let Some(line) = lines.next_line().await? {
                    let content: Vec<_> = line.split(",").collect();
                    if content.len() > 1 {
                        local_count += 1;
                    }
                }
                let mut count = count.lock().await;
                *count += local_count;
                Ok(()) as Result<(), std::io::Error>
            }));
        }
    }
    for task in tasks {
        task.await??;
    }
    let count = count.lock().await;
    //println!("Total lines: {}", *count);
    println!("{}, {}", *count, now.elapsed().as_millis());

    Ok(())
}
