// Type Conversions
use std::io;

fn main() {
    // Literal is the value we are typing
    // Overflow causes an error
    // let _x: u8 = 12; // 0 - 255
    // let _x: i8 = 12;
    // let _y: i8 = 10; // (-128) - 127
    // Addition generates an overflow
    // let _x: u8 = 255;
    // let _y: u8 = 1;
    // let _y: u8 = 10;
    // let _x: f64 = 255.0;
    // let _y: f64 = 10.0;
    // Cast Literal default without defining the type by
    // adding the type at the end!
    // let _x = 255.0f32;
    // let _y = 10.0f32;
    // let _x = 127_i8;
    // let _y = 10_i8;
    // let _x = 127_000i64;
    // Explictly change type using 'as'
    // let _x = 127_000 as i64;
    // let _y = 10_i64;
    let _x = (i32::MAX as i64) + 1;
    let _y = 10_i32;

    // Cannot add variables that do not have the same type
    // let z = _x + _y;
    // let z = _y - _x;
    // Result of an operation has the same type of the operands
    // let z = _x / _y;
    // let z = _x * _y;
    // Modular operation returns remainder after division
    // let z = _x % _y;
    // let z = _x / _y;
    // Use Explicit Cast type to perform arithmatic
    // let z = _x / (_y as i64);
    // Causes overflow but no error is given
    let z = (_x as i32) / _y;
    // Should convert smaller value to larger to prevent overflow!
    println!("{}", z);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("expected to read line");

    // trim() removes '/0' when entering in console
    // parse() returns an integer from parsing the string
    // unwrap() takes valid integer result and unwrap it into the actual type
    let int_input: i64 = input.trim().parse().unwrap();

    println!("{}", int_input + 2);
}
