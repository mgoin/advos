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

use crate::global_constants::{SUPERBLOCK_MAGIC, BG_DESC_SIZE, INODE_SIZE};

use core::ptr::{read_volatile};
use core::ptr::null_mut;
use crate::console::Console;
use core::fmt::Write;
use crate::{print, println};

//Filesystem Label from Assembly
extern "C" {
    static FILE_SYSTEM: u8;
}

pub struct Device {
    device: *const u8,
    pub superblock: SuperBlock,
    pub group_descriptor_table: GroupDescriptor,
    block_size: u32,
}

#[repr(C)]
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

#[repr(C)]
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

#[repr(C)]
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
    i_block: [u32; 15],
    i_generation: u32,
    i_file_acl: u32,
    i_dir_acl: u32,
    i_faddr: u32,
}

impl Device {
    //Create an empty Device
    pub fn new() -> Device {
        Device {
            device: null_mut(),
            superblock: SuperBlock::new(),
            group_descriptor_table: GroupDescriptor::new(),
            block_size: 0,
        }
    }

    //Read in the Superblock appropriately
    pub fn read_superblock(&mut self) -> Result<(), ()> {

        //Read in the Superblock
        unsafe {
            self.device = &FILE_SYSTEM as *const u8;
            self.superblock = read_volatile(self.device.add(1024) as *const SuperBlock) as SuperBlock;
        }

        // Check magic signature
        if self.superblock.s_magic != SUPERBLOCK_MAGIC {
            return Err(());
        }

        //Store the actual block size as part of the Device
        self.block_size = (1024 as u32).wrapping_shl(self.superblock.s_log_block_size);
        return Ok(());
    }

    //Reads in the current relative GroupDescriptor
    pub fn read_group_descriptor_table(&mut self, group_number: u32) -> Result<(), ()> {
        let gd_count = self.superblock.s_blocks_count / self.superblock.s_blocks_per_group + 1;

        //Check if the group_number is valid
        if group_number > gd_count {
            return Err(());
        }

        //Read in the descriptor
        unsafe {
            self.group_descriptor_table = read_volatile(self.device.add(2048 + group_number as usize * BG_DESC_SIZE) as *const GroupDescriptor) as GroupDescriptor;
        }

        return Ok(())
    }

    //Loads an Inode in
    pub fn load_inode(&mut self, inode_number: u32) -> Inode {
        let group_number: u32 = (inode_number - 1) / self.superblock.s_inodes_per_group;
        let inode_local_number: u32 = (inode_number - 1) % self.superblock.s_inodes_per_group;

        //Read in the group descriptor
        match self.read_group_descriptor_table(group_number) {
            Ok(()) => (),
            Err(()) => panic!("Bad Group Descriptor"),
        }
        let inode_table_block: u32 = self.group_descriptor_table.bg_inode_table;

        //Read in an inode
        let cur_inode: Inode;
        unsafe {
            cur_inode = read_volatile(self.device.add(inode_table_block as usize * self.block_size as usize + inode_local_number as usize * INODE_SIZE) as *const Inode) as Inode;
        }

        return cur_inode;
    }

    //Read and print the data from an inode
    pub fn read_inode(&mut self, inode_number: u32) {
        let cur_inode = self.load_inode(inode_number);
        //let arr = self.load_inode(inode_number);

        let mut index = 0 as usize;
        let mut block_number = cur_inode.i_block[index] as usize;
        let mut size = cur_inode.i_size;

        while block_number != 0 && size != 0 {
            let block: *const u8;
            unsafe {
                block = self.device.add(self.block_size as usize * block_number) as *const u8;
            }
            //Handle direct blocks... directly
            if index < 12 {
                size = self.print_block(block, size);
            }
            //Handle singly indirect blocks
            else if index == 12 {
                size = self.single_indirect_read(block as *const u32, size);
            }
            //Handle doubly indirect blocks
            else if index == 13 {
                size = self.double_indirect_read(block as *const u32, size);
            }
            //Handle triply indirect blocks
            else {
                size = self.triple_indirect_read(block as *const u32, size);
            }
            index = index + 1;
            if index == 15 {
                break;
            }
            block_number = cur_inode.i_block[index] as usize;
        }
    }

    //Print the contents of a block, but don't read too much
    pub fn print_block(&mut self, block: *const u8, mut size: u32) -> u32 {
        unsafe {
            for i in 0..self.block_size {
                print!("{}", {read_volatile(block.add(i as usize) as *const u8) as char});
                size = size - 1;
                if size == 0 {
                    break;
                }
            }
        }

        return size;
    }

    //Reads a singly indirect inode block
    pub fn single_indirect_read(&mut self, block: *const u32, mut size: u32) -> u32 {
        let mut index = 0 as usize;
        unsafe {
            let mut block_number = read_volatile(block.add(index) as *const u32) as usize;

            while block_number != 0 && size != 0 {
                let b = self.device.add(self.block_size as usize * block_number) as *const u8;
                size = self.print_block(b, size);

                index = index + 1;
                if index as u32 >= self.block_size / 32 {
                    break;
                }
                block_number = read_volatile(block.add(index) as *const u32) as usize;
            }
        }

        return size;
    }

    //Reads a doubly indirect inode block
    pub fn double_indirect_read(&mut self, block: *const u32, mut size: u32) -> u32 {
        let mut index = 0 as usize;
        unsafe {
            let mut block_number = read_volatile(block.add(index * 4) as *const u32) as usize;

            while block_number != 0 && size != 0 {
                let b = self.device.add(self.block_size as usize * block_number) as *const u32;
                size = self.single_indirect_read(b, size);

                index = index + 1;
                if index as u32 >= self.block_size / 32 {
                    break;
                }
                block_number = read_volatile(block.add(index) as *const u32) as usize;
            }
        }

        return size;
    }

    //Reads a triply indirect inode block
    pub fn triple_indirect_read(&mut self, block: *const u32, mut size: u32) -> u32 {
        let mut index = 0 as usize;
        unsafe {
            let mut block_number = read_volatile(block.add(index * 4) as *const u32) as usize;

            while block_number != 0 && size != 0 {
                let b = self.device.add(self.block_size as usize * block_number) as *const u32;
                size = self.double_indirect_read(b, size);

                index = index + 1;
                if index as u32 >= self.block_size / 32 {
                    break;
                }
                block_number = read_volatile(block.add(index) as *const u32) as usize;
            }
        }

        return size;
    }
}

impl SuperBlock {
    //Create an empty Superblock
    pub fn new() -> SuperBlock {
        SuperBlock {
            s_inodes_count: 0,
            s_blocks_count: 0,
            s_r_blocks_count: 0,
            s_free_blocks_count: 0,
            s_free_inodes_count: 0,
            s_first_data_block: 0,
            s_log_block_size: 0,
            s_log_fragment_size: 0,
            s_blocks_per_group: 0,
            s_frags_per_group: 0,
            s_inodes_per_group: 0,
            s_mtime: 0,
            s_wtime: 0,
            s_mnt_count: 0,
            s_max_mnt_count: 0,
            s_magic: 0,
        }
    }

    //Print some of the various Superblock fields for testing
    pub fn print(&mut self) {
        println!("Number of Inodes is {}", {self.s_inodes_count});
        println!("Number of Blocks is {}", {self.s_blocks_count});
        println!("Number of Free Inodes is {}", {self.s_free_inodes_count});
        println!("Number of Free Blocks is {}", {self.s_free_blocks_count});
        println!("First Data Block is {}", {self.s_first_data_block});
        println!("Log Block size is {}", {self.s_log_block_size});
        println!("Log Fragment size is {}", {self.s_log_fragment_size});
        println!("Number of Blocks per Group is {}", {self.s_blocks_per_group});
        println!("Number of Frags per Group is {}", {self.s_frags_per_group});
        println!("Number of Inodes per Group is {}", {self.s_inodes_per_group});
    }
}

impl GroupDescriptor {
    //Makes an empty GroupDescriptor
    pub fn new() -> GroupDescriptor {
        GroupDescriptor {
            bg_block_bitmap: 0,
            bg_inode_bitmap: 0,
            bg_inode_table: 0,
            bg_free_blocks_count: 0,
            bg_free_inodes_count: 0,
            bg_used_dirs_count: 0,
        }
    }
}

impl Inode {
    //Makes an empty Inode
    pub fn new() -> Inode {
        Inode {
            i_mode: 0,
            i_uid: 0,
            i_size: 0,
            i_atime: 0,
            i_ctime: 0,
            i_mtime: 0,
            i_dtime: 0,
            i_gid: 0,
            i_links_count: 0,
            i_blocks: 0,
            i_flags: 0,
            i_osd1: 0,
            i_block: [0; 15],
            i_generation: 0,
            i_file_acl: 0,
            i_dir_acl: 0,
            i_faddr: 0,
        }
    }
}
