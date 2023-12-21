// coding playground for Rust

#[derive(Debug)]
struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

trait Convey {
    fn weight(&self) -> f64;
    fn dimensions(&self) -> Dimensions;
}

struct ConveyorBelt<T: Convey> {
    pub items: Vec<T>,
}

impl<T: Convey> ConveyorBelt<T> {
    pub fn add(&mut self, item: T) {
        self.items.push(item);
    }
}

struct CarPart {
    width: f64,
    height: f64,
    depth: f64,
    weight: f64,
    part_number: String,
}

impl Default for CarPart {
    fn default() -> Self {
        Self {
            width: 5.0,
            height: 1.0,
            depth: 2.0,
            weight: 3.0,
            part_number: "1234".to_string(),
        }
    }
}
impl Convey for CarPart {
    fn weight(&self) -> f64 {
        self.weight
    }
    fn dimensions(&self) -> Dimensions {
        Dimensions {
            width: self.width,
            height: self.height,
            depth: self.depth,
        }
    }
}

fn main() {
    let mut belt: ConveyorBelt<CarPart> = ConveyorBelt { items: vec![] };
    let part = CarPart {
        width: 1.0,
        height: 2.0,
        depth: 3.0,
        weight: 4.0,
        part_number: "1234".to_string(),
    };
    let default_part = CarPart::default();
    belt.add(part);
    belt.add(default_part);

    for item in belt.items {
        println!("weight: {}", item.weight());
        println!("dimensions: {:?}", item.dimensions());
    }
}
