/*
- Variables are immutable.
-   When a variable is immutable, once a value is found to a name, you can't change that value.
-   In order to make variables mutable, add 'mut' for the variable.
 */

/*
- Constants
-   Constants are values that are bound to a name and are not allows to change.
-   You must use 'const' to declare a constant
-   Constants in Rust must be named uppercase and underscores
 */

/*
- Shadowing
-   Shadowing is when the 2nd variable that is named the same is assigned a new value, this value will be the new value
 */

/*
- Scalar Types
-   A scalar type represents a single value. Rust has 4 scalar types: 1. Integer, 2. floats, 3 booleans, 4. characters
-   1. Integer Types; Integer is a number without a fractional component. (Whole numbers)
-   2. Floats Types: Floats is a number with decimal point
-   3. Boolean Types: it has 2 values true/false
-   4. Character Type:
 */

/*
- Compound Types
-   Compound types can group multiple values into 1. Rust has Tuples and arrays.

- Tuple Type
    - A Type is a general way of grouping together a number of values which a variety of types into one compound type.
    - Tuple type have a fixed length, once declared they can't grow or shrink in size.
    - "tup" binds the entire tuple in 1 single compound.
 */
/*fn main() {
    // create immutable variable
    let immutable_var = 6;
    println!("Value of Immutable Variable: {}", immutable_var);

    // create mutable variable
    let mut muttable_var = 9;
    muttable_var = 10;
    println!("Value of Immutable Variable: {}", muttable_var);

    // create constant
    const CONST_VALUE: i32 = 10 * 10;
    println!("Value of Constant Variable: {}", CONST_VALUE);

    // Shadowing
    let shadow = 1;
    let x = shadow + 1;
    {
        let x = x * 2;
        println!("The Value of x in the inner scope is: {x}");
    }
    println!("The value of x in the outer scope is: {x}");

    // Integer
    let integer = 4;
    // float
    let float = 4.2;
    // character
    let char = "0xBinary";
    println!("Value Integer: {}, Value Float: {}, Value Character: {}", integer, float, char);

    // Define tuple
    let tup = (500, 6.4, 1);
    // get single value from tuple
    let (x, y, z) = tup;
    println!("Value y: {}, Value x: {}, Value z; {}", x, y, z);

    // Define tuple method2
    let method2 = (500, 5, 2.4);
    let test1 = method2.0;
    println!("Value of Index 0: {}", test1);
}
 */

// Import the standard input/output module for user input
use std::io;

fn main() {
    // Define an array of integers
    let array = [1, 2, 3, 4, 5];

    // Prompt the user to enter an array index
    println!(">> Please Enter An Array Index!\n");

    // Create a new empty string to store the user input
    let mut index = String::new();

    // Read user input into the index string
    io::stdin()
        .read_line(&mut index)
        // Print an error message if reading fails
        .expect("Failed to read line.");

    // Convert the user input (String) to a numeric type (usize)
    let index: usize = index
        // Remove leading and trailing whitespaces
        .trim()
        // Parse the string into a usize
        .parse()
        // Print an error message if parsing fails
        .expect("Index entered was not a number!\n");

    // Retrieve the element from the array at the specified index
    let array_element = array[index];

    // Print the value of the element at the specified index
    println!(">> The value of the element at index {} is: {}", index, array_element);
}
