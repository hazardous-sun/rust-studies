/*
Rust code uses snake case as the conventional style for function and variables names, in which all
letter cases are lowercase and underscores separate words.

A function can call a different function even though it was declared AFTER the caller function, as
long as it is in the scope of the caller function.
 */

fn arguments(x: i32, y: &str) {
    /*
    We can define 3.3 Functions to have parameters, which are special variables that are part of a
    function's signature. When a function has parameters, you can provide it with concrete values
    for those parameters. The formal name for the parameters of a function is "arguments".

    In function signatures, you must declare the type of each argument. This is a deliberate
    decision in Rust's design: requiring type annotations in function definitions means the compiler
    almost never needs you to use them elsewhere in the code to figure out what type you mean. The
    compiler is also able to give more helpful error messages if it knows what types the function
    expects.
     */

    println!("This function received an integer as the first argument: {x}, and \
    a string as the second argument: {y}");
}

fn statements_expressions() {
    /*
    Function bodies are made up of a series of statements optionally ending in an expression. So
    far, the 3.3 Functions we've covered haven't included an ending expression, but you have seen an
    expression as part of a statement. Because Rust is an expression-based language, this is an
    important distinction to understand. Other languages don't have the same distinctions, so let's
    look at what statements and expressions are and how their differences affect the bodies of
    3.3 Functions.

    * Statements are instructions that perform some action and do not return a value
    * Expressions evaluate to a resultant value. Let's look at some examples

    Creating a variable and assigning a value to it with the let keyword is a statement. "let y = 6"
    is a statement. Function definitions are also statements.

    Statements do not return values, therefore, you can't assign a "let" statement to another
    variable, as the following code tries to do; you'll get an error:
     */

    // let x = (let y = 6);
    /*
    The "let y = 6" STATEMENT does not return a value, so there isn't anything for x to bind to.
    This is different from what happens in other languages, such as C and Ruby, where the assignment
    returns the value of the assignment. In those languages, you can write "x = y = 3" and have both
    x and y have the value 6; that is not the case in Rust.

    Expressions evaluate to a value and make up most of the rest of the code that you'll write in
    Rust. Consider a math operation, such as "5 + 6", which is an expression that evaluates to the
    value of 11. Expressions can be part of statements; in "let y = 6;", the 6 in the statement is
    an expression that evaluates to the value 6. Calling a function is an expression. Calling a
    macro is an expression. A new scope block created with curly brackets is an expression, for
    example:
     */
    let y = {
        let x = 3;
        x + 1
    }; // y = 4
    println!("The value of y is: {y}");

    let y = {
        let x = 4;
        let y = 5;
        let z = 6;
        x * y * z
    }; // y = 120
    println!("The value of y is: {y}");
}

// *************************************************************************************************
// Functions with return
/*
In Rust, the return value of the 3.3 Functions is synonymous with the value of the final expression in
the block of the body of a function. You can return early from a function by using the return
keyword and specifying a value, but most 3.3 Functions return the last expression implicitly.
The returned value of a function must be declared in the signature of the function with a "->"
followed by the type the function returns:
 */
fn five() -> i32 {
    5 // returns 5 since this is the last line of the function
}

fn five_again() -> i32 {
    2;
    5 // also returns 5
}

/*
This function won't compile because, due to the semicolon, the compiler does not interpret 5 as the
returned value, but just as an expression that is not bound to a variable. The error message raised
here would be "error[E0308]: mismatched types", since the signature of the function states that it
should return a defined integer of 32-bit size, but it is returning "unit type", "()", the Rust
version of a void function.
fn five_once_more() -> i32 {
    5;
}
*/

fn plus_one(x: i32) -> i32 {
    x + 1
}
// *************************************************************************************************

fn main() {
    println!("Main function.");
    function_defined_after_caller();

    let x: i32 = 76543;
    let y = "Roberto Carlos";
    arguments(x, y);

    statements_expressions();
    let z: i32 = five();
    println!("Value of z: {z}");

    let a: i32 = five_again();
    println!("Value of a: {a}");

    let b: i32 = plus_one(2);
    println!("Value of b: {b}");
}

fn function_defined_after_caller() {
    println!("This function was declared after the caller function.");
}