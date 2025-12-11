# XData spec
## Block size
All structures (superblock, metadata blocks, data blocks) align to 4096 bytes.
## Block structure
Block 0: Superblock, 4096B\
Block 1: Bitmap (size = number of blocks (rounds up or down to the nearest 4096) 1 for allocated 0 for free)\
Block 2: Metadata block, 4096B per file\
Rest: Data\
Last: Copy of Block 0, 1, 2
### Superblock structure
offset 0 - size 8 - magic - u64 0x58444154 ;XDAT\
offset 8 - size 4 - version - u32\
offset 12 - size 8 - blktot - u64\
offset 20 - size 8 - metastart - u64\
offset 28 - size 8 - metact - u64\
offset 36 - size 16 - uuid - u8[16]\
offset 52 - size 64 - sha512 - u8[64]
### Bitmap structure
Bitmap is at block 1. Bits 0, 1, and 2 are always set to 01. (Superblock, Bitmap block and Metadata block)
### Metadata block structure


