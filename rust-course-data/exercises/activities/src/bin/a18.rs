// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

struct Customer {
    age: u8,
}

fn can_make_restricted_purchase(customer: &Customer) -> Result<(), String> {
    if customer.age >= 21 {
        Ok(())
    } else {
        Err("Customer is not old enough to make a restricted purchase".to_string())
    }
}

fn main() {
    let customer = Customer { age: 22 };
    match can_make_restricted_purchase(&customer) {
        Ok(_) => println!("Customer can make a restricted purchase"),
        Err(e) => println!("Customer cannot make a restricted purchase: {}", e),
    }
}
