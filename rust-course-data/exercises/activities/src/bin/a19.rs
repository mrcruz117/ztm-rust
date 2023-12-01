// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock


use std::collections::HashMap;

struct InvItem {
    amount:   i32,
}

fn main() {
    let mut inventory = HashMap::new();
    inventory.insert("Chairs", InvItem { amount: 5 });
    inventory.insert("Beds", InvItem { amount: 3 });
    inventory.insert("Tables", InvItem { amount: 2 });
    inventory.insert("Couches", InvItem { amount: 0 });

    let mut total = 0;
    for (key, value) in &inventory {
        if value.amount == 0 {
            println!("{}: Out of stock", key);
        } else {
            println!("{}: {}", key, value.amount);
        }
        total += value.amount;
    }
    println!("Total: {}", total);
}
