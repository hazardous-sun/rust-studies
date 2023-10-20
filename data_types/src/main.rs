use std::io;

// Scalar type
fn integers() {
    /*
    Integers:
    | Lenght | Signed | Unsigned |
    | :---: | :---: | :---: |
    | 8-bit | i8 | u8 |
    | 16-bit | i16 | u16 |
    | 32-bit | i32 | u32 |
    | 64-bit | i64 | u64 |
    | 128-bit | i128 | u128 |
    | arch | isize | usize |

    isize and usize types depend on the architecture of the computer the program is running on, which is
    denoted in the table as "arch":
     * 64 bits if you are on a 64-bit architecture and 32 bits if you are on a 32-bit architecture

    Integers can be written with literals in any of the following forms:
    | Number literals | Example |
    | :---: | :---: |
    | Hex | 0xff |
    | Octal | 0o77 |
    | Binary | 0b1111_0000 |
    | Byte (u8 only) | b'A'|
     */

    let var1: u8 = 255; // max value var1 can hold
    let var2: u16 = 65535; // max value var2 can hold
    let var3: u32 = 4294967295; // max value var3 can hold
    let var4: u64 = 18446744073709551615; // max value var4 can hold
    let var5: u128 = 340282366920938463463374607431768211455; // max value var5 can hold

    println!("var1 = {var1}");
    println!("var2 = {var2}");
    println!("var3 = {var3}");
    println!("var4 = {var4}");
    println!("var5 = {var5}");

    // let var1: u8 = var1 + 1;

    // Different literals
    let var6 = 0xff; // 255 in decimal
    let var7 = 0o77; // 63 in decimal
    let var8 = 0b1111_0000; // 240 in decimal
    let var9 = b'A'; // 65 in decimal

    println!("var6 = {var6}");
    println!("var7 = {var7}");
    println!("var8 = {var8}");
    println!("var9 = {var9}");
}

fn floating_points() {
    let x: f64 = 2.0; // 64 bits long
    let y: f32 = 3.0; // 32 bits long
    let z = 5.5; // 64 bits long by default

    println!("x = {x}");
    println!("y = {y}");
    println!("z = {z}");
}

fn numeric_operations() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 5.4 * 30.0;

    // division
    let quotient = 56.7 / 32.2;
    let truncated_quotient = -5 / 3; // results in -1

    // remainder
    let remainder = 43 % 5;

    println!("sum = {sum}");
    println!("difference = {difference}");
    println!("product = {product}");
    println!("quotient = {quotient}");
    println!("truncated_quotient = {truncated_quotient}");
    println!("remainder = {remainder}");
}

fn boolean() {
    let t = true;
    let f: bool = false; // with explicit type annotation
}

fn character() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}

// Compound types
fn tuples() {
    /*
    A tuple is a general way of grouping together a number of values with a variety of types into
    one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size

    We create a tuple by writing a comma-separated list of values inside parentheses. Each position
    in the tuple has a type, and the types of the different values in the tuple don't have to be the
    same
     */

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    /*
    The variable tup binds to the entire tuple because a tuple is considered a single compound
    element. To get the individual values out of a tuple, we can use pattern matching to destructure
    a tuple value like this:
     */

    let (x, y, z) = tup;
    // now x contains a i32 with the value of 500, y a f64 with the value of 6.4 and z a u8 with 1

    println!("The value of y is: {y}"); // prints 6.4

    /*
    We can also access a tuple element directly bu using a period (".") followed by the index of the
    value we want to access:
     */

    let x = (500, 6.4, 1);
    let a = x.0; // a receives the value of 500
    let b = x.1; // b receives the value of 6.4
    let c = x.2; // c receives the value of 1

    println!("a = {a}"); // prints 500
    println!("b = {b}"); // prints 6.4
    println!("c = {c}"); // prints 1
}

fn arrays() {
    /*
    Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other
    languages, arrays in Rust must have a fixed length.

    Arrays are written as a comma-separated list inside square brackets:
     */

    let a = [1, 2, 3, 4, 5];

    /*
    Arrays allocate memory on the stack rather than the heap, or when you want to ensure you always
    have a fixed number of elements.

    Arrays are more useful when you know the number of elements will not need to change. For
    example, if you were using the names of the month in a program, you would probably use an array
    rather than a vector because you know it will always contain 12 elements:
     */

    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    /*
    You can initialize an array to contain the same value for each element by specifying the initial
    value, followed by a semicolon, and then the length of the array in square brackets:
     */
    let a = [3; 5];
    // a is an array with the following values [3, 3, 3, 3, 3]

    /*
    An array is a single chunk of memory of a know, fixed size that can be allocated on the stack.
    You can access elements of an array using indexing, like this:
     */
    let a = [5, 4, 3, 2, 1];
    let x = a[0]; // x == 5
    let y = a[1]; // y == 4
    let z = a[2]; // z == 3

    // *********************************************************************************************
    // Invalid array element access
    /*
    The following code is an example of a program that passes through compilation time without
    issues but can panic doing runtime:
     */
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    // The following code will panic if you input a value higher than 5 (the length of the array)
    let element = a[index];

    println!("The value of element at index {index} is: {element}");
}

fn main() {
    arrays();
}
