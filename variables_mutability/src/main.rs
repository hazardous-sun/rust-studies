fn immutable_variable() {
    let x = 5;
    println!("x = {x}");

    /*
    In Rust, all variables are immutable by nature, which means that running the following line of
    code would raise error "error[E0384]: cannot assign twice to immutable variable" during compilation
    time
     */
    // x = 6;
    println!("x = {x}");
}

fn mutable_variable() {
    /*
    Even though variables are immutable by nature in Rust, a developer can make them mutable by
    adding the "mut" key word during the initialization of the variable
     */
    let mut x = 5;
    println!("x = {x}");

    x = 5;
    println!("x = {x}");
}

fn constants() {
    /*
    Constants are ALWAYS immutable and require their type to be explicitly declared during constant
    initialization

    Can be declared in any scope, including the global scope
     */

    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;

    println!("THREE_HOURS_IN_SECONDS = {THREE_HOURS_IN_SECONDS}");
}

fn shadowing() {
    let x = 5; // creates a "x" variable with the value of 5
    // now the value of x is 5

    let x = x + 1;  // creates a new variable that shadows "x" with the value of x+1
    // now the value of x is 6

    {
        let x = x * 2;  // creates a third variable that also shadows "x" with the value of x*2
        // now the value of x INSIDE THE INNER SCOPE is 12
        println!("The value of 'x' in the inner scope is: {x}"); // prints the third value
    }

    println!("The value of x is: {x}"); // prints the second value

    /*
    Shadowing is different from marking a variable as mutable because we'll get a compile-time error
    if we accidentally try to reassign to this variable without using the let keyword. By using let,
    we can perform a few transformations on a value but have the variable be immutable after those
    transformations.

    The other difference between mut and shadowing is that because we're effectively creating a new
    variable when we use the let keyword again, we can change the type of the value but reuse the
    same name. For example, let's say our program asks a user to show how many spaces they want
    between some text by inputting space characters, and then we want to store that input as a
    number:
     */

    let spaces = "     ";
    let spaces = spaces.len();

    /*
    The first spaces variable is a string type and the second spaces is a number type. Shadowing
    thus spares us from having to come up with different names, such as spaces_str and spaces_num;
    instead, we can reuse the simpler spaces name. However, if we try to use mut for this, as show
    in the following code, we'll get a compile-time error:
     */

    let mut mut_spaces = "     ";
    // mut_spaces = mut_spaces.len(); // this will raise "error[E0308]: mismatched types"
}

fn main() {}
