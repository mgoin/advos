
/*
How To Read An Inode
1. Read the Superblock to find the size of each block, the number of blocks per group, number Inodes per group, and the starting block of the first group (Block Group Descriptor Table).
2. Determine which block group the inode belongs to.
3. Read the Block Group Descriptor corresponding to the Block Group which contains the inode to be looked up.
4. From the Block Group Descriptor, extract the location of the block group's inode table.
5. Determine the index of the inode in the inode table.
6. Index the inode table (taking into account non-standard inode size).
7. Directory entry information and file contents are located within the data blocks that the Inode points to.

How To Read the Root Directory
The root directory's inode is defined to always be 2. Read/parse the contents of inode 2.
*/

use crate::global_constants::SUPERBLOCK_MAGIC;

pub struct SuperBlock {
    // Number of inodes
    inodes_count: usize,
    // Number of blocks
    blocks_count: usize,
    // Number of reserved blocks
    reserved_blocks_count: usize,
    // Number of free blocks
    free_blocks_count: usize,
    // Number of free inodes
    free_inodes_count: usize,
    // First data block
    first_data_block: usize,
    // Number of blocks per group
    blocks_per_group: usize,
    // Number of fragments per group
    frags_per_group: usize,
    // Number of inodes per group
    inodes_per_group: usize,
    // Magic signature
    magic: u32,
    // Location of first inode
    first_inode: usize,
    // Block size
    block_size: usize,
    // Fragment size
    fragment_size: usize,
    // Inode size
    inode_size: usize,
}

pub struct Group {
    // Block bitmap block
    block_bitmap: usize,
    // Inode bitmap block
    inode_bitmap: usize,
    // Inode table block
    inode_table: usize,
    // Free block count
    free_blocks_count: usize,
    // Free inode count
    free_inodes_count: usize,
}

pub struct Inode {
    // Size in bytes
    size: usize,
    // Group id
    group_id: usize,
    // Link count
    links_count: usize,
    // Block count
    blocks_count: usize,
    // Pointers to the blocks that contain the data the 
    // inode is describing. The first 12 are pointers to the physical blocks
    // and the last 3 pointers contain more and more levels of indirection
    data_blocks: [u32: 15],
}

pub fn read_superblock(sb: &mut SuperBlock) -> Result<(), Error> {
    // Skip over boot table???
    // Read in the superblock
    // Check magic signature
    if sb.magic != SUPERBLOCK_MAGIC {
        return Error();
    }
    return Ok();
}