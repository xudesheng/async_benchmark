use std::env::args;
use std::ffi::OsStr;
use std::fs;
use std::io::*;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;
use threadpool::ThreadPool;

fn main() -> Result<()> {
    let now = Instant::now();

    let pathname = args().nth(1).expect("missing path argument");
    let p = Path::new(&pathname);

    let entries = fs::read_dir(p)?;
    let count = Arc::new(Mutex::new(0u64));

    let workers = num_cpus::get() - 1;
    let pool = ThreadPool::new(workers);

    for entry in entries {
        let path = entry?.path();

        if path.extension() == Some(OsStr::new("csv")) {
            let file = fs::File::open(path)?;
            let file = BufReader::new(file);
            let count_copy = Arc::clone(&count);

            pool.execute(move || {
                let mut local_count = 0u64;
                let mut lines = file.lines();
                while let Some(line) = lines.next() {
                    match line {
                        Ok(line) => {
                            let content: Vec<_> = line.split(",").collect();
                            if content.len() > 1 {
                                local_count += 1;
                            }
                        }
                        Err(_) => {}
                    }
                }
                let mut count_copy = count_copy.lock().unwrap();
                *count_copy += local_count;
            });
        }
    }
    pool.join();

    let count = count.lock().unwrap();
    println!("{}, {}", *count, now.elapsed().as_millis());
    Ok(())
}
