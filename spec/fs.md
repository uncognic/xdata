# XData spec
## Block size
All structures (superblock, metadata blocks, data blocks) align to 4096 bytes.
## Block structure
Block 0: Superblock, 4096B
Block 1: Bitmap (size = number of blocks (rounds up or down to the nearest 4096) 1 for allocated 0 for free)
Block 2: Metadata block, 4096B per file
Rest: Data
Last: Backup of block 0
### Superblock structure
