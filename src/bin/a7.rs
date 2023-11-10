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


enum color {
    blue,
    red,
    black,
    pink
}

fn print_color (my_color:color){
match my_color {
    color::blue => println!("blue color"),
    color::red => println!("red color"),
    color::black => println!("black color"),
    color::pink => println!("pink color"),
    }
}

fn main() {
    print_color(color::red);
    print_color(color::black);
    print_color(color::blue);
    print_color(color::pink);
   
}
