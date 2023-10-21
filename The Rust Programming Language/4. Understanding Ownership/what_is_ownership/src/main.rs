/*
Ownership is a set of rules that govern how a Rust program manages memory. All programs have to
manage the way they use a computer's memory while running. Some languages have garbage collection
that regularly looks for no-longer-used memory as the program runs; in other languages, the
programmer must explicitly allocate and free the memory. Rust uses a third approach: memory is
managed through a system of ownership with a set of rules that the compiler checks. If any of the
rules are violated, the program won't compile. None of the features of ownership will slow down your
program while it's running.

Ownership Rules:
* Each value in Rust has an owner
* There can only be one owner at a time
* When the owner goes out of scope, the value will be dropped
*/

fn variable_scope() {
    /*
    As a first example of ownership, we'll look at the scope of some variables. A scope is the range
    within a program for which an item is valid. Take the following variables:
     */

    let s = "hello";

    /*
    The variable "s" refers to a string literal, where the value of the string is hardcoded into the
    text of our program. The variable is valid from the point at which it's declared until the end
    of the current scope.
     */
    {                          // s is not valid here, since it's not yet declared
        let s: &str = "hello"; // s is valid from this point forward
        // do stuff with s
    }                          // this scope is now over, and s is no longer valid

    /*
    There are two important points in time here:
    1. When "s" comes into scope, it is valid
    2. It remains valid until it goes out of scope

    At this point, the relationship between scopes and when variables are valid is similar to that
    in other programming languages. Now we'll build on top of this understanding by introducing the
    "String" type.
    */
}

fn string_type() {
    /*
    To illustrate the rules of ownership, we need a data type that is more complex than those we
    covered in "Data Types" section of chapter 3. The types covered previously are of a known size,
    can be stored on the stack and popped off the stack when their scope is over, and can be quickly
    and trivially copied to make a new, independent instance if another part of code needs to use
    the same value in a different scope. But we want to look at data that is stored on the heap and
    explore how Rust knows when to clean up that data, and the String type is a great example.

    We'll concentrate on the parts of String that relate to ownership. These aspects also apply to
    other complex data types, whether they are provided by the standard library or created by you.

    We've already seen string literals, where a string value is hardcoded into our program. String
    literals are convenient, but they aren't suitable for every situation in which we may want to
    use text. One reason is that they are immutable. Another is that not every string value can be
    known when we write our code: for example, what if we want to take user input and store it? For
    these situations, Rust ha a second string type, "String". This type manages data allocated on
    the heap and as such is able to store an amount of text that is unknown to us at compile time.
    You can create a "String" from a string literal using "from" function, like so:
     */

    let s = String::from("hello");
}

fn main() {
    println!("Hello, world!");
}
