// coding playground for Rust
trait CheckIn {
    fn check_in(&self);
    fn process(&self);
}

struct Pilot;
impl CheckIn for Pilot {
    fn check_in(&self) {
        println!("Pilot checking in");
    }
    fn process(&self) {
        println!("Pilot processing");
    }
}
struct Passenger;
impl CheckIn for Passenger {
    fn check_in(&self) {
        println!("Passenger checking in");
    }
    fn process(&self) {
        println!("Passenger processing");
    }
}

struct Cargo;
impl CheckIn for Cargo {
    fn check_in(&self) {
        println!("Cargo checking in");
    }
    fn process(&self) {
        println!("Cargo processing");
    }
}

fn process_item<T: CheckIn>(item: T) {
    item.check_in();
    item.process();
}

fn main() {
    let pilot = Pilot;
    let passenger = Passenger;
    let cargo1 = Cargo;
    let cargo2 = Cargo;

    process_item(pilot);
    process_item(passenger);
    process_item(cargo1);
    process_item(cargo2);
}
