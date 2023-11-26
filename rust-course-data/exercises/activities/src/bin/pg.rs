// coding playground for Rust

fn prnt_vect_elements(vect: &Vec<i32>) {
    for i in vect {
        println!("{}", i);
    }
}

fn main() {
    let vect: Vec<i32> = vec![1, 2, 3, 4, 5];

    prnt_vect_elements(&vect);
}
