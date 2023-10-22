/*
The issue with the tuple code in the previous example is that we have to return the String to the
calling function so we can still use the String after the call to calculate_length, because the
String was moved into calculate_length. Instead, we can provide a reference to the String value.
A reference is like a pointer in that it's an address we can follow to access the data stored at
that address; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed
to point to a valid value of a particular type for the life of that reference.

Here is how you would define and use a calculate_length function that has a reference to an object
as a parameter instead of taking ownership of the value:
 */

fn passing_references() {
    let s1 = String::from("hello");
    let len = calculate_lenght(&s1);
    // println!("The length of '{}' is {}.", s1, len);
}

fn calculate_lenght(s: &String) -> usize {
    s.len()
}

/*
First, notice that all the tuple code in the the variable declaration and the function return value
is gone. Second, note that we pass "&s1" into calculate_length and, in its definition, we take
"&String" rather than "String". These ampersands represent references, and they allow you to refer
to some value without taking ownership of it.

----------------        --------------------        -----------------
| name | value |        | name     | value |        | index | value |
|--------------|        |-------------------        -----------------
| prt  |       | -----> | ptr      |       | -----> |   0   |   h   |
----------------        | len      |   5   |        |   1   |   e   |
                        | capacity |   5   |        |   2   |   l   |
                        --------------------        |   3   |   l   |
                                                    |   4   |   o   |
                                                    -----------------
** Note: The opposite of referencing by using "&" is dereferencing, which is accomplished with the
dereference operator, "*". We'll see some uses of the dereference operator in Chapter 8 and discuss
details of dereferencing in Chapter 15.

Let's take a closer look at the function call here:

let s1 = String::from("hello");
let len = calculate_lenght(&s1);

The "&s1" syntax lets us create a reference that refers to the value of s1 but does not own it.
Because it does not own it, the value it points to will not be dropped when the reference stops
being used.

Likewise, the signature of the function uses "&" to indicate that the type of the parameter "s" is a
reference. Let's add some explanatory annotations:

fn calculate_lenght(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, it is
  // not dropped

The scope in which the variable "s" is valid is the same as any function parameter's scope, but the
value pointed to by the reference is not dropped when "s" stops being used, because "s" doesn't have
ownership. When functions have references as parameters instead of the actual values, we won't need
to return the values in order to give back ownership, because we never had ownership.

We call the action of creating a reference "borrowing". As in real life, if a person owns something,
you can borrow it from them. When you're done, you have to give it back. You don't own it.

So, what happens if we try to modify something we're borrowing?

The following code raises error message "error[E0596]: cannot borrow `*some_string` as mutable, as
it is behind a `&` reference", since it tries to alter the value of the String passed as a
reference:

fn change(some_string: &String) {
    some_string.push_str("text");
}

Just as variables are immutable by default, so are references. We're not allowed to modify something
we have a reference to.

We can fix the previous code, allowing us to modify a borrowed value with just a few small tweaks
that use, instead, a mutable reference:
*/

fn f() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/*
First we change "s" to be mutable. Then we create a mutable reference with "&mut s" where we call
the "change" function, and update the function signature to accept a mutable reference with
"some_string: &mut String". This makes it very clear that the "change" function will mutate the
value it borrows.

Mutable references have one big restriction: if you have a mutable reference to a value, you can
have no other references to that values. This code that attempts to create two mutable references
to "s" will fail:

let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s;
println!("{}, {}", r1, r2);

This code raises error message "error[E0499]: cannot borrow `s` as mutable more than once at a
time".

This error says that this code is invalid because we cannot borrow "s" as a mutable more than once
at a time. The first mutable borrow is in "r1" and must last until it's used in the println!, but
between the creation of that mutable reference and its usage, we tried to create another mutable
reference in "r2" that borrows the same data as "r1".

The restriction preventing multiple mutable references to the same data at the same time allows for
mutation but in a very controlled fashion. It's something that new Rustaceans struggle with because
most languages let you mutate whenever you'd like. The benefit of having this restriction is that
Rust can prevent data races at compile time. A data race is similar to a race condition and happens
when these three behaviors occur:

1. Two or more pointers access the same data at the same time
2. At least one of the pointes is being used to write the data
3. There's no mechanism being used to synchronize access to the data

Data races cause undefined behavior and can be difficult to diagnose and fix when you're trying to
track them down at runtime; Rust prevents this problem by refusing to compile code with data races!

As always, we can use curly brackets to create a new scope, allowing for multiple mutable
references, just not SIMULTANEOUS ones:
*/

fn references_on_scope() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems
    let r2 = &mut s;
}

/*
Rust enforces a similar rule for combining mutable and immutable references. The following code
raises error message "error[E0502]: cannot borrow `s` as mutable because it is also borrowed as
immutable":

let mut s = String::from("hello");
let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG problem

We also cannot have a mutable reference while we have an immutable one to the same value.

Users of an immutable reference don't expect the value to suddenly change out from under them!
However, multiple immutable references are allowed because no one who is just reading the data has
the ability to affect anyone else's reading of the data.

Note that a reference's scope starts from where it is introduced and continues through the last time
that reference is used. For instance, the following code will compile because the last usage of the
immutable references, the "println!", occurs before the mutable reference is introduced:
*/

fn multiple_references() {
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2); // variables r1 and r2 will not be used after this point
    let r3 = &mut s; // no problem
    // println!("{}", r1); // this line of code would raise error message "error[E0502]: cannot
    // borrow `s` as mutable because it is also borrowed as immutable"
}

/*
The scopes of the immutable references "r1" and "r2" end after the "println!" where they are last
used, which is before the mutable reference "r3" is created. These scopes don't overlap, so this
code is allowed: the compiler can tell that the reference is no longer being used at a point before
the end of the scope.

Even though borrowing errors may be frustrating at times, remember that it's the Rust compiler
pointing out a potential bug early (at compile time rather than at runtime) and showing you exactly
where the problem is. Then you don't have to track down why your data isn't what you thought it was.
*/

fn multiple_immutable_references() {
    let s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &s; // no problem
}

/*
In languages with pointers, it's easy to erroneously create a "dangling pointer" - a pointer that
references a location in memory that may have been given to someone else - by freeing some memory
while preserving a pointer to that memory. In Rust, by contrast, the compiler guarantees that
references will never be dangling references: if you have a reference to some data, the compiler
will ensure that the data will not go out of scope before the reference to the data does.

Let's try to create a dangling reference to see how Rust prevents them with the compile-time error:

fn x() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello")
    &s
}

The previous code raises error message "error[E0106]: missing lifetime specifier".

This error message refers to a feature we haven't covered yet: lifetimes. We'll discuss lifetimes in
detail in Chapter 10. But, if you disregard the parts about lifetimes, the message does contain the
key to why this code is a problem:

"help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime"

Let's take a closer look at exactly what's happening at each stage of our "dangle" code:

fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello");

    &s // we return a reference to the String "s"

} // Here, "s" goes out of scope, and is dropped. Its memory goes away. DANGER!

Because "s" is created inside "dangle", when the code of "dangle" is finished, "s" will be
deallocated. But we tried to return a reference to it. That means this reference would be pointing
to an invalid String. That's no good! Rust won't let us do this.

The solution here is to return the String directly:

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

This works without any problems. Ownership is moved out, and nothing is deallocated.
*/

/*
The Rules of References

Let's recap what we've discussed about references:

* At any given time, you can have EITHER one mutable reference OR any number of immutable references
* References must always be valid

 */

fn main() {
    passing_references();
}
