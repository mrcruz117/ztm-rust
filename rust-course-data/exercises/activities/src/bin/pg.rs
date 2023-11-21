// coding playground for Rust
struct Temperature {
    degrees_f: f64,
}

impl Temperature {
    fn freezing() -> Self {
        Self { degrees_f: 32.0 }
    }

    fn show_temp(&self) {
        println!("The temperature is {}°F", self.degrees_f);
    }
}

fn main() {
    let hot = Temperature { degrees_f: 99.9 };
    hot.show_temp();

    let cold = Temperature::freezing();
    cold.show_temp();
    cold.show_temp();
    cold.show_temp();
    cold.show_temp();
    cold.show_temp();
}
