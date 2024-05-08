use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use sha2::{Sha256, Digest};
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Invalid amounts of arguments!");
        println!("Example: cargo run <sha256 hash>");
        exit(1);
    }

    let wanted_hash = &args[1];
    let password_file = "src/rockyou.txt";
    let mut attempts = 1;

    println!("Attempting to hack: {}!\n", wanted_hash);

    let password_list = File::open(password_file).unwrap();
    let reader = BufReader::new(password_list);

    for line in reader.lines() {
        let line = line.unwrap();
        let password = line.trim().to_owned();
        let password_hash = Sha256::digest(password.as_bytes());
        println!("[{}] {} == {:x}", attempts, password, password_hash);

        if format!("{:x}", password_hash) == *wanted_hash {
            println!("Password hash found after {} attempts! {} hashes to {}!", attempts, password, wanted_hash);
            exit(0);
        }
        attempts += 1;
    }

    println!("Password hash not found!");
    exit(0);
}
