// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Color {
    Red,
    Yellow,
    Blue
}

fn print_color(selected_color : Color){
    match selected_color {
        Color::Red => println!("Red color"),
        Color::Yellow => println!("Yellow color"),
        Color::Blue => println!("Blue color")
    }
}
fn main() {
    print_color(Color::Blue);
}

