// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.
use std::{collections::HashMap, io};
enum MainMenu {
    Add,
    View,
    Edit,
    Remove,
    Quit,
}

#[derive(Debug, Clone)]
pub struct Bill {
    name: String,
    amount: f64,
}

pub struct Bills {
    inner: HashMap<String, Bill>,
}

impl Bills {
    fn new() -> Bills {
        Self {
            inner: HashMap::new(),
        }
    }
    fn add(&mut self, bill: Bill) {
        self.inner.insert(bill.name.to_string(), bill);
    }

    fn get_all(&self) -> Vec<&Bill> {
        self.inner.values().collect()
    }

    fn remove(&mut self, name: &str) -> bool {
        self.inner.remove(name).is_some()
    }
}

impl MainMenu {
    fn from_str(input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(MainMenu::Add),
            "2" => Some(MainMenu::View),
            "3" => Some(MainMenu::Edit),
            "4" => Some(MainMenu::Remove),
            "5" => Some(MainMenu::Quit),
            _ => None,
        }
    }
    fn show() {
        println!("1. Add");
        println!("2. View");
        println!("3. Edit");
        println!("4. Remove");
        println!("5. Quit");
        println!("Enter choice: ");
    }
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Error reading input");
    }
    let input = buffer.trim().to_lowercase();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

fn get_bill_amount() -> Option<f64> {
    println!("Bill Amount:");
    loop {
        let amount = match get_input() {
            Some(amount) => amount,
            None => return None,
        };
        if &amount == "" {
            return None;
        }
        let parsed_amount: Result<f64, _> = amount.parse();
        match parsed_amount {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Please enter a number"),
        }
    }
}

mod menu {
    use crate::{get_bill_amount, get_input, Bill, Bills};
    pub fn add_bill(bills: &mut Bills) {
        println!("Bill Name:");
        let name = match get_input() {
            Some(name) => name,
            None => return,
        };
        let amount = match get_bill_amount() {
            Some(amount) => amount,
            None => return,
        };
        let bill = Bill { name, amount };
        bills.add(bill);
        print!("Bill added");
    }

    pub fn view_bills(bills: &Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill);
        }
    }

    pub fn remove_bill(bills: &mut Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill);
        }
        println!("Enter bill name to remove:");
        let name = match get_input() {
            Some(name) => name,
            None => return,
        };

        if bills.remove(&name) {
            println!("Bill removed");
        } else {
            println!("Bill not found");
        }
    }
}

fn main() {
    let mut bills = Bills::new();
    loop {
        // display menu
        MainMenu::show();
        let input = get_input().expect("Error reading input");
        // make choice based on input
        match MainMenu::from_str(input.as_str()) {
            Some(MainMenu::Add) => menu::add_bill(&mut bills),
            Some(MainMenu::View) => menu::view_bills(&bills),
            Some(MainMenu::Edit) => println!("Edit"),
            Some(MainMenu::Remove) => menu::remove_bill(&mut bills),
            Some(MainMenu::Quit) => break,
            None => return,
        }
    }
}
