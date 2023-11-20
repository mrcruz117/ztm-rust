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
    Blue,
    Green,
    RgbColor(u8, u8, u8), // tuple
    CmykColor{cyan: u8, magenta: u8, yellow: u8, black: u8}, // struct
}

fn print_color(c: Color) {
    match c {
        Color::Red => println!("red"),
        Color::Blue => println!("blue"),
        Color::Green => println!("green"),
        Color::RgbColor(0, 0, 0) => println!("black"),
        Color::RgbColor(r, g, b) => println!("rgb({}, {}, {})", r, g, b),
        Color::CmykColor{cyan: _, magenta: _, yellow: _, black: 255} => println!("black"),
        Color::CmykColor{cyan: c, magenta: m, yellow: y, black: k} => println!("cmyk({}, {}, {}, {})", c, m, y, k),
    }
}

fn main() {
    print_color(Color::Red);
    print_color(Color::Blue);
    print_color(Color::Green);
    print_color(Color::RgbColor(0, 0, 0));
    print_color(Color::RgbColor(255, 255, 255));
    print_color(Color::CmykColor{cyan: 0, magenta: 0, yellow: 0, black: 255});
    print_color(Color::CmykColor{cyan: 100, magenta: 100, yellow: 100, black: 100});
}
