/*
To understand when we might want to use structs, let's write a program that calculates the area of
a rectangle. We'll start by using single variables, and then refactor the program until we're using
structs instead.

We use structs to add meaning by labeling the data. We can transform the tuple we're using into a
struct with a name for the whole as well as names for the parts, as shown in the following example;
 */

/*
struct Rectangle {
    height: u32,
    width: u32,
}

fn calculate_area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

fn main() {
    let rectangle: Rectangle = Rectangle {
        height: 10,
        width: 35,
    };
    println!("Area of the rectangle: {}", calculate_area(&rectangle));
}

Here we've defined a struct and named it Rectangle. Inside the curly brackets, we defined the fields
as width and height, both of which have the type u32. Then, in main, we created a particular
instance of Rectangle that has a width of 30 and a height of 50.

Our area function is now defined with one parameter, which we've named rectangle, whose type is an
immutable borrow of a struct Rectangle instance. As mentioned in Chapter 4, we want to borrow the
struct rather than take ownership of it. This way, main retains its ownership and can continue using
rect1, which is the reason we use the "&" in the function signature and where we call the function.

The calculate_area function accesses the width and height fields of the Rectangle instance (note
that accessing fields of a borrowed struct instance does ot move the field values, which is why you
ofter see borrows of structs). Our function signature for area now says exactly what we mean:
calculate the area of Rectangle, using its width and height fields. This conveys that the width and
height are related to each other, and it gives descriptive names to the values rather than using the
tuple index values of 0 and 1. This is a win for clarity.
 */

/*
It'd be useful to be able to print an instance of Rectangle while we're debugging our program and
see the values for all its fields. The following code tries using the println! macro as we have used
in previous chapters. This won't work, however.

struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {}", rect1);
}

When we compile this code, we get the following error message: "error[E0277]: `Rectangle` doesn't
implement `std::fmt::Display`"

The println! macro can do many kinds of formatting, and by default, the curly brackets tell println!
to use formatting known as "Display": output intended for direct end user consumption. The primitive
types we've seen so far implement Display by default because there's only one way you'd want to show
a 1 or any other primitive type to a user. But with structs, the way println! should format the
output is less clear because there are more display possibilities: Do you want commas or not? Do you
want to print the curly brackets? Should all the fields be shown? Due to this ambiguity, Rust
doesn't try to guess what we want, and structs don't have a provided implementation of Display to
use with println! and the {} placeholder.

If we continue reading the errors, we'll find this helpful note:
"
   = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
"

Let's try it! The println! macro will now look like "println!("rect1 is {:?}, rect1");". Putting the
specifier ":?" inside the curly brackets tells println! we want to use an output format called
"Debug". The Debug trait enables us to print our struct in a way that is useful for developers so
we can see its value while we're debugging our code.

If we compile the code with this change, we still get an error:
"error[E0277]: `Rectangle` doesn't implement `Debug`"

But again, the compiler gives us a helpful note:
"
   = help: the trait `Debug` is not implemented for `Rectangle`
   = note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
"

Rust does include functionality to print out debugging information, but we have to explicitly opt in
to make that functionality available for our struct. To do that, we add the outer attribute
"#[derive(Debug)] just before the struct definition:
}


#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

fn main() {
    let rect1 = Rectangle {
        height: 50,
        width: 30
    };
    println!("rect1 is {:?}", rect1);
}


It's not the prettiest output, but it shows the values of all the fields for this instance, which
would definitely help during debugging. When we have larger structs, it's useful to have output
that's a bit easier to read; in those cases, we can use "{:#?}" instead of "{:?}" in the println!
string. In this example, using the "{:#?}" style will output the following:


#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

fn main() {
    let rect1 = Rectangle {
        height: 50,
        width: 30
    };
    println!("rect1 is {:#?}", rect1);
}


Another way to print out a value using a the Debug format is to use the "dbg!" macro, which takes
ownership of an expression (as opposed to println!, which takes a reference), prints the file and
line number of where that "dbg!" macro call occurs in your code along with the resultant value of
that expression, and returns ownership of the value.

NOTE: Calling the dbg! macro prints to the standard error console stream (stderr), as opposed to
println!, which prints to the standard output console stream (stdout). We'll talk more about stderr
and stdout in the "Writing Error Messages to Standard Error Instead of Standard Output" section in
Chapter 12.

Here's an example where we're interested in the value that gets assigned to the width field, as well
as the value of the whole struct in rect1:
 */

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        height: dbg!(30 * scale),
        width: 30
    };

    dbg!(&rect1);
}

/*
We can put dbg! around the expression "30 * scale" and, because dbg! returns ownership of the
expression's value, the width field will get the same value as if we didn't have the dbg! call
there. We don't want dbg! to take ownership of rect1, so we use a reference to rect1 in the next
call. Here's what the output of this example looks like:


     Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/example_program_using_structs`
[src/main.rs:161] 30 * scale = 60
[src/main.rs:165] &rect1 = Rectangle {
    height: 60,
    width: 30,
}


We can see the first bit of output came from "src/main.rs" line 161 where we're debugging the
expression "30 * scale", and its resultant value is 60 (the Debug formatting implemented for
integers is to print only their value). The dbg! call on line 165 of src/main.rs outputs the value
of &rect1, which is the Rectangle struct. This output uses the pretty Debug formatting of the
Rectangle type. The dbg! macro can be really helpful when you're trying to figure out what your
code is doing!

In addition to the Debug trait, Rust has provided a number of traits for us to use with the "derive"
attribute that can add useful behavior to our custom types. We'll cover how to implement these
traits with custom behavior as well as how to create your own traits in Chapter 10.

Our "calculate_area" function s very specific: it only computes the area of rectangles. It would be
very helpful to tie this behavior more closely to our Rectangle struct, because it won't work with
any other type. Let's look at how we can continue to refactor this code by turning the
"calculate_area" function into an "calculate_area" method defined on our Rectangle type.
 */