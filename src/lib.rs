    use std::fs::File;
    use uuid::Uuid;
    use std::io::{Write, Seek, SeekFrom, Read};
    const BLOCK_SIZE: u64 = 4096;
    pub fn mkfs(filename: &str)
    {
        let mut file = File::create(filename).expect("failed to create img");

        let magic: u64 = 0x54414458; // "TADX" -> XDAT
        let version: u32 = 1;
        let blktot: u64 = 8;
        let metastart: u64 = 2;
        let metact: u64 = 0;
        let uuid: [u8; 16] = *Uuid::new_v4().as_bytes();
        let sha512: [u8; 64] = [0u8; 64];
        let bmpstart: u64 = 1;
        const SUPERBLOCK: u64 = 0;

        let mut buf = [0u8; 4096];
        buf[0..8].copy_from_slice(&magic.to_le_bytes());
        buf[8..12].copy_from_slice(&version.to_le_bytes());
        buf[12..20].copy_from_slice(&blktot.to_le_bytes());
        buf[20..28].copy_from_slice(&metastart.to_le_bytes());
        buf[28..36].copy_from_slice(&metact.to_le_bytes());
        buf[36..52].copy_from_slice(&uuid);
        buf[52..116].copy_from_slice(&sha512);

        file.seek(SeekFrom::Start(SUPERBLOCK)).unwrap();
        file.write_all(&buf).unwrap();
        write_bitmap(&mut file, blktot as usize, bmpstart);
    }
    pub fn write_bitmap(file: &mut File, total_blocks: usize, bitmap_start: u64)
    {
        let mut bits = vec![0u8; total_blocks];

        bits[0] = 1;
        bits[1] = 1;

        file.seek(SeekFrom::Start(bitmap_start * BLOCK_SIZE)).unwrap();
        file.write_all(&bits).unwrap();
    }
    pub fn alloc_bitmap(file: &mut File, total_blocks: usize, bitmap_start: u64) -> Option<usize>
    {
        let mut bits = vec![0u8; total_blocks];
        file.seek(SeekFrom::Start(bitmap_start * BLOCK_SIZE)).unwrap();
        file.read_exact(&mut bits).unwrap();

        for (i, byte) in bits.iter_mut().enumerate() {
            if *byte == 0 {
                *byte = 1;

                file.seek(SeekFrom::Start(bitmap_start * BLOCK_SIZE)).unwrap();
                file.write_all(&bits).unwrap();
                return Some(i)
            }
        }
        None


    }
    pub fn free_bitmap(file: &mut File, total_blocks: usize, bitmap_start: u64, blk: usize) -> Option<usize>
    {
        let mut bits = vec![0u8; total_blocks];
        file.seek(SeekFrom::Start(bitmap_start * BLOCK_SIZE)).unwrap();
        file.read_exact(&mut bits).unwrap();
        if blk < total_blocks {
            bits[blk] = 0;
            file.seek(SeekFrom::Start(bitmap_start * BLOCK_SIZE)).unwrap();
            file.write_all(&bits).unwrap();
            return Some(blk)
        }
        None

    }