// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display


fn bool_func(boolean: bool) {
   
    match boolean {
        true => println!("it's true"),
        false => println!("it's false"),
    }
}

fn main() {
bool_func(false);
}
