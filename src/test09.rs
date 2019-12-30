use std::env::args;

use async_std::fs::*;
use async_std::io::{self, BufReader};
use async_std::prelude::*;
use async_std::task;
use std::ffi::OsStr;
use std::path::Path;
use std::time::Instant;

fn main() -> io::Result<()> {
    let now = Instant::now();

    let pathname = args().nth(1).expect("missing path argument");

    let count: u64 = task::block_on(async {
        let path = Path::new(&pathname);
        let mut entries = match read_dir(path).await {
            Ok(entries) => entries,
            Err(_) => return 0u64,
        };
        let mut count = 0u64;
        let mut tasks: Vec<_> = Vec::new();

        while let Some(res) = entries.next().await {
            match res {
                Ok(entry) => {
                    let path = entry.path();
                    if path.extension() == Some(OsStr::new("csv")) {
                        tasks.push(task::spawn(async move {
                            let mut count = 0u64;

                            let file = match File::open(path).await {
                                Ok(file) => file,
                                Err(_) => return 0u64,
                            };
                            let mut lines = BufReader::new(file).lines();

                            while let Some(line) = lines.next().await {
                                match line {
                                    Ok(line) => {
                                        let content: Vec<_> = line.split(",").collect();
                                        if content.len() > 1 {
                                            count += 1;
                                        }
                                    }
                                    Err(_) => {}
                                }
                            }
                            count
                        }))
                    }
                }
                Err(_) => {}
            }
        }

        for task in tasks {
            count += task.await;
        }
        //println!("The file contains {} lines.", count);
        count
    });

    println!("{}, {}", count, now.elapsed().as_millis());
    Ok(())
}
