use std::env;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::hash::Hasher;

use metrohash::MetroHash64;

const BLOCK_SIZE: u64 = 4096;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: read <disk.img> <block_number> <output_file>");
        std::process::exit(1);
    }
    let disk_path = &args[1];
    let block_number: u64 = args[2].parse().expect("invalid block number");
    let output_file = &args[3];

    let mut disk = File::open(disk_path).expect("failed to open disk image");
    let mut out = File::create(output_file).expect("failed to create output file");

    let mut buffer = vec![0u8; BLOCK_SIZE as usize];
    disk.seek(SeekFrom::Start(block_number * BLOCK_SIZE)).unwrap();
    disk.read_exact(&mut buffer).expect("failed to read block");
    out.write_all(&buffer).expect("failed to write to output file");

    let mut hasher = MetroHash64::new();
    hasher.write(&buffer);
    let block_hash = hasher.finish();
    println!("read block {} -> metrohash64: {:016x}", block_number, block_hash);
}