/*
Methods are similar to functions: we declare them with the "fn" keyword and a name, they can have
parameters and a return value, and they contain some code that's run when the method is called from
somewhere else. Unlike function, methods are defined within the context of a struct (or an enum or a
trait object, which we cover in Chapter 6 and Chapter 17, respectively), and their first parameter
is ALWAYS "self", which represents the instance of the struct the method is being called on.

Let's change the "area" function that has a Rectangle instance as a parameter and instead make an
area method defined on the Rectangle struct, as shown in the following code:
 */

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

fn main() {
    let rect1 = Rectangle {
        height: 30,
        width: 30
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

/*
To define a function within the context of Rectangle, we start an "impl" (implementation) block for
Rectangle. Everything within this impl block will be associated with the Rectangle type. Then we
move the area function within the impl curly brackets and change the first (and in this case, only)
parameter to be "self" in the signature and everywhere within the body. In main, where we called the
area function and passed rect1 as an argument, we can instead use method syntax to call the area
method on our Rectangle instance. The method syntax goes after an instance: we add a dot followed by
the method name, parentheses, and any arguments.

In the signature for area, we use "&self" instead of "rectangle: &Rectangle". The "&self" is
actually short for "self: &Self". Within and impl block, the type "Self" is an alias for the type
that the impl block is for. Methods myst have a parameter named self in the first parameter spot.
Note that we still need to use the "&" in front of the self shorthand to indicate that this method
borrows the "Self" instance, just as we did in "rectangle: &Rectangle". Methods can take ownership
of self, borrow self immutably, as we've done here, or borrow self mutably, just as they can any
other parameter.

We chose &self here for the same reason we used &Rectangle in the function version: we don't want to
take ownership, and we just want to read the data in the struct, not write to it. If we wanted to
change the instance that we've called the method on as part of what the method does, we'd use
"&mut self" as the first parameter. Having a method that takes ownership of the instance by using
just "self" as the first parameter is rare; this technique is usually used when the method
transforms self into something else and you want to prevent the caller from using the original
instance after the transformation.

The main reason for using methods instead of functions, in addition to providing method syntax and
not having to repeat the type of self in every methods signature, is for organization. We've put all
the things we can do with an instance of a type in one impl block rather than making future users of
our code search for capabilities of Rectangle in various places in the library we provided.

Note that we can choose to gi
 */