struct CustomResource;
impl Drop for CustomResource {
    fn drop(&mut self) {
        println!("Resource is being cleaned up!");
    }
}
fn main() {
    let _resource = CustomResource;
    println!("End of main.");
}