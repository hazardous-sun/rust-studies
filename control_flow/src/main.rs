fn if_expressions() {
    /*
    An if expression allows you to branch your code depending on conditions. You provide a condition
    and then state, "if this condition is met, run this block of code. If the condition is not met,
    do not run this block of code."
     */

    let number = 3;
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    /*
    All if expressions start with the keyword if, followed by a condition. In this case, the
    condition checks whether or not the variable number has a values less than 5. We place the block
    of code to execute if the condition is true immediately after the condition inside curly
    brackets. Blocks of code associated with the conditions in if expressions are sometimes called
    arms, just like the arms in match expressions.

    Optionally, we can include an else expression, which we chose to do here, to give the program
    an alternative block of code to execute should the condition evaluate to false. If you don't
    provide an else expression and the condition is false, the program will just skip the if block
    and move on to the next bit of code.
     */

    let number = 7;
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    /*
    It is also worth noting that the condition in this code must be a bool. If the condition isn't a
    bool, we'll get an error. For example, the following code:
     */

    /*
    The following code will raise error message "error[E0308]: mismatched types", since the compiler
    was expecting a bool and received a i32.

    let number = 3;
    if number {
        println!("number was three");
    }

    Unlike languages as Ruby and Python, Rust will not automatically try to convert non-Boolean
    types to a Boolean. You must be explicit and always provide "if" with a Boolean as its
    condition. If we want the "if" code block to run only when a number is equal to 0, for example,
    we can change the if expression to the following:
    */

    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}

fn else_if() {
    /*
    You can use multiple conditions by combining "if" and "else" in an "else if" expression, for
    example:
     */

    let number = 4;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_in_let() {
    /*
    Because "if" is an expression, we can use it on the right side of a "let" statement to assign
    the outcome to a variable:
     */

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}"); // prints 5

    /*
    Remember that blocks of code evaluate to the last expression in them, and numbers by themselves
    are also expressions. In this case, the value of the whole "if" expression depends on which
    block of code executes. This means the values that have the potential to be results from each
    arm of the "if" must be the same type. If the types are mismatched, as in the following example,
    we'll get an error:
     */

    /*
    The following code raises error message "error[E0308]: `if` and `else` have incompatible types"
    during compilation time, because the first arm evaluates to an integer and the second evaluates
    to a string.

    let condition = true;
    let number = if condition { 5 } else { "six" };
    println!("The value of number is: {number}");

    This won't work because variables must have a single, and immutable, type, and Rust
    needs to know at compile time what type the number variable is, definitively. Knowing the type
    of number lets the compiler verify the type is valid everywhere we use number. Rust wouldn't be
    able to do that if the type of number was only determined at runtime; the compiler would be more
    complex and would make fewer guarantees about the code if it had to keep track of multiple
    hypothetical types for any variable.
    */
}

fn repetition_with_loops() {
    /*
    It's often useful to execute a block of code more than once. For this task, Rust provides
    several loops, which will run through the code inside the loop body to the end and then start
    immediately back at the beginning.

    Rust has three kinds of loops: loop, while, and for.

    The "loop" keyword tells Rust to execute a block of code over and over again forever or until
    you explicitly tell it to stop.

    The following code is an example of an infinite repeat loop:

    loop {
        println!("again!");
    }

    When we run this program, we'll see "again!" printed over and over continuously until we stop
    the program manually.

    Fortunately, Rust also provides a way to break out of a loop using code. You can place the
    "break" keyword withing the loop to tell the program when to stop executing the loop. There is
    also the "continue" keyword, that tells the program to continue to the next iteration without
    running the rest of the content inside the scope.

    One of the uses of a loop is to retry an operation you know might fail, such as checking whether
    a thread has completed its job. You might also need to pass the result of that operation out of
    the loop to the rest of your code. To do this, you can add the value you want returned after the
    "break" expression you use to stop the loop; that value will be returned out of the loop so you
    can use it, as shown here:
     */

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}"); // prints 20
}

fn main() {
    repetition_with_loops();
}
