/*
Structs are similar to tuples, discussed in "The Tuple Type" section, in that both hold multiple
related values. Like tuples, the pieces of a struct can be different types. Unlike with tuples, in a
struct you'll name each piece of data so it's clear what the values mean. Adding these names means
that structs are more flexible than tuples: you don't have to rely on the order of the data to
specify or access the values of an instance.

To define a struct, we enter the keyword "struct" and name the entire struct. A struct's name should
describe the significance of the pieces of data being grouped together. Then, inside curly brackets,
we define the names and types of the pieces of data, which we call fields. For example, the
following code shows a struct that stores information about a user account:
 */

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

/*
To use a struct after we've defined it, we create an "instance" of that struct by specifying
concrete values for each of the fields. We create an instance by stating the name of the struct and
then add curly brackets containing "key: value" pairs, where the keys are the names of the fields
and the values are the data we want to store in those fields. We don't have to specify the fields
in the same order in which we declared them in the struct. In other words, the struct definition is
like a general template for the type, and instances fill in that template with particular data to
create values of the type. For example, we can declare a particular user as shown in the following
code:
 */

/*
fn declaring_user() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1
    };
}
*/

/*
To get a specific value from a struct, we use dot notation. For example, to access this user's email
address, we use "user1.email". If the instance is mutable, we can change the value by using the dot
notation and assigning into a particular field. The following code shows how to change the value in
the "email" field of a mutable "User" instance:
 */

/*
fn altering_user_value() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1
    };

    user1.email = String::from("anotheremail@example.com");
}
*/

/*
Note that the entire instance must be mutable; Rust doesn't allow us to mark only certain fields as
mutable. As with any expression, we can construct a new instance of the struct as the last
expression in the function body to implicitly return that new instance.

The following code shows a "build_user" function that returns a User instance with the given email
and username. The "active" field gets the value of "true", and the sign_in_count gets a value of 1.
 */

/*
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1
    }
}
*/

/*
It makes sense to name the function parameters with the same name as the struct fields, but having
to repeat the email and username field names and variables is a bit tedious. If the struct had more
fields, repeating each name would get even more annoying. Luckily, there's a convenient shorthand!

Because the parameter names and the struct field names are exactly the same in the previous example,
we can use the "field init shorthand" syntax to rewrite "build_user" so it behaves exactly the same
but doesn't have the repetition of "username" and "email", as shown in the following example:
 */

/*
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}
*/

/*
Here, we're creating a new instance of the "User" struct, which has a field named "email". We want
to se the "email" field's value to the value in the "email" parameter of the "build_user" function.
Because the "email" field and the "email" parameter have the same name, we only need to write
"email" rather than "email: email".

It's often useful to create a new instance of a struct that includes most of the values from another
instance, but changes some. You can do this using "struct update syntax".

First, in the following code we show how to create a new "User" instance in "user2" regularly,
without the update syntax. We set a new value for "email" but otherwise use the same values from
"user1" that we created previously:
 */

fn replicating_values() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1
    };

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count
    };
}

/*
Using struct update syntax, we can achieve the same effect with less code, as show in the following
example. The syntax ".." specified that the remaining fields not explicitly set should have the same
value as the fields in the given instance.
 */

fn struct_update_syntax() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}

/*
The previous code also creates an instance in "user2" that has a different value for "email" but has
the same values for the "username", "active", and "sign_in_count" fields from user1. THe "..user1"
must come last to specify that any remaining fields should get their values from the corresponding
fields in "user1", but we can choose to specify values for as many fields as we want in any order,
regardless of the order of the fields in the struct's definition.

Note that the struct update syntax uses "=" like an assignment; this is because it moves the data,
just as we saw in the "Variables and Data Interacting with Move" section. In this example, we can no
longer use "user1" as a whole after creating "user2" because the String in the "username" field of
"user1" was moved into "user2". If we had giver user2 new String values for both "email" and
"username", and thus only used the "active" and "sign_in_count" values from "user1", then "user1"
would still be valid after creating "user2". Both "active" and "sign_in_count" are types that
implement the "Copy" trait, so the behavior we discussed in the "Stack-Only Data: Copy" section
would apply.

Rust also supports structs that look similar to tuples, called "tuple structs". Tuple structs have
the added meaning the struct name provides but don't have names associated with their fields;
rather, they just have the types of the fields. Tuple structs are useful when you want to give the
whole tuple a name and make the tuple a different type from other tuples, and when naming each field
as in a regular struct would be verbose or redundant.

To define a tuple struct, start with the "struct" keyword and the struct name followed by the types
in the tuple. For example, here we define and use two tuple structs named "Color" and "Point":
 */

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn using_tuple_structs() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

/*
Note that the "black" and "origin" values are different types because they're instances of different
tuple structs. Each struct you define is its own type, even though the fields within the struct
might have the same types. For example, a function that takes a parameter of type "Color" cannot
take a "Point" as an argument, even though both types are made up of three "i32" values. Otherwise,
tuple struct instances are similar to tuples in that you can destructure them into their individual
pieces, and you can use a "." followed by the index to access an individual value.

You can also define structs that don't have any fields! These are called unit-like structs because
they behave similar to "()", the unit type that we mentioned in "The Tuple Type" section. Unit-like
structs can be useful when you need to implement a trait on some type but don't have any data that
you want to store in the type itself. We'll discuss traits in Chapter 10. Here's an example of
declaring and instantiating a unit strut named "AlwaysEqual":
 */

struct AlwaysEqual;

fn using_unit_like_structs() {
    let subject = AlwaysEqual;
}

/*
To define "AlwaysEqual", we use the "struct" keyword, the name we want, and then a semicolon. No
need for curly brackets or parentheses! Then we can get an instance of "AlwaysEqual" in the
"subject" variable in a similar way: using the name we defined, without any curly brackets or
parentheses. Imagine that later we'll implement behavior for this type such that every instance of
"AlwaysEqual" is always equal to every instance of any other type, perhaps to have a known result
for testing purposes. We wouldn't need any data to implement that behavior! You'll see in Chapter 10
how to define traits and implement them on any type, including unit-like structs.
 */

/*
Ownership of Struct Data

In the USer struct definition, we used the owned String type rather than the &str string slice type.
This is a deliberate choice because we want each instance of this struct to own all of its data and
fora that data to be valid for as long as the entire struct is valid.

It's also possible for structs to store references to data owned by something else, but to do so
requires the use of lifetimes, a Rust feature that we'll discuss in Chapter 10. Lifetimes ensure
that the data referenced by a struct is valid for as long as the struct is. Let's say you try to
store a reference in a struct without specifying lifetimes, like the following: this won't work:

struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64
}

fn main() {
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1
    }
}

The compiler will complain that it needs lifetime specifiers: "error[E0106]: missing lifetime
specifier"

In Chapter 10, we'll discuss how to fix these error so you can store references in structs, but for
now, we'll fix errors like these using owned types like String instead of references like &str
 */

fn main() {
    println!("Hello, world!");
}
