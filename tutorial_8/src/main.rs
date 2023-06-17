// Functions, Expressions & Statements
// The definition of the function can be placed anywhere

fn main() {
    println!("Hello, world!");
    // add_numbers(20, 30);
    // test();

    // Statement does not return a value
    // Other programming languages you can nest statements,
    // but in rust you can't
    // let x = (let y = 20);
    // let x = 20;

    // Expression evalutes and returns a value
    // let x = 2 < 3;

    // In this case, we are assigning a value to value,
    // then returning the value
    let number = {
        let x = 3;
        // Cannot put a ;, otherwise does not work
        // Expression below is being returned
        x + 1
        // If we add a ;, then we are not return an expression,
        // but a statement
        // x + 1;
    };
    println!("{}", number);

    // let result = add_numbers(2, 3);
    let result = add_numbers(1, 3);
    println!("{}", result);
}

// -> is the return operation to define what type is returned
// Write expression without ; to return that value
// An error will occur if we do not specify the return type!
fn add_numbers(x: i32, y: i32) -> i32 {
    // Expression below is returned
    // x + y
    // Can also use return keyword with ; or not!
    // return x + y
    // return x + y;
    // let result = x + y;
    let _result = x + y;
    // if result > 10 {
    if _result > 10 {
        // return result - 10;
        // result - 10;
        // Inorder to return a value without using the keyword return,
        // simply store the expression in the variable _
        let _ = _result - 10;
    }
    // Can only write expression at the very end
    // If needed to return early, then use keyword return
    // result
    _result
}

// Need to specify types of parameters in function definition
// function definition is also a statement,
// cannot store a function declaration to a variable
// let function = fn add_numbers(...) {...}
// fn add_numbers(x: i32, y: i32) {
//     println!("The sum is: {}", x + y);
// }

// convention in Rust to write names is snakecase
// fn test() {
//     println!("Test has been called...");
// }
