use std::env;
use std::fs::File;
use std::io::Read;
use std::io::{BufWriter, Write};

fn write_to_file(bytes: &Vec<u8>, w: u64) {
    println!("Writing {} bytes", w*(bytes.len() as u64));
    let mut file = BufWriter::new(
        File::create(format!("{} MB", w))
            .expect("error creating file"));
    for _ in 0..w {
        file
            .write_all(bytes)
            .expect("Error writing bytes");
    }

    file.flush().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {    
        println!("Usage: file-duplicator <file> <count>");
        return;
    }

    let filename = &args[1];
    let filesize = args[2]
        .parse::<u64>()
        .expect("Second argument should be positive integer");
    println!("Reading file {}", filename);
    
    let mut file = File::open(filename)
        .expect("Error opening file");

    let mut bytes: Vec<u8> = Vec::new();
    file.read_to_end(&mut bytes)
        .expect("error reading file");
    write_to_file(&bytes, filesize);
}
