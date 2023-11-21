// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics

// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color {
    Brown,
    White,
    Black,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Brown => println!("Brown"),
            Color::White => println!("White"),
            Color::Black => println!("Black"),
        }
    }
}

struct Dimensions {
    length: f64,
    width: f64,
    height: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("Dimensions: {}x{}x{}", self.length, self.width, self.height);
    }
}

struct Box {
    dimensions: Dimensions,
    weight: f64,
    color: Color,
}

impl Box {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }

    fn print(&self) {
        println!("Weight: {}", self.weight);
        self.color.print();
        self.dimensions.print();
    }
}

fn main() {
    let small_dimensions = Dimensions {
        length: 1.0,
        width: 1.0,
        height: 1.0,
    };

    let small_box = Box::new(1.0, Color::Brown, small_dimensions);
    small_box.print();

    let large_dimensions = Dimensions {
        length: 10.0,
        width: 10.0,
        height: 10.0,
    };

    let large_box = Box::new(10.0, Color::White, large_dimensions);
    large_box.print();
}
