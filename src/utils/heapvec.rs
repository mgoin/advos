use crate::memman::MemManager;

pub struct HeapVec<T> {
    buffer: *mut T,
    size: usize,
}

pub struct HeapVecIterator<T> {
    vec: HeapVec<T>,
    location: usize,
}

impl<T> HeapVec<T> {
    pub fn new(items: usize) -> HeapVec<T> {
        HeapVec { 
          buffer: MemManager::kmalloc(items * core::mem::size_of<T>()),
          size: 0,
        };
    }
}

impl<T> Drop for HeapVec<T> {
    fn drop(&mut self) {
      MemManager::kfree(self.buffer);
    }
}
