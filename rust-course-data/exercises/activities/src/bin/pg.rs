// coding playground for Rust

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    match numbers.is_empty() {
        true => println!("No numbers"),
        false => println!("{} numbers", numbers.len()),
    }
}
