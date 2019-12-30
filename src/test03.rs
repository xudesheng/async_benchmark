use std::cell::RefCell;
use std::env::args;
use std::ffi::OsStr;
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;
use tokio::fs;
use tokio::io::AsyncBufReadExt;
use tokio::prelude::*;
use tokio::stream::StreamExt;

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();

    let mut rt = tokio::runtime::Runtime::new()?;
    let local = tokio::task::LocalSet::new();

    let count = local.block_on(&mut rt, main_inner()).unwrap();
    println!("{}, {}", count, now.elapsed().as_millis());
    Ok(())
}
async fn main_inner() -> Result<u64, std::io::Error> {
    let pathname = args().nth(1).expect("missing path argument");
    let p = Path::new(&pathname);

    let mut tasks = vec![];

    let count = Rc::new(RefCell::new(0u64));

    let mut entries = fs::read_dir(p).await?;

    while let Some(res) = entries.next().await {
        let entry = res?;
        let path = entry.path();
        //println!("{:?}", path);

        let count = count.clone();
        if path.extension() == Some(OsStr::new("csv")) {
            tasks.push(tokio::task::spawn_local(async move {
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
                *count.borrow_mut() += local_count;

                Ok(()) as Result<(), std::io::Error>
            }));
        }
    }

    for task in tasks {
        task.await??;
    }

    //println!("Total lines: {}", count.borrow());
    let count = (*count.borrow()).clone();
    Ok(count)
}
