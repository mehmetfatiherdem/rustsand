fn main() {
    
    // variables and mutability
    let x = 5; // immutable by default
    println!("immutable x is {x}");

    let mut y = 5; // made it mutable
    println!("mutable y before mutation is {y}");
    y = 10;
    println!("mutable y after mutation is {y}");

    // constants left at 47 in rust-book.pdf

}
