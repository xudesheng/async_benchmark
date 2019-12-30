use std::env::args;
use std::ffi::OsStr;
use std::fs;
use std::io::*;
use std::path::Path;
use std::thread;
use std::time::Instant;

fn main() -> Result<()> {
    let now = Instant::now();

    let pathname = args().nth(1).expect("missing path argument");
    let p = Path::new(&pathname);

    let entries = fs::read_dir(p)?;
    let mut count = 0u64;

    let mut thread_vec: Vec<_> = Vec::new();

    for entry in entries {
        let path = entry?.path();
        //println!("{:?}", path);
        if path.extension() == Some(OsStr::new("csv")) {
            let file = fs::File::open(path)?;
            let file = BufReader::new(file);

            let thread_handler = thread::spawn(move || {
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

                local_count
            });

            thread_vec.push(thread_handler);
        }
    }

    for thread_handler in thread_vec {
        count += thread_handler.join().unwrap();
    }

    //println!("Total lines: {}", count);
    println!("{}, {}", count, now.elapsed().as_millis());
    Ok(())
}
