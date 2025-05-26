struct CustomResource;

impl CustomResource {
    fn before_drop(&self) {
        println!("Doing something before drop!");
    }
}

impl Drop for CustomResource {
    fn drop(&mut self) {
        println!("Resource is being cleaned up!");
    }
}

fn main() {
    {
        let resource = CustomResource;
        resource.before_drop();
        // resource goes out of scope here → Drop::drop() is called
    }
    println!("Resource is now dropped.");
}

