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
    let mut count = 0u64;
    for entry in entries {
        let path = entry?.path();
        //println!("{:?}", path);
        if path.extension() == Some(OsStr::new("csv")) {
            let file = fs::File::open(path)?;
            let file = BufReader::new(file);
            let mut lines = file.lines();
            while let Some(line) = lines.next() {
                let line = line?;
                let content: Vec<_> = line.split(",").collect();
                if content.len() > 1 {
                    count += 1;
                }
            }
        }
    }
    //println!("Total lines: {}", count);
    println!("{}, {}", count, now.elapsed().as_millis());
    Ok(())
}
