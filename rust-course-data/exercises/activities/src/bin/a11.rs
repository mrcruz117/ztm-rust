// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number

struct GroceryItem {
    quantity: i32,
    id: i32,
}
// * Create a function to display the quantity, with the struct as a parameter
fn print_quantity(item: &GroceryItem) {
    println!("quantity: {:?}", item.quantity);
}
// * Create a function to display the id number, with the struct as a parameter

fn print_id(item: &GroceryItem) {
    println!("id: {:?}", item.id);
}

fn main() {
    let item = GroceryItem {
        quantity: 1,
        id: 2,
    };
    print_quantity(&item);
    print_id(&item);
}
