// scalar type = single value (int, double)
// compound type = multiple values (tuple, arrays)
fn main() {
    // By default unused variable names cause warning
    // However if one adds an _ to the beginning,
    // the warning goes away!
    // Scalar types:
    // Assigning type integer 32-bit to variable x
    let _x: i32 = 2;
    // i8
    // i16
    // i32 is default
    // i64
    // i128
    // whole numbers only
    // Assigning type unsigned integer 32-bit to variable x
    let _x: u32 = 2;
    // u8 and i8 represent the same number of unique values
    // however different ranges
    // u8 0 <= (2^8 -1) (0-255)
    // i8 -2^7 <= (2^7-1) ((-128)-127)

    // f32 (single precision) vs f64 (double precision)
    // default type is f64
    // let floating_point: f32 = 10.9;
    let _floating_point = 10.9;

    // bool stores true(1) or false(0)
    let _true_or_false: bool = false;

    // char stores a single character
    let _letter: char = 'a';

    // Compound type
    // Tuple can store multiple values
    // let tup: (i32, bool, char) = (1, true, 's');
    let mut tup: (i32, bool, char) = (1, true, 's');
    tup.0 = 10;
    // Can not add an element to a tuple
    // Inorder to access the different values of a tuple,
    // must use the variable_name.index
    // println!("{}, {}, {}", tup.0, tup.1, tup.2);
    // Both Tuples have a type of what they store
    // let tup2: (i8, bool, char) = (1, true, 's');
    // By default tuples are immutable (cannot change
    // let mut tup2: (i8, bool, char) = (1, true, 's');

    // Array values need to all be the same type
    // Cannot add values to an array, need to create a new
    // array!
    // let arr = [1, 2, 3, 4, 5];
    // let mut arr = [1, 2, 3, 4, 5];
    // Define the type and the size
    let mut arr:[i32; 5] = [1, 2, 3, 4, 5];
    arr[4] = 3;
    // Values in an array are not initialized, one
    // must initialize values!
    // let mut arr:[i32; 5];
    // An empty array does not meet the specified size of 5
    // let mut arr: [i32, 5] = [];
    println!("{}", arr[4]);

    let x: u8 = 4;
    // The type of y is u8
    // let y = x;
    // We cannot change the type through assigning the value
    // stored in a different variable
    let _y = x;
    // However we can store the same value in both
    let y: i32 = 4;
    println!("{}, {}", x, y);
}
