use std::fs::File;
use std::io::{Write, Seek, SeekFrom};
use uuid::Uuid;
fn mkfs(filename: &str) {
    let mut file = File::create(filename).expect("failed to create img");

    let magic: u64 = 0x54414458; // "TADX" -> XDAT
    let version: u32 = 1;
    let blktot: u64 = 1;
    let metastart: u64 = 1;
    let metact: u64 = 0;
    let uuid: [u8; 16] = *Uuid::new_v4().as_bytes();
    let sha512: [u8; 64] = [0u8; 64];

    let mut buf = [0u8; 4096];
    buf[0..8].copy_from_slice(&magic.to_le_bytes());
    buf[8..12].copy_from_slice(&version.to_le_bytes());
    buf[12..20].copy_from_slice(&blktot.to_le_bytes());
    buf[20..28].copy_from_slice(&metastart.to_le_bytes());
    buf[28..36].copy_from_slice(&metact.to_le_bytes());
    buf[36..52].copy_from_slice(&uuid);
    buf[52..116].copy_from_slice(&sha512);

    file.seek(SeekFrom::Start(0)).unwrap();
    file.write_all(&buf).unwrap();
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("using default disk.img");
        mkfs("disk.img");
        std::process::exit(0);
    }
    mkfs(&args[1]);
}
