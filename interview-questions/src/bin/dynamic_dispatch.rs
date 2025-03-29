trait Animal {
    fn make_sound(&self);
}
struct Dog;
struct Cat;
impl Animal for Dog {
    fn make_sound(&self) {
        println!("Bark!");
    }
}
impl Animal for Cat {
    fn make_sound(&self) {
        println!("Meow!");
    }
}
fn main() {
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];
    for animal in animals {
        animal.make_sound();
    }
}