// coding playground for Rust
use humantime::format_duration;
use std::time::Duration;



fn main() {
    let d = Duration::from_secs(123456789);
    println!("{}", format_duration(d));
}
    