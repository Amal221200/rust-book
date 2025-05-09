use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an index:");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Please type a number!");

    if index >= a.len() {
        println!("The index is too big!");
        return;
    }

    let element = a[index];
    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
