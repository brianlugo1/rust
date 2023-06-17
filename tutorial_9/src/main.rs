// Memory Management, Heap & Stack
// The Rust Stack
// FILO / LIFO
// Fastest storage system
// Stack can only store objects with a fixed size

// The Rust Heap
// Use for dynamically sized objects

fn main() {
    // x is within a scope, the main()'s scope
    // let x = 2;
    // let y = x;

    // example();
    // add(x, y);

    let _x = 2;
    // Create a string that can vary in size
    // The variable name is stored in the stack,
    // the value stores an address on the heap
    // The heap stores the actual value at the
    // stored address
    // Storing on heap is slower than on the
    // stack
    let _string = String::from("hello");
}

// fn add(x : i32, y: i32) {
//     x + y
// }

// fn example() {
//     let a = "true";
//     let b = false;
// }
