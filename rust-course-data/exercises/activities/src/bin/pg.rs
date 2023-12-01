// coding playground for Rust

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let mut plus_one_vec = vec![];

    for number in &numbers {
        plus_one_vec.push(number + 1);
    }

    let plus_one_iter: Vec<_> = numbers.iter().map(|number| number + 1).collect();

    println!("push {:?}", plus_one_vec);
    println!("iter {:?}", plus_one_iter);
}
