/*
The first collection type we’ll look at is Vec<T>, also known as a vector. Vectors allow you to
store more than one value in a single data structure that puts all the values next to each other in
memory. Vectors can only store values of the same type. They are useful when you have a list of
items, such as the lines of text in a file or the prices of items in a shopping cart.

Creating a New Vector

To create a new empty vector, we call the Vec::new function, as shown in Listing 8-1.
 */

fn creating_a_vector() {
    let v: Vec<i32> = Vec::new();
}

/*
Note that we added a type annotation here. Because we aren’t inserting any values into this vector,
Rust doesn’t know what kind of elements we intend to store. This is an important point. Vectors are
implemented using generics; we’ll cover how to use generics with your own types in Chapter 10. For
now, know that the Vec<T> type provided by the standard library can hold any type. When we create a
vector to hold a specific type, we can specify the type within angle brackets. In Listing 8-1, we’ve
told Rust that the Vec<T> in v will hold elements of the i32 type.

More often, you’ll create a Vec<T> with initial values and Rust will infer the type of value you
want to store, so you rarely need to do this type annotation. Rust conveniently provides the vec!
macro, which will create a new vector that holds the values you give it. Listing 8-2 creates a new
Vec<i32> that holds the values 1, 2, and 3. The integer type is i32 because that’s the default
integer type, as we discussed in the “Data Types” section of Chapter 3.
 */

fn vec_macro() {
    let v = vec![1, 2, 3];
}

/*
Because we’ve given initial i32 values, Rust can infer that the type of v is Vec<i32>, and the type
annotation isn’t necessary. Next, we’ll look at how to modify a vector.


Updating a Vector

To create a vector and then add elements to it, we can use the push method, as shown in Listing 8-3.
 */

fn pushing_values_to_vector() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v.push(9);
}

/*
As with any variable, if we want to be able to change its value, we need to make it mutable using
the mut keyword, as discussed in Chapter 3. The numbers we place inside are all of type i32, and
Rust infers this from the data, so we don’t need the Vec<i32> annotation.
Reading Elements of Vectors

There are two ways to reference a value stored in a vector: via indexing or using the get method. In
the following examples, we’ve annotated the types of the values that are returned from these
3.3 Functions for extra clarity.

Listing 8-4 shows both methods of accessing a value in a vector, with indexing syntax and the get
method.
 */

fn accessing_values_from_vector() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element.")
    }
}

fn iterating_values_in_a_vector() {
    let mut v = vec![100, 32, 57];

    for i in &v {
        println!("{i}");
    }

    println!();

    for i in &mut v {
        *i += 50;
    }

    /*
    To change the value that the mutable reference refers to, we have to use the * dereference
    operator to get to the value in i before we can use the += operator. We’ll talk more about
    the dereference operator in the “Following the Pointer to the Value with the Dereference
    Operator” section of Chapter 15.

    Iterating over a vector, whether immutably or mutably, is safe because of the borrow checker's
    rules.
     */

    for i in &v {
        println!("{i}");
    }
}

fn using_an_enum_to_store_multiple_types() {
    /*
    Vectors can only store values that are the same type. This can be inconvenient; there are
    definitely use cases for needing to store a list of items of different types. Fortunately, the
    variants of an enum are defined under the same enum type, so when we need one type to represent
    elements of different types, we can define and use an enum!

    For example, say we want to get values from a row in a spreadsheet in which some of the columns
    in the row contain integers, some floating-point numbers, and some strings. We can define an
    enum whose variants will hold the different value types, and all the enum variants will be
    considered the same type: that of the enum. Then we can create a vector to hold that enum and
    so, ultimately, holds different types.
     */

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    /*
    Rust needs to know what types will be in the vector at compile time so it knows exactly how much
    memory on the heap will be needed to store each element. We must also be explicit about what
    types are allowed in this vector. If Rust allowed a vector to hold any type, there would be a
    chance that one or more of the types would cause errors with the operations performed on the
    elements of the vector. Using an enum plus a match expression means that Rust will ensure at
    compile time that every possible case is handled, as discussed in Chapter 6.

    If you don’t know the exhaustive set of types a program will get at runtime to store in a
    vector, the enum technique won’t work. Instead, you can use a trait object, which we’ll cover in
    Chapter 17.
     */
}

fn dropping_a_vector_drops_its_elements() {
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
    /*
    When the vector gets dropped, all of its contents are also dropped, meaning the integers it
    holds will be cleaned up. The borrow checker ensures that any references to contents of a vector
    are only used while the vector itself is valid.
     */
}

fn main() {
   using_an_enum_to_store_multiple_types();
}
