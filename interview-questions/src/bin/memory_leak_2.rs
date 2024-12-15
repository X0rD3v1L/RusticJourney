fn main() {
    let mut vecs = Vec::new();

    /*
    for _ in 0..1000 { // Limit the loop to a reasonable number
        vecs.push(Box::new(42)); // Allocate memory
    }
    */
    
    //Don't run
    loop {
        vecs.push(Box::new(42));
        // Continuously allocating memory on the heap
        // No way to free the memory, leading to a memory leak
    }

    //Solution

    // Drop the allocated memory by clearing the vector, Now the memory is deallocated
    // vecs.clear();
}