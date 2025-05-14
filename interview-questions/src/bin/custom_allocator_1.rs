/*
How do you write a wrapper around malloc so that it returns the address 
which can accommodate all addresses divisible by a certain integer.
(Suppose, I want addresses divisible by 513 only. How do I write the wrapper around malloc)
*/

use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

struct AlignedMemory {
    ptr: *mut u8,       // Original allocated pointer
    aligned_ptr: *mut u8,  // Aligned pointer
}

impl AlignedMemory {
    /// Creates a memory block aligned to any positive integer.
    fn new(size: usize, alignment: usize) -> Self {
        if alignment == 0 {
            panic!("Alignment must be a positive integer");
        }

        // Calculate extra space needed: alignment + sizeof(usize) for metadata
        let extra_space = alignment + std::mem::size_of::<usize>();
        let layout = Layout::from_size_align(size + extra_space, 1).unwrap();

        unsafe {
            // Allocate memory
            let raw_ptr = alloc(layout);
            if raw_ptr.is_null() {
                panic!("Memory allocation failed");
            }

            // Calculate the aligned address
            let addr = raw_ptr as usize;
            let aligned_addr = ((addr + alignment - 1) / alignment) * alignment;
            let aligned_ptr = aligned_addr as *mut u8;

            // Store the original pointer just before the aligned address
            let meta_ptr = (aligned_ptr as *mut usize).offset(-1);
            ptr::write(meta_ptr, addr);

            Self { ptr: raw_ptr, aligned_ptr }
        }
    }

    /// Returns the aligned pointer.
    fn as_ptr(&self) -> *mut u8 {
        self.aligned_ptr
    }
}

impl Drop for AlignedMemory {
    fn drop(&mut self) {
        unsafe {
            // Retrieve the original pointer
            let meta_ptr = (self.aligned_ptr as *mut usize).offset(-1);
            let original_ptr = ptr::read(meta_ptr) as *mut u8;

            // Calculate the layout for deallocation
            let size = 1024 + std::mem::size_of::<usize>();
            let layout = Layout::from_size_align(size, 1).unwrap();

            // Deallocate the original memory
            dealloc(original_ptr, layout);
        }
    }
}

fn main() {
    let alignment = 513;
    let size = 1024;

    let aligned_mem = AlignedMemory::new(size, alignment);
    println!("Aligned pointer: {:p}", aligned_mem.as_ptr());

    // Use the aligned memory (example: writing some data)
    unsafe {
        ptr::write(aligned_mem.as_ptr(), 42);
        println!("Value at aligned address: {}", *aligned_mem.as_ptr());
    }
}
