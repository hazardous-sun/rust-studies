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

    /*
    The double colon "::" operator allows us to namespace this particular "from" function under the
    String type rather than using some sort of name like string_from. We'll discuss this syntax more
    in the "Method Syntax" section on Chapter 5, and when we talk about namespacing with modules in
    "Paths for Referring to an Item in the Module Tree" in Chapter 7.

    This kind of string CAN be mutated:
     */

    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // prints "hello, world!"

    /*
    So, what's the difference here? Why can "String" be mutated but literals cannot? The difference
    is in how these two types deal with memory.
     */
}

fn memory_and_allocation() {
    /*
    In the case of a string literal, we know the contents at compile time, so the text is hardcoded
    directly into the final executable. This is why string literals are fast and efficient. But
    these properties only come from the string literal's immutability. Unfortunately, we can't put a
    blob (Binary Large OBject) of memory into the binary for each piece of text whose size is
    unknown at compile time and whose size might change while running the program.

    With the String type, in order to support a mutable, growable piece of text, we need to allocate
    an amount of memory on the heap, unknown at compile time, to hold the contents. This means:

    * The memory must be requested from the memory allocator at runtime
    * We need a way of returning this memory to the allocator when we're done with our String

    That first part is done by us: when we call String::from, its implementation requests the memory
    it needs. This is pretty much universal in programming languages.

    However, the second part is different. In languages with garbage collector (GC), the GC keeps
    track of and cleans up memory that isn't being used anymore, and we don't need to think about
    it. In most languages without a GC, it's our responsibility to identify when memory is no longer
    being used and to call code to explicitly free it, just as we did to request it. Doing this
    correctly has historically been a difficult programming problem. If we forget, we'll waste
    memory. If we do it too early, we'll have an invalid variable. If we do it twice, that's a bug
    too. We need to pair exactly one "allocate" with exactly one "free".

    Rust takes a different path: the memory is automatically returned once the variable that owns it
    goes out of scope. Here's a version of our scope example using a String instead of a string
    literal:
     */

    {
        let s = String::from("hello"); // s is valid from this point forward
        // do stuff with s
    }                                  // this scope is now over, and s is no longer valid

    /*
    There is a natural point at which we can return the memory our String needs to the allocator:
    when "s" goes out of scope. When a variable goes out of scope, Rust calls a special function for
    us. This function is called "drop", and it's where the author of "String" can put the code to
    return the memory. RUst calls "drop" automatically at the closing curly bracket.

    This pattern has a profound impact on the way Rust code is written. It may seem simple right
    now, but the behavior of code can be unexpected in more complicated situations when we want to
    have multiple variables use the data we've allocated on the heap. Let's explore some of those
    situations now.
     */
}

fn variables_data_move() {
    /*
    Multiple variables can interact with the same data in different ways in Rust. Let's look at an
    example using an integer:
     */

    let x = 5;
    let y = x;

    /*
    We can probably guess what this is doing: "bind the value 5 to x; then make a copy of the value
    in x and bind it to y." We now have two variables, x and y, and both equal 5. This is indeed
    what is happening, because integers are simple values with a known, fixed size, and these two 5
    values are pushed onto the stack.

    Now let's take a look at the String version:
     */

    let s1 = String::from("hello");
    let s2 = s1;

    /*
    This looks very similar, so we might assume that the way it works would be the sambe: that is,
    the second line would make a copy of the value in s1 and bind it to s2. But this isn't quite
    what happens.

    The following ASCII art represents what is happening to String under the covers. A String is
    made up of three parts: a pointer to the memory that holds the contents of the string, a length,
    and a capacity. This group of data is stored on the stack.

    --------------------                -----------------
    | name     | value |                | index | value |
    | ptr      |       | -------------> | 0     | h     |
    | len      | 5     |                | 1     | e     |
    | capacity | 5     |                | 2     | l     |
    --------------------                | 3     | l     |
                                        | 4     | o     |
                                        -----------------

    The length is how much memory, in bytes, the contents of the String are currently using. The
    capacity is the total amount of memory, in bytes, that the String has received from the
    allocator. The difference between length and capacity matters, but not in this context, so for
    now, it's fine to ignore the capacity.

    When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length, and
    the capacity that are on the stack. We do not copy the DATA on the HEAP that the pointer refers
    to. In other words, the data representation in memory looks like:

    --------------------
    | name     | value |
    | ptr      |       | ---+
    | len      | 5     |    |
    | capacity | 5     |    |          -----------------
    --------------------    |          | index | value |
                            +--------> | 0     | h     |
    --------------------    |          | 1     | e     |
    | name     | value |    |          | 2     | l     |
    | ptr      |       | ---+          | 3     | l     |
    | len      | 5     |               | 4     | o     |
    | capacity | 5     |               -----------------
    --------------------

    Earlier we said that when a variable goes out of scope, Rust automatically calls the "drop"
    function and cleans up the heap memory for that variable. But the previous representation shows
    both "data" pointers pointing to the same location. This is a problem: when s2 and s1 go out of
    scope, they will both try to free the same memory. This is known as a "double free error" and is
    onde of the memory safety bugs we mentioned previously. Freeing memory twice can lead to memory
    corruption, which can potentially lead to security vulnerabilities.

    To ensure memory safety, after the line "let s2 = s1;", Rust considers s1 as no longer valid.
    Therefore, Rust doesn't need to free anything when s1 goes out of scope. Check out what happens
    when you try to use s1 after s2 is created; it won't work:
     */

    /*
    The following line of code will raise error message "error[E0382]: borrow of moved value: `s1`"
    due to the use of an invalid reference:

    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s1);
    */

    /*
    If you've heard the terms "shallow copy" and "deep copy" while working with other languages, the
    concept of copying the pointer, length, and capacity without copying the data probably sounds
    like making a shallow copy. But because Rust also invalidates the first variable, instead of
    being called a shallow copy, it's known as a "move". In this example, we would say that s1 was
    "moved" into s2.

    That solves out problem! With only s2 valid, when it goes out of scope it alone will free the
    memory, and we're done.

    In addition, there's a design choice that's implied by this: Rust will never automatically
    create "deep" copies of you data. Therefore, any automatic copying can be assumed to be
    inexpensive in terms of runtime performance.
     */
}

fn variables_data_clone() {
    /*
    If we DO want to deeply copy the heap data of the String, not just the stack data, we can use a
    common method called "clone". We'll discuss method syntax in Chapter 5, but because methods are
    a common feature in many programming languages, you've probably seen them before.

    Here's an example of the "clone" method in action:
     */

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}\ts2 = {}", s1, s2); // prints "hello hello"

    /*
    This works just fine and explicitly produces the behavior where the heap data DOES get copied.

    When you see a call to "clone", you know that some arbitrary code is being executed and that
    code may be expensive. It's a visual indicator that something different is going on.
     */
}

fn stack_only_data_copy() {
    /*
    There's another wrinkle we haven't talked about yet. This code using integers works and is
    valid:
     */

    let x = 5;
    let y = x;
    println!("x ={x}\ty = {y}"); // prints 5 5

    /*
    But this code seems to contradict what we just learned: we don't have a call to "clone", but x
    is still valid and wasn't moved into y.

    The reason is that types such as integers that have a known size at compile time are stored
    entirely on the stack, so copies of the actual values are quick to make. That means there's no
    reason we would want to prevent x from being valid after we create the variable y. In other
    words, there's no difference between deep and shallow copying here, so calling "clone" wouldn't
    do anything different from the usual shallow copying, and we can leave it out.

    Rust has a special annotation called the "Copy" trait that we can place on types that are stored
    on the stack, as integers are (we'll talk more about traits in Chapter 10). If a type implements
    the "Copy" trait, variables that use it do not move, but rather are trivially copied, making
    them still valid after assignment to another variable.

    Rust won't let us annotate a type with "Copy" if the type, or any of its parts, has implemented
    the "Drop" trait. If the type needs something special to happen when the value goes out of scope
    and we add the "Copy" annotation to that type, we'll get a compile-time error.

    So, what types implement the "Copy" trait? You can check the documentation for the given type to
    be sure, but as a general rule, any group of simple scalar values can implement "Copy", and
    nothing that requires allocation or is some form of resource can implement "Copy". Here are
    some of the types that implement "Copy":

    * All the integer types, such as u32
    * All the floating-point types, such as f64
    * The Boolean type, bool, with values true and false
    * The character type, char
    * Tuples, if they only contain types that also implement "Copy". For example, (i32, i32
        implements "Copy", but (i32, String) does not
     */
}

// *************************************************************************************************
/*
The mechanics of passing a value to a function are similar to those when assigning a value to a
variable. Passing a variable to a function will move or copy, just as assignment does. Here are
some examples with some annotation showing where variables go to into and out of scope.
*/
fn ownership_and_functions() {
    let s = String::from("Ablubleblé"); // s comes into scope
    takes_ownership(s);      // s's value moves into the function...
    //... and so is no longer valid here

    let x: i32 = 5;                     // x comes into scope
    makes_copy(x);           // x would move into the function, but i32 is Copy, so it's
    // okay to still use x afteraward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens

fn takes_ownership(some_string: String) { // some_stirng comes into scope
    println!("some_string = {}", some_string);
} // Here, some_string goes out of scope and "drop" is called. The backing memory is freed

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("some_integer = {}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens

/*
If we tried to use "s" after the call to "takes_ownership", Rust would throw a compile-time error.
These static checks protect us from mistakes.
 */

// *************************************************************************************************

/*
Returning values can also transfer ownership. The following code shows an example of a function
that returns some value.
*/
fn return_values_and_scope() {
    let s1 = gives_ownership();                // gives_ownership moves its return value to s1

    let s2 = String::from("hello");            // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also
                                               // moves its return value into s3

} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens. s1 goes out of
  // scope and is dropped

fn gives_ownership() -> String {                // gives_ownership will move its return value into
                                                // the function that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string  // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String { // a string comes into scope

    a_string // a_string is returned and moves out to the calling function
}

/*
The ownership of a variable follows the same pattern every time: assigning a value to another
variable moves it. When a variable that includes data on the heap goes out of scope, the value will
be cleaned up by "drop", unless ownership of the data has been moved to another variable.

While this works, taking ownership and then returning ownership with every 3.3 Functions is a bit
tedious. What if we want to let a function use a value but not take ownership? It's quite annoying
that anything we pass in also needs to be passed back if we want to use it again, in addition to any
data resulting from the body of the function that we might want to return as well.

Rust does let us return multiple values using a tuple:
 */
/*
fn fun() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}", s2, len);
    // println!("{}", s1); // this line of code would raise error message "error[E0382]: borrow of
    // moved value: `s1`"
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}
*/
// *************************************************************************************************

fn main() {

}
