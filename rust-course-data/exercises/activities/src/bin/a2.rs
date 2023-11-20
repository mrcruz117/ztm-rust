// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result
// 
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let a = 5;
    let b = 7;
    let result = add(a, b);
    println!("{:?}", result);
}
