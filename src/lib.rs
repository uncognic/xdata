use std::fs::File;
use uuid::Uuid;
use metrohash::MetroHash64;
use std::io::{Write, Seek, SeekFrom, Read};
use std::hash::Hasher;
const BLOCK_SIZE: u64 = 4096;
pub fn mkfs(filename: &str)
{
    let mut file = File::create(filename).expect("failed to create img");

    let magic: u64 = 0x54414458; // "TADX" -> XDAT
    let version: u32 = 1;
    let blktot: u64 = 8;
    let metastart: u64 = 2;
    let metacount: u64 = 1;
    let uuid: [u8; 16] = *Uuid::new_v4().as_bytes();
    let mut mhash: u64 = 0;
    let bmpstart: u64 = 1;
    const SUPERBLOCK: u64 = 0;

    let mut buf = [0u8; 4096];
    buf[0..8].copy_from_slice(&magic.to_le_bytes());
    buf[8..12].copy_from_slice(&version.to_le_bytes());
    buf[12..20].copy_from_slice(&blktot.to_le_bytes());
    buf[20..28].copy_from_slice(&metastart.to_le_bytes());
    buf[28..36].copy_from_slice(&metacount.to_le_bytes());
    buf[36..52].copy_from_slice(&uuid);
    buf[52..60].copy_from_slice(&mhash.to_le_bytes());
    buf[60..68].copy_from_slice(&bmpstart.to_le_bytes());

    {
        let mut tmp = buf;
        tmp[52..60].copy_from_slice(&0u64.to_le_bytes());
        let mut hasher = MetroHash64::new();
        hasher.write(&tmp[..68]);
        mhash = hasher.finish();
        buf[52..60].copy_from_slice(&mhash.to_le_bytes());
    }

    file.seek(SeekFrom::Start(SUPERBLOCK)).unwrap();
    file.write_all(&buf).unwrap();
    write_bitmap(&mut file, blktot as usize, bmpstart);
}
pub fn write_bitmap(file: &mut File, total_blocks: usize, bitmap_start: u64)
{
    let mut bits = vec![0u8; total_blocks];

    bits[0] = 1;
    bits[1] = 1;
    bits[2] = 1;

    let mut block = vec![0u8; BLOCK_SIZE as usize];
    let copy_len = std::cmp::min(bits.len(), block.len());
    block[..copy_len].copy_from_slice(&bits[..copy_len]);

    file.seek(SeekFrom::Start(bitmap_start * BLOCK_SIZE)).unwrap();
    file.write_all(&block).unwrap();
}
pub fn alloc_bitmap(file: &mut File, total_blocks: usize, bitmap_start: u64) -> Option<usize>
{
    let mut block = vec![0u8; BLOCK_SIZE as usize];
    file.seek(SeekFrom::Start(bitmap_start * BLOCK_SIZE)).unwrap();
    file.read_exact(&mut block).unwrap();

    let mut bits = vec![0u8; total_blocks];
    let copy_len = std::cmp::min(total_blocks, block.len());
    bits[..copy_len].copy_from_slice(&block[..copy_len]);

    for (i, byte) in bits.iter_mut().enumerate() {
        if *byte == 0 {
            *byte = 1;

            block[..copy_len].copy_from_slice(&bits[..copy_len]);
            file.seek(SeekFrom::Start(bitmap_start * BLOCK_SIZE)).unwrap();
            file.write_all(&block).unwrap();
            return Some(i)
        }
    }
    None
}
pub fn free_bitmap(file: &mut File, total_blocks: usize, bitmap_start: u64, blk: usize) -> Option<usize>
{
    let mut block = vec![0u8; BLOCK_SIZE as usize];
    file.seek(SeekFrom::Start(bitmap_start * BLOCK_SIZE)).unwrap();
    file.read_exact(&mut block).unwrap();

    let copy_len = std::cmp::min(total_blocks, block.len());
    let mut bits = vec![0u8; total_blocks];
    bits[..copy_len].copy_from_slice(&block[..copy_len]);

    if blk < total_blocks {
        bits[blk] = 0;
        block[..copy_len].copy_from_slice(&bits[..copy_len]);
        file.seek(SeekFrom::Start(bitmap_start * BLOCK_SIZE)).unwrap();
        file.write_all(&block).unwrap();
        return Some(blk)
    }
    None

}
#[derive(Debug)] 
pub struct FileStruct {
    pub name: String,
    pub size: usize,
    pub blkoffset: usize,
    pub blkend: usize,
    pub createtime: i64,
    pub modifytime: i64,
    pub readtime: i64
}
#[derive(Debug)]
pub struct DirStruct {
    pub name: String,
    pub offset: usize,
    pub entries: Vec<FileStruct>
}