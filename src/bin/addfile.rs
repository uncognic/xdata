use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::hash::Hasher;
use std::time::{SystemTime, UNIX_EPOCH};
use metrohash::MetroHash64;
use xdata::{alloc_bitmap, free_bitmap, FileStruct};

const BLOCK_SIZE: u64 = 4096;

fn u64_from_le_bytes(bytes: &[u8]) -> u64 {
    let mut array = [0u8; 8];
    array.copy_from_slice(&bytes[0..8]);
    u64::from_le_bytes(array)
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: addfile <disk.img> <datafile>");
        std::process::exit(1);
    }
    let disk = &args[1];
    let src = &args[2];

    let mut disk = OpenOptions::new()
        .read(true)
        .write(true)
        .open(disk)
        .expect("failed to open disk image");

    let mut superblock = [0u8; BLOCK_SIZE as usize];
    disk.seek(SeekFrom::Start(0)).unwrap();
    disk.read_exact(&mut superblock).unwrap();
    let total_blocks = u64_from_le_bytes(&superblock[12..20]) as usize;
    let bitmap_start = u64_from_le_bytes(&superblock[60..68]);

    let mut src = File::open(src).expect("failed to open source file");
    let mut data = Vec::new();
    src.read_to_end(&mut data).expect("failed to read source file");

    let blocks_needed = ((data.len() as u64 + BLOCK_SIZE - 1) / BLOCK_SIZE) as usize;
    let mut hasher = MetroHash64::new();

    let mut allocated_blocks = Vec::new();

    for i in 0..blocks_needed {
        match alloc_bitmap(&mut disk, total_blocks, bitmap_start) {
            Some(blk) => {
                allocated_blocks.push(blk);
                let off = (blk as u64) * BLOCK_SIZE;
                disk.seek(SeekFrom::Start(off)).unwrap();
                let start = i * BLOCK_SIZE as usize;
                let end = ((i + 1) * BLOCK_SIZE as usize).min(data.len());
                let mut block = vec![0u8; BLOCK_SIZE as usize];

                if start < end {
                    block[..(end - start)].copy_from_slice(&data[start..end]);
                }
                hasher.write(&block);
                disk.write_all(&block).unwrap();
            }
            None => {
                eprintln!("out of space: freeing allocated blocks");
                for &b in &allocated_blocks {
                    let _ = free_bitmap(&mut disk, total_blocks, bitmap_start, b);
                }
                std::process::exit(2);
            }
        }
    }

    if blocks_needed == 0 {
        hasher.write(&[]);
    }

    let data_hash = hasher.finish();

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
    let file_struct = FileStruct {
        name: std::path::Path::new(&args[2]).file_name().and_then(|s| s.to_str()).unwrap_or(&args[2]).to_string(),
        size: data.len(),
        blkoffset: *allocated_blocks.first().unwrap_or(&0),
        blkend: *allocated_blocks.last().unwrap_or(&0),
        createtime: now,
        modifytime: now,
        readtime: now,
    };

    println!("wrote {} bytes in {} blocks: {:?}", data.len(), blocks_needed, allocated_blocks);
    println!("file metadata: {:?}", file_struct);
    println!("metrohash64: {:016x}", data_hash);
}