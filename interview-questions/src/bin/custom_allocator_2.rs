//Implement an allocator which allocates data on addresses multiple of 10
use std::alloc::{alloc, dealloc, Layout};

struct MyAllocator;

impl MyAllocator {
    pub fn alloc(&self, size: usize) -> *mut u8 {
        let layout = Layout::from_size_align(size, 16).unwrap(); // Align to 16 (power of 2)
        
        unsafe {
            // Allocate memory and cast it to a raw pointer
            let ptr = alloc(layout);
            if ptr.is_null() {
                panic!("Allocation failed");
            }

            // Adjust pointer to the nearest multiple of 10
            let aligned_ptr = (ptr as usize + 9) & !0xF; // Round up to the next multiple of 10
            aligned_ptr as *mut u8
        }
    }

    pub fn dealloc(&self, ptr: *mut u8, size: usize) {
        let layout = Layout::from_size_align(size, 16).unwrap(); // Align to 16 (power of 2)
        unsafe {
            dealloc(ptr, layout); // Deallocate memory
        }
    }
}

fn main() {
    let allocator = MyAllocator;

    // Allocate memory for 100 bytes
    let size = 100;
    let ptr = allocator.alloc(size);
    println!("Allocated memory at address: {:?}", ptr);

    // Deallocate memory
    allocator.dealloc(ptr, size);
    println!("Memory deallocated.");
}
