<<<<<<< HEAD
// Michael Goin, Jacob Rutherford, Jiajia Zhao, Jonathan Ambrose
//
// 1-25-2019
//
// The purpose of this lab is to implement a Stack-allocated vector,
// including the ability to see the buffer size, the number of elements
// in use, the ability to allocate a new StackVec, and the ability to push
// and pop elements.
// Furthermore, an iterator was to be implemented that allowed one to
// directly iterate over a StackVec.
// Main was also implemented that allowed a user to interact with the data
// structure for testing.
=======
/* Michael Goin, Jacob Rutherford, Jiajia Zhao, Jonathan Ambrose
 *
 * 1-25-2019
 *
 * The purpose of this lab is to implement a Stack-allocated vector,
 * including the ability to see the buffer size, the number of elements
 * in use, the ability to allocate a new StackVec, and the ability to push
 * and pop elements.
 * Furthermore, an iterator was to be implemented that allowed one to
 * directly iterate over a StackVec.
 * Main was also implemented that allowed a user to interact with the data
 * structure for testing.
 */
>>>>>>> 69590b1fe164f8de8a2570e5fe3a8f0e6c888b70

pub struct StackVec<'a, T: 'a> {
    buffer: &'a mut [T], // Reference to the storage array
    size: usize,         // Total number of used elements in the vector
}

pub struct StackVecIterator<'a, T: 'a> {
    vector: &'a StackVec<'a, T>, // The vector to iterate across.
    location: usize,             // The element the iterator is currently on.
}

impl<'a, T: 'a> StackVec<'a, T> {
<<<<<<< HEAD
    // Returns a new vector that uses the given storage as storage
=======
    //Returns a new vector that uses the given storage as storage
>>>>>>> 69590b1fe164f8de8a2570e5fe3a8f0e6c888b70
    pub fn new(t: &'a mut [T]) -> StackVec<'a, T> {
        StackVec { buffer: &mut *t,
                   size: 0 }
    }

    // Returns the number of elements in the vector
    pub fn size(&self) -> usize {
        self.size
    }

    // Returns the maximum number of elements allowed in the vector
    pub fn buffer_size(&self) -> usize {
        self.buffer.len()
    }

    // Pushes |data| onto the bottom of the vector and increases the number of
    // elements in the vector by 1.
    // Returns Ok if it can do this or Err if there isn't enough room
    pub fn push(&mut self, data: T) -> Result<(), ()> {
        if self.buffer_size() <= self.size() {
            return Err(());
        }
        self.buffer[self.size] = data;
        self.size += 1;
        Ok(())
    }

    // Pops the top of the vector and returns a reference to that element wrapped
    // in Ok. If there are none to pop, this returns Err.
    pub fn pop(&mut self) -> Result<&mut T, ()> {
        if self.size() == 0 {
            return Err(());
        }
        let data = &mut self.buffer[self.size - 1];
        self.size -= 1;
        Ok(data)
    }

    // Returns the top of the vector as an iterator
    pub fn iter(&'a self) -> StackVecIterator<'a, T> {
        StackVecIterator { vector: &self,
                           location: 0 }
    }
}

// Implementation of the Index trait for a StackVec
// Simply returns an immutable reference to the item at index
impl<'a, T: 'a> core::ops::Index<usize> for StackVec<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        if index >= self.size() {
            panic!("StackVec: index out of bounds");
        }
        &self.buffer[index]
    }
}

// Implementation of the IndexMut trait for StackVec
// Simply returns a mutable reference to the item at index
impl<'a, T: 'a> core::ops::IndexMut<usize> for StackVec<'a, T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        if index >= self.size() {
            panic!("StackVec: index out of bounds");
        }
        &mut self.buffer[index]
    }
}

// Implementation of the Iterator trait for our StackVecIterator
impl<'a, T: 'a> Iterator for StackVecIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.location < self.vector.size() {
            let ret = &self.vector[self.location];
            self.location += 1;
            Some(&ret)
        } else {
            None
        }
    }
}

#[macro_export]
macro_rules! stackvec {
    // Make empty vector from storage
    ( $storage:expr ) => { StackVec::new($storage) };

    // Make vector from storage and initialize with variable number of elements
    ( $storage:expr, $( $x:expr ),+ ) => {
        {
            let mut temp_vec = StackVec::new($storage);
            // Push back all of the elements
            $(
                if !temp_vec.push($x).is_ok() {
                    panic!("Not enough storage for initial elements.");
                }
            )+
            temp_vec
        }
    };
}

<<<<<<< HEAD
// Example Usage:
//
// fn main() {
// let mut array: [f64; 5] = [0.0; 5];
// let mut sv = stackvec!(&mut array);
//
// Start command system loop for user
// 'command: loop {
// Get user input
// print!("Enter a command ('quit' to quit): ");
// io::stdout().flush().unwrap();
// let mut input = String::new();
// io::stdin().read_line(&mut input).unwrap();
//
// let command = scan_fmt!(&input, "{}", String);
// if command == None {
// println!("Invalid command.");
// continue 'command;
// }
//
// match command.unwrap().as_ref() {
// "get" => {
// Extract value from input
// let index = scan_fmt!(&input, "get {}", usize).unwrap();
// Check if index is valid and get value
// if index >= sv.size() {
// println!("Invalid index #{}", index);
// } else {
// println!("Value at {} = {:.04}", index, sv[index]);
// }
// },
// "set" => {
// Extract value from input
// let (i, v) = scan_fmt!(&input, "set {} {}", usize, f64);
// if i == None || v == None {
// println!("Invalid command. (expected: set index value)");
// continue 'command;
// }
// let (i, v) = (i.unwrap(), v.unwrap());
// Check if index is valid and set value
// if i >= sv.size() {
// println!("Invalid index #{}", i);
// } else {
// sv[i] = v;
// println!("Value at {} = {:.04}", i, sv[i]);
// }
// },
// "print" => {
// if sv.size() == 0 {
// println!("Vector is empty.");
// } else {
// Print all elements of the vector
// let mut counter = 0;
// for i in sv.iter() {
// println!("[{:03}] = {:.04}", counter, i);
// counter += 1;
// }
// }
// },
// "push" => {
// Extract value from input
// let value = scan_fmt!(&input, "push {}", f64);
// if value == None {
// println!("Invalid command. (expected: push value)");
// continue 'command;
// }
// let value = value.unwrap();
// Check if value can be pushed back and push back
// if sv.push(value).is_ok() {
// println!("Pushed back {:.04}", value);
// } else {
// println!("Vector is full.");
// }
// },
// "pop" => {
// Check if top of vector could be popped and pop
// if let Ok(value) = sv.pop() {
// println!("Popped {:.04}", value);
// } else {
// println!("Vector is empty.");
// }
// },
// "quit" => break 'command,
// _ => println!("Unrecognized command."),
// }
// }
// }
=======
/*
 * Example Usage:
 *
fn main() {
    let mut array: [f64; 5] = [0.0; 5];
    let mut sv = stackvec!(&mut array);

    // Start command system loop for user
    'command: loop {
        // Get user input
        print!("Enter a command ('quit' to quit): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = scan_fmt!(&input, "{}", String);
        if command == None {
            println!("Invalid command.");
            continue 'command;
        }

        match command.unwrap().as_ref() {
            "get" => {
                // Extract value from input
                let index = scan_fmt!(&input, "get {}", usize).unwrap();
                // Check if index is valid and get value
                if index >= sv.size() {
                    println!("Invalid index #{}", index);
                } else {
                    println!("Value at {} = {:.04}", index, sv[index]);
                }
            },
            "set" => {
                // Extract value from input
                let (i, v) = scan_fmt!(&input, "set {} {}", usize, f64);
                if i == None || v == None {
                    println!("Invalid command. (expected: set index value)");
                    continue 'command;
                }
                let (i, v) = (i.unwrap(), v.unwrap());
                // Check if index is valid and set value
                if i >= sv.size() {
                    println!("Invalid index #{}", i);
                } else {
                    sv[i] = v;
                    println!("Value at {} = {:.04}", i, sv[i]);
                }
            },
            "print" => {
                if sv.size() == 0 {
                    println!("Vector is empty.");
                } else {
                    // Print all elements of the vector
                    let mut counter = 0;
                    for i in sv.iter() {
                        println!("[{:03}] = {:.04}", counter, i);
                        counter += 1;
                    }
                }
            },
            "push" => {
                // Extract value from input
                let value = scan_fmt!(&input, "push {}", f64);
                if value == None {
                    println!("Invalid command. (expected: push value)");
                    continue 'command;
                }
                let value = value.unwrap();
                // Check if value can be pushed back and push back
                if sv.push(value).is_ok() {
                    println!("Pushed back {:.04}", value);
                } else {
                    println!("Vector is full.");
                }
            },
            "pop" => {
                // Check if top of vector could be popped and pop
                if let Ok(value) = sv.pop() {
                    println!("Popped {:.04}", value);
                } else {
                    println!("Vector is empty.");
                }
            },
            "quit" => break 'command,
            _ => println!("Unrecognized command."),
        }
    }
}
*/
>>>>>>> 69590b1fe164f8de8a2570e5fe3a8f0e6c888b70
