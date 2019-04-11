
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

use crate::global_constants::{SUPERBLOCK_MAGIC, BLOCK_SIZE};

use core::mem::size_of;

pub struct Device {
    device: mut *u32,
    superblock: SuperBlock,
    group_descriptor_table: GroupDescriptor,
}

pub struct SuperBlock {
    // Number of inodes
    s_inodes_count: u32,
    // Number of blocks
    s_blocks_count: u32,
    // Number of reserved blocks
    s_r_blocks_count: u32,
    // Number of free blocks
    s_free_blocks_count: u32,
    // Number of free inodes
    s_free_inodes_count: u32,
    // First data block
    s_first_data_block: u32,
    // Block size
    s_log_block_size: u32,
    // Fragment size
    s_log_fragment_size: u32,
    // Number of blocks per group
    s_blocks_per_group: u32,
    // Number of fragments per group
    s_frags_per_group: u32,
    // Number of inodes per group
    s_inodes_per_group: u32,
    // Mount time
    s_mtime: u32,
    // Write time
    s_wtime: u32,
    // Mount count
    s_mnt_count: u16,
    // Maximal mount count
    s_max_mnt_count: u16,
    // Magic signature
    s_magic: u32,
}

pub struct GroupDescriptor {
    // Block id for first block of block bitmap
    bg_block_bitmap: u32,
    // Block id for first block of inode bitmap
    bg_inode_bitmap: u32,
    // Block id for first block of inode table
    bg_inode_table: u32,
    // Total number of free blocks
    bg_free_blocks_count: u16,
    // Total number of free inodes
    bg_free_inodes_count: u16,
    // Number of inodes allocated to directories
    bg_used_dirs_count: u16,
}

pub struct Inode {
    // Indicates format of file and access rights
    i_mode: u16,
    // User id associated with file
    i_uid: u16,
    // Size of the file bytes
    i_size: u32,
    // Number of seconds since Jan 1st 1970 of the last time this was accessed
    i_atime: u32,
    // Number of seconds since Jan 1st 1970 of when this was created
    i_ctime: u32,
    // Number of seconds since Jan 1st 1970 of the last time this inode was modified
    i_mtime: u32,
    // Number of seconds since Jan 1st 1970 of when this inode was deleted
    i_dtime: u32,
    // POSIX group which has access to this file
    i_gid: u16,
    // How many times this inode is linked (referred to)
    i_links_count: u16,
    // Total number of blocks reserved to contain the data of this inode
    i_blocks: u32,
    // Indicates how ext2 implementation should behave for this inode's data
    i_flags: u32,
    // OS dependent value
    i_osd1: u32,
    // Block numbers containing the data for this inode
    // The first 12 blocks are direct blocks, 
    i_block: [u32: 15],
    i_generation: u32,
    i_file_acl: u32,
    i_dir_acl: u32,
    i_faddr: u32,
}

impl Device {
    pub fn read_superblock(&mut self) -> Result<(), Error> {
        // Skip over boot table???
        // Read in the superblock
        //read(&self.superblock, size_of::<SuperBlock>(), 1, file_descriptor);
        // Check magic signature
        if self.superblock.magic != SUPERBLOCK_MAGIC {
            return Error();
        }
        return Ok();
    }

    pub fn read_group_descriptor_table(&mut self) -> Result<(), Error> {
        let gd_count = self.superblock.s_blocks_count / self.superblock.s_blocks_per_group + 1;

        let position = (self.superblock.s_first_data_block + 1) * BLOCK_SIZE;

    }

    pub fn read_inode(&mut self, inode_number: u32) {
        let group_number: u32 = (inode_number - 1) / self.superblock.s_inodes_per_group;
        let inode_local_number: u32 = (inode_num - 1) % self.superblock.s_inodes_per_group;
        let inode_table_block: u32 = self.group_descriptor_table.bg_inode_table;

    }
}