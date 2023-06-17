fn main() {
    // When defining a variable, either define type manually or let compiler set the type!
    // However, the type of the variable that does not have a defined type cannot be changed through out the file!
    // Example of how to define the type (u32 = unsigned int 32-bit)
    // let x: u32 = 4;
    // let x = 4;
    // let mut x = 4;
    let x = 4;
    // Example of how to embed variable in string
    println!("x is: {}", x);
    // By default variables cannot be changed the value
    // Need to add mut
    // x = 5;

    // Make custom scope with {}
    {
        // Interior scope allows x to only be equal to 2 inside the scope, and not outside the scope!
        // let x = 2;
        // An exterior scope variable with the same name can be used in the interior scope variable!
        let x = x - 2;
        println!("x is: {}", x);
    }

    // When defining a new varible with let, the type can be changed!
    let x = "hello";
    println!("x is: {}", x);

    // Inside rust, one can recreate a variable that is immutable to be able to assign the variable a new value!
    let x = 5;
    println!("x is: {}", x);

    // const value cannot be changed, the variable name needs to follow UPPERCASE Snakecase, and must set the type!
    const SECONDS_IN_MINUTE: u32 = 60;
    // const values cannot be redefined to be reassigned!
    // const SECONDS_IN_MINUTE: u32 = 100
    println!("{}", SECONDS_IN_MINUTE);
}
