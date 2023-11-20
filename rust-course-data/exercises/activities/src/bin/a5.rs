// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

use std::time::Instant;

fn count_to_n (n: i32) {
    let mut i = 1;
    loop {
        println!("{}", i);
        i += 1;
        if i > n {
            break;
        }
    }
}

fn main() {
    let start = Instant::now();
    count_to_n(15);
    let duration = start.elapsed();

    println!("Time elapsed in count_to_n is: {:?}", duration);
}
