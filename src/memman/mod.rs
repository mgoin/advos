// Michael Goin, Jacob Rutherford, and Jonathan Ambrose
// 2-26-2019
// This code implements the memory manager.

use crate::{HEAP_END, HEAP_START};
use core::ptr::{read_volatile, write_volatile};

pub struct Descriptor {
    len: u16,
    taken: u16,
}

pub struct MemManager;

impl MemManager {
    // Initialize by setting the first descriptor at the start of the heap
    #[no_mangle]
    pub fn init() -> () {
        unsafe {
            let desc = HEAP_START as *mut Descriptor;
            write_volatile(&mut ((*desc).len),
                           ((HEAP_END as u32) - (HEAP_START as u32)) as u16);
            write_volatile(&mut ((*desc).taken), 0);
        }
    }

    // Mallocs a block of memory. Returns a Result
    // Ok has a u32 in it upon return corresponding to
    // the address of the memory
    // Error will return a string with the error message
    #[no_mangle]
    pub fn kmalloc(sz: usize) -> Result<u32, &'static str> {
        unsafe {
            let mut start = HEAP_START as u32;
            let end = HEAP_END as u32;
            let mut pnt = 0 as u32;

            // Get the size to a multiple of 4
            let size = (sz as u32) + 3 - (sz as u32 - 1) % 4 as u32;

            // Start at heap_start
            // Go until we reach the end or break
            while start != end {
                // Make a descriptor at the start
                let desc = start as *mut Descriptor;

                // Check if we're taken
                let t = read_volatile(&((*desc).taken)) as u16;
                if t != 1 {
                    // Check if it's large enough
                    let s = read_volatile(&((*desc).len)) as u16;
                    if s as u32 >=
                       size + core::mem::size_of::<Descriptor>() as u32
                    {
                        let len = read_volatile(&mut ((*desc).len)) as u16;

                        // If it is big enough, mark as taken
                        write_volatile(&mut ((*desc).taken), 1 as u16);

                        // Split the block if the block was big enough that it
                        // would leave more than a Descriptor
                        let s2 =
                            size + core::mem::size_of::<Descriptor>() as u32;
                        if s2 != len as u32 &&
                           s2 + core::mem::size_of::<Descriptor>() as u32 !=
                           len as u32
                        {
                            write_volatile(&mut ((*desc).len),
                                           (size +
                                            core::mem::size_of::<Descriptor>()
                                            as u32)
                                           as u16);

                            let new_desc = (start +
                                            size +
                                            (core::mem::size_of::<Descriptor>()
                                             as u32))
                                           as *mut Descriptor;
                            let new_taken =
                                read_volatile(&((*new_desc).taken)) as u16;
                            if new_taken != 1 {
                                write_volatile(&mut ((*new_desc).taken),
                                               0 as u16);
                                write_volatile(&mut ((*new_desc).len),
                                               s -
                                               read_volatile(&((*desc).len))
                                               as u16);
                            }
                        }

                        // Set the pointer that we'll return
                        pnt = start + core::mem::size_of::<Descriptor>() as u32;
                        break;
                    }
                }

                // If we didn't break, we set the location of the new Descriptor
                start = start + read_volatile(&((*desc).len)) as u32;
            }

            // If the pointer isn't 0, we return it
            if pnt != 0 {
                Ok(pnt)
            }
            // Otherwise, return an error
            else {
                Err("Not enough memory")
            }
        }
    }

    // This function frees a given pointer
    // It returns an empty Ok or and Err string
    pub fn kfree(p: u32) -> Result<(), &'static str> {
        let mut badpnt = 0 as u16;
        unsafe {
            let desc = (p - core::mem::size_of::<Descriptor>() as u32)
                       as *mut Descriptor;

            // If the pointer isn't taken, it's a bad pointer
            if read_volatile(&((*desc).taken)) != 1 {
                badpnt = 1;
            } else {
                // Set the descriptor to not be taken
                write_volatile(&mut ((*desc).taken), 0 as u16);
            }
        }
        if badpnt == 0 {
            Ok(())
        } else {
            Err("Bad pointer")
        }
    }

    // This function coalesces the memory, bringing together contiguous
    // elements that are not taken
    #[no_mangle]
    pub fn kcoalesce() -> () {
        unsafe {
            let mut start = HEAP_START as u32;
            let end = HEAP_END as u32;
            let mut next: u32 = start;

            // Start at heap_start
            // Go until we reach the end or break
            while next != end {
                // Make a descriptor at the start
                let desc = start as *mut Descriptor;

                // Find where the next descriptor will be
                next = (start + read_volatile(&((*desc).len)) as u32) as u32;

                // If we aren't at the end, try to merge
                if next != end {
                    let desc2 = next as *mut Descriptor;

                    // Check if either are taken. If neither is taken, merge them
                    let t = read_volatile(&((*desc).taken)) as u16;
                    let t2 = read_volatile(&((*desc2).taken)) as u16;

                    if t != 1 && t2 != 1 {
                        write_volatile(&mut ((*desc).len),
                                       read_volatile(&((*desc).len)) +
                                       read_volatile(&((*desc2).len)));
                    } else {
                        // If we couldn't merge, move to the next element
                        start = next;
                    }
                }
            }
        }
    }
}
