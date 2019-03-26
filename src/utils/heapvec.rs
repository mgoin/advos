use crate::memman::MemManager;

use core::ptr::{read_volatile, write_volatile};

pub struct HeapVec<T> {
    buffer: *mut T,
    capacity: usize,
    size: usize,
}

pub struct HeapVecIterator<T> {
    vec: HeapVec<T>,
    location: usize,
}

impl<T> HeapVec<T> {
    pub fn new(items: usize) -> HeapVec<T> {
        HeapVec { 
          buffer: MemManager::kmalloc(items * core::mem::size_of::<T>()).unwrap() as *mut T,
          capacity: items,
          size: 0,
        }
    }

    pub fn push(&mut self, data: T) -> Result<(), ()> {
        if self.size >= self.capacity {
            return Err(());
        }

        unsafe {
            write_volatile(self.buffer.add(self.size), data);
        }

        self.size += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
         match self.size {
           0 => None,
           _ if self.size > 0 => {
             let d: T;
             unsafe {
                d = read_volatile(self.buffer.add(self.size - 1));
             }
             self.size -= 1;
             Some(d)
           },
           _ => None,
         }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl<T> Drop for HeapVec<T> {
    fn drop(&mut self) {
      MemManager::kfree(self.buffer as u32).unwrap();
    }
}

impl<T> core::ops::Index<usize> for HeapVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        if index >= self.size {
          panic!("HeapVec: index out of bounds");
        }
        unsafe { &(*self.buffer.add(index)) }
    }
}

impl<T> core::ops::IndexMut<usize> for HeapVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        if index >= self.size {
          panic!("HeapVec: index out of bounds");
        }
        unsafe { &mut(*self.buffer.add(index)) }
    }
}
