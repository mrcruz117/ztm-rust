// coding playground for Rust
use std::collections::HashMap;

#[derive(Debug)]
struct Contents {
    contents: String,
}

fn main() {
    let mut lockers = HashMap::new();
    lockers.insert(
        1,
        Contents {
            contents: "books".to_string(),
        },
    );
    lockers.insert(
        2,
        Contents {
            contents: "homework".to_string(),
        },
    );
    lockers.insert(
        3,
        Contents {
            contents: "supplies".to_string(),
        },
    );

    for (key, value) in &lockers {
        println!("{:?}: {:?}", key, value.contents);
    }
}
