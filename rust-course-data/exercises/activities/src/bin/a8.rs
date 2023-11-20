// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks

// * Use a struct to store drink flavor and fluid ounce information
enum Flavor {
    Orange,
    Grape,
    LemonLime,
}
struct Drink {
    flavor: Flavor,
    ounces: f64,
}
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor
fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Orange => println!("orange"),
        Flavor::Grape => println!("grape"),
        Flavor::LemonLime => println!("lemon-lime"),
    }
    println!("{} ounces", drink.ounces);
}

fn main() {
    let drink = Drink {
        flavor: Flavor::Orange,
        ounces: 12.0,
    };
    print_drink(drink);
}
