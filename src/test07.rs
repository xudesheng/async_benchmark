use rayon::prelude::*;
use std::env::args;
use std::ffi::OsStr;
use std::fs;
use std::io::*;
use std::path::Path;
use std::time::Instant;

fn main() -> Result<()> {
    let now = Instant::now();

    let pathname = args().nth(1).expect("missing path argument");
    let p = Path::new(&pathname);

    let entries = fs::read_dir(p)?;

    let mut path_vec: Vec<_> = Vec::new();

    for entry in entries {
        let path = entry?.path();
        //println!("{:?}", path);
        path_vec.push(path);
    }

    let count: u64 = path_vec
        .par_iter_mut()
        .map(|path| {
            let mut count = 0u64;
            if path.extension() == Some(OsStr::new("csv")) {
                let file = match fs::File::open(path) {
                    Ok(file) => file,
                    Err(_) => return 0u64,
                };
                let file = BufReader::new(file);
                let mut lines = file.lines();

                while let Some(line) = lines.next() {
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
            }

            count
        })
        .reduce(|| 0, |x, y| x + y);

    //println!("Total lines: {}", count);
    println!("{}, {}", count, now.elapsed().as_millis());
    Ok(())
}
