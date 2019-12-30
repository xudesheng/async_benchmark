use chrono::prelude::*;
use std::env::args;
use std::fs::{self, File};
use std::io::BufWriter;
use std::io::Write;
use std::process::Command;
use std::time::Instant;

use std::path::Path;
use std::process;

//const TOTAL_LINES:u32 = 20_000_000u32; //total will generate 2M records

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();

    let me = args().nth(0).unwrap();

    let pathname = args().nth(1).expect("missing path argument");

    let file_acmount = match args().nth(2) {
        Some(amount) => match amount.parse::<i64>() {
            Ok(num) => num,
            Err(_) => {
                println!("async <path name> <file amounts>");
                process::exit(1);
            }
        },
        None => {
            println!("async <path name> <file amounts>");
            process::exit(1);
        }
    };

    let total_lines = match args().nth(3) {
        Some(amount) => match amount.parse::<i64>() {
            Ok(num) => num,
            Err(_) => {
                println!("async <path name> <file amounts> <total_lines");
                process::exit(1);
            }
        },
        None => 2_000_000i64,
    };

    let path = Path::new(&pathname);
    if path.exists() && path.is_dir() {
        println!("Path should not exist since this program will create and remove automatically.");
        process::exit(1);
    }

    fs::create_dir_all(&path)?;
    let lines_per_file = total_lines / file_acmount;
    for index in 0..file_acmount {
        let file_path = Path::new(&path).join(format!("test{}.csv", index));
        let file = File::create(&file_path)?;
        let mut file = BufWriter::new(file);
        for row in 0..lines_per_file {
            file.write_all(
                format!("{},{},{}\n", index, row, Utc::now().timestamp_nanos()).as_bytes(),
            )?;
        }
        file.flush()?;
    }

    let parent = Path::new(&me).parent().unwrap();

    for index in 0..=9 {
        let command_name = format!("test{:0>2}", index);
        let command_path = Path::new(parent).join(&command_name);

        let command = command_path.to_str().unwrap().to_string();

        let local_pathname = pathname.clone();
        //println!("{} {}", &command, &local_pathname);

        match Command::new(command).arg(local_pathname).output() {
            Ok(output) => {
                println!(
                    "{},{},{}",
                    command_name,
                    output.status.code().unwrap(),
                    std::str::from_utf8(&output.stdout).unwrap().trim()
                );
            }
            Err(e) => println!("{:?}", e),
        };
    }

    fs::remove_dir_all(&path)?;
    println!("total time:{}", now.elapsed().as_millis());
    Ok(())
}
