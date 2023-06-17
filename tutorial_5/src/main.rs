// Prelude brings in basic functionality
// Can import a crate (Package or Library)
// Inside crate there is a Module
// Using the std crate, import module io
use std::io;

fn main() {
    println!("Hello, world!");
    // :: allows to access a sub component
    // Do in the String Module and access new()
    let mut input = String::new();

    // stdin() reads in text from the console
    // Creating a mutable reference to directly modify
    // data stored in the variable
    // & is a reference, by default is immutable
    // read_line() returns a result object
    // expect() catches errors, check if valid value is read
    // read_line() can only take a String type
    io::stdin().read_line(&mut input).expect("failed to read line");
    // Warning is given if not handling potential error
    // io::stdin().read_line(&mut input);
    println!("{}", input);
}
