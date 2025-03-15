fn main() {
    println!("Hello, main!");
    // another_function();
    let mut x = 5;
    {
        let z = 3;
        x = z;
    };
    println!("The value of y is: {}", x);
}

fn another_function() {
    println!("Another function.");
}
