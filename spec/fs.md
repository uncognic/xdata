# XData spec
## Block size
Block size is 4096.
## Block structure
Block 0: Superblock, 4096B\
Block 1: Bitmap (size = number of blocks (rounds up or down to the nearest 4096) 1 for allocated 0 for free)\
Block 2: Metadata block\
Rest: Data\
Last: Copy of Block 0, 1, 2
### Superblock structure
Contains global partition metadata.\
\
offset 0 - size 8 - magic - u64 0x54414458 ;TADX but XDAT because of endianness\
offset 8 - size 4 - version - u32\
offset 12 - size 8 - blktot - u64\
offset 20 - size 8 - metastart - u64\
offset 28 - size 8 - metact - u64\
offset 36 - size 16 - uuid - u8[16]\
offset 52 - size 8 - xxh3 - u64\
offset 60 - size 8 -  bmpstart - u64 = 1
### Bitmap structure
Bitmap is at block 1. Bits 0, 1, and 2 are always set to 01. (Superblock, Bitmap block and Metadata block)\
\
offset 0 - size 8 - bmpmagic - u64 0x504D5442 ;PMTB but BTMP because of endianness\
### Metadata block structure
128B per file. Can store metadata for 32 files in one block. 


