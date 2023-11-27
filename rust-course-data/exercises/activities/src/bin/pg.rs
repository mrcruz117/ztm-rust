// coding playground for Rust

enum Discount {
    Percent(f64),
    Flat(f64),
}

struct Ticket {
    event: String,
    price: i32,
}

fn main() {
    let n = 5;
    match n {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        other => println!("Other: {:?}", other),
    }

    let flat = Discount::Flat(10.0);
    match flat {
        Discount::Flat(amount) => println!("Flat: {}", amount),
        Discount::Percent(percent) => println!("Percent: {}", percent),
    }

    let concert = Ticket {
        event: "Concert".to_owned(),
        price: 51,
    };

    match concert {
        Ticket { price: 50, event } => println!("Event: {}", event),
        Ticket { event, price } => println!("Event: {}, Price: {}", event, price),
    }
}
